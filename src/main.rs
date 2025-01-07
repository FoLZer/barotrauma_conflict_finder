pub mod content_file;
pub mod content_package;
pub mod loading;
pub mod logger;
pub mod player_config;
pub mod shared;

use std::{path::PathBuf, sync::Arc};

use clap::Parser;
use iced::{
    Element, Subscription, Task,
    futures::{SinkExt, Stream, StreamExt, channel::mpsc::UnboundedReceiver, lock::Mutex},
    stream,
    widget::{
        Column, button, column, container, pick_list, radio, row, scrollable, text, text_editor,
        text_input,
    },
};
use loading::{ConflictType, LoadingState};
use log::LevelFilter;
use logger::SimpleLogger;
use strum::IntoEnumIterator;

#[derive(Parser)]
struct Args {
    #[arg(default_value = r#"C:\Program Files (x86)\Steam\steamapps\common\Barotrauma"#)]
    game_path: String,
    config_player_path: Option<String>,
}

impl Args {
    pub fn config_player_path(&self) -> PathBuf {
        self.config_player_path
            .as_ref()
            .map(|v| PathBuf::from(v.clone()))
            .unwrap_or_else(|| PathBuf::from(self.game_path.clone()).join("config_player.xml"))
    }
}

impl Default for Args {
    fn default() -> Self {
        Args::parse()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ScreenChanged(Screen),
    GamePathChanged(String),
    ConfigPathChanged(String),
    LogMessage(String),
    LogScreenAction(text_editor::Action),
    StartParsing,
    LoadProgress(Result<loading::Progress, ()>),
    ConflictTypeSelected(ConflictType),
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Screen {
    #[default]
    Main,
    Settings,
    Logs,
    LoadingMods,

    ConflictSolver,
}

struct App {
    pub args: Args,
    pub screen: Screen,

    pub logs: text_editor::Content,

    pub logger_rx: Arc<Mutex<UnboundedReceiver<String>>>,

    pub loading_state: Option<LoadingState>,
    pub selected_conflict_type: ConflictType,
}

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        column![
            //Screen selection
            row![
                radio(
                    "Main",
                    Screen::Main,
                    Some(self.screen),
                    Message::ScreenChanged
                ),
                radio(
                    "Settings",
                    Screen::Settings,
                    Some(self.screen),
                    Message::ScreenChanged
                ),
                radio(
                    "Logs",
                    Screen::Logs,
                    Some(self.screen),
                    Message::ScreenChanged
                ),
            ]
            .push_maybe(
                self.loading_state
                    .as_ref()
                    .filter(|v| !matches!(v, LoadingState::Finished(_)))
                    .map(|_| radio(
                        "Loading",
                        Screen::LoadingMods,
                        Some(self.screen),
                        Message::ScreenChanged,
                    ))
            )
            .push_maybe(
                self.loading_state
                    .as_ref()
                    .filter(|v| matches!(v, LoadingState::Finished(_)))
                    .map(|_| radio(
                        "Conflict Solver",
                        Screen::ConflictSolver,
                        Some(self.screen),
                        Message::ScreenChanged,
                    ))
            ),
            //Main view
            container(match self.screen {
                Screen::Main => {
                    Into::<Element<'_, Message>>::into(column![
                        text!("Game Path: {}", &self.args.game_path),
                        button("Parse").on_press(Message::StartParsing)
                    ])
                }
                Screen::Settings => {
                    column![
                        row![
                            text!("Game Path:"),
                            text_input("", &self.args.game_path).on_input(Message::GamePathChanged)
                        ],
                        row![
                            text!("Config Path (Optional):"),
                            text_input("", self.args.config_player_path.as_ref().map_or("", |v| v))
                                .on_input(Message::ConfigPathChanged)
                        ]
                    ]
                    .into()
                }
                Screen::Logs => {
                    text_editor(&self.logs)
                        .on_action(Message::LogScreenAction)
                        .into()
                }
                Screen::LoadingMods => {
                    column![
                        match &self.loading_state {
                            Some(p) => match p {
                                LoadingState::Started => text!("Loading: Pre-initialization"),
                                LoadingState::ReadingModList => text!("Loading: Reading Mod List"),
                                LoadingState::LoadingCoreContent =>
                                    text!("Loading: Loading Core Content"),
                                LoadingState::LoadingMods { i, max } =>
                                    text!("Loading: Loading Mods: {} / {}", i, max),
                                LoadingState::LoadingConflicts =>
                                    text!("Loading: Loading Conflicts"),
                                LoadingState::Finished(_) => text!("Loading: Finished!"),
                            },
                            None => text!("Loading: Error: Not currently loading anything"),
                        },
                        text_editor(&self.logs).on_action(Message::LogScreenAction)
                    ]
                    .into()
                }
                Screen::ConflictSolver => {
                    let Some(LoadingState::Finished(conflicts)) = &self.loading_state else {
                        return text!("Error! No loaded mods!").into();
                    };
                    let selected_conflicts =
                        self.selected_conflict_type.get_conflict_by_type(conflicts);
                    row![container(column![
                        pick_list(
                            ConflictType::iter()
                                .filter(|t| !t.get_conflict_by_type(conflicts).is_empty())
                                .collect::<Vec<_>>(),
                            Some(self.selected_conflict_type),
                            Message::ConflictTypeSelected
                        ),
                        scrollable(Column::with_children(selected_conflicts.iter().map(
                            |(identifier, _)| {
                                Into::<Element<'_, Message>>::into(text!("{}", identifier))
                            }
                        )))
                    ])]
                    .into()
                }
            })
        ]
        .into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ScreenChanged(screen) => {
                self.screen = screen;
            }
            Message::GamePathChanged(s) => {
                self.args.game_path = s;
            }
            Message::ConfigPathChanged(s) => {
                self.args.config_player_path = if s.len() > 0 { Some(s) } else { None }
            }
            Message::LogMessage(s) => {
                self.logs
                    .perform(text_editor::Action::Move(text_editor::Motion::DocumentEnd));
                self.logs
                    .perform(text_editor::Action::Edit(text_editor::Edit::Paste(
                        Arc::new(format!("{}\n", s)),
                    )));
            }
            //Allow user to copy from log screen but not edit
            Message::LogScreenAction(action) => {
                if !action.is_edit() {
                    self.logs.perform(action);
                }
            }
            Message::StartParsing => {
                let game_path = PathBuf::from(&self.args.game_path);
                let config_player_path = self.args.config_player_path();

                self.loading_state = Some(LoadingState::Started);

                let task = Task::stream(loading::load(game_path, config_player_path));

                return Task::done(Message::ScreenChanged(Screen::LoadingMods))
                    .chain(task.map(|progress| Message::LoadProgress(progress)))
                    .chain(Task::done(Message::ScreenChanged(Screen::ConflictSolver)));
            }
            Message::LoadProgress(progress) => match progress {
                Ok(progress) => {
                    if self.loading_state.is_some() {
                        self.loading_state.replace(progress.into());
                    }
                }
                Err(_) => {
                    return Task::done(Message::ScreenChanged(Screen::Logs));
                }
            },
            Message::ConflictTypeSelected(t) => self.selected_conflict_type = t,
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        #[derive(Hash)]
        struct Sub;

        Subscription::run_with_id(Sub, log_worker(self.logger_rx.clone()))
    }
}

fn log_worker(rx: Arc<Mutex<UnboundedReceiver<String>>>) -> impl Stream<Item = Message> {
    stream::channel(5, |mut output| async move {
        let mut l = rx.lock().await;
        loop {
            let m = l.select_next_some().await;
            output.send(Message::LogMessage(m)).await.unwrap();
        }
    })
}

fn main() -> iced::Result {
    let logger = SimpleLogger::new()
        .with_module_level("wgpu_core", LevelFilter::Warn)
        .with_module_level("naga", LevelFilter::Warn)
        .with_module_level("wgpu_hal", LevelFilter::Warn)
        .with_module_level("cosmic_text", LevelFilter::Warn)
        .with_module_level("iced_graphics", LevelFilter::Warn)
        .with_module_level("iced_winit", LevelFilter::Warn)
        .with_module_level("iced_wgpu", LevelFilter::Warn);
    let logger_rx = logger.rx.clone();
    logger.init().unwrap();
    iced::application("Barotrauma Conflict Finder", App::update, App::view)
        .subscription(App::subscription)
        .run_with(|| {
            let state = App {
                args: Default::default(),
                screen: Default::default(),
                logs: Default::default(),
                logger_rx,
                loading_state: Default::default(),
                selected_conflict_type: ConflictType::Item,
            };
            (state, Task::none())
        })
}
