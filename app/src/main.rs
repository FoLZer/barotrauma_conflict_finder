/*
barotrauma_conflict_finder
Copyright (C) 2025  FoLZer

This program is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; version 2.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
*/

pub mod logger;
pub mod manifest;

use std::{path::PathBuf, sync::Arc};

use asset_parser::{
    content_package::{ContentPackage, Regular},
    loading::{ConflictType, LoadingState},
    shared::version::Version,
};
use clap::Parser;
use iced::{
    Element, Length, Subscription, Task,
    futures::{SinkExt, Stream, StreamExt, channel::mpsc::UnboundedReceiver, lock::Mutex},
    stream,
    widget::{
        Column, Row, Space, button, column, container, pick_list, radio, row, scrollable, text,
        text_editor, text_input,
    },
};
use log::LevelFilter;
use logger::SimpleLogger;
use manifest::{ModIdentifier, ModManifest};
use strum::IntoEnumIterator;

const CURRENT_GAME_VERSION: Version = Version {
    major: 1,
    minor: 7,
    build: Some(7),
    revision: Some(0),
};

#[derive(Parser)]
struct Args {
    #[arg(default_value = r#"C:\Program Files (x86)\Steam\steamapps\common\Barotrauma"#)]
    game_path: String,
    config_player_path: Option<String>,
    patch_mod_path: Option<String>,
}

impl Args {
    pub fn config_player_path(&self) -> PathBuf {
        self.config_player_path
            .as_ref()
            .map(|v| PathBuf::from(v.clone()))
            .unwrap_or_else(|| PathBuf::from(self.game_path.clone()).join("config_player.xml"))
    }

    pub fn patch_mod_path(&self) -> PathBuf {
        self.patch_mod_path
            .as_ref()
            .map(|v| PathBuf::from(v.clone()))
            .unwrap_or_else(|| {
                PathBuf::from(self.game_path.clone()).join(r#"LocalMods\conflict_finder_patchmod"#)
            })
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
    Conflict1EditorAction(text_editor::Action),
    Conflict2EditorAction(text_editor::Action),
    StartParsing,
    LoadProgress(Result<asset_parser::loading::Progress, ()>),
    ConflictTypeSelected(ConflictType),
    ConflictButtonPressed(usize),
    ConflictFileButtonPressed(usize),
    XMLHighlighterThemeSelected(iced::highlighter::Theme),
    PatchModLoaded(Arc<(ContentPackage<Regular>, ModManifest)>),
    LoadPatchMod,
    PatchModPathChanged(String),
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
    pub selected_conflict_index: Option<usize>,
    pub selected_conflict_file_index: Option<usize>,

    pub conflict1_text: text_editor::Content,
    pub conflict2_text: text_editor::Content,

    pub xml_highlight_theme: iced::highlighter::Theme,

    pub patch_mod: Option<Arc<(ContentPackage<Regular>, ModManifest)>>,
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
                    .filter(|v| !matches!(v, LoadingState::Finished(..)))
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
                    .filter(|v| matches!(v, LoadingState::Finished(..)))
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
                        ],
                        row![
                            text!("Patch Mod Path:"),
                            text_input("", &self.args.patch_mod_path().to_str().expect("encountered non UTF-8 text in patch mod path, these are not compatible!"))
                                .on_input(Message::PatchModPathChanged)
                        ],
                        text!("Patch Mod will be loaded once the mods are parsed"),
                        row![
                            text!("Xml Highlighter Theme:"),
                            pick_list(
                                iced::highlighter::Theme::ALL,
                                Some(self.xml_highlight_theme),
                                Message::XMLHighlighterThemeSelected
                            ),
                        ],
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
                                LoadingState::Finished(..) => text!("Loading: Finished!"),
                            },
                            None => text!("Loading: Error: Not currently loading anything"),
                        },
                        text_editor(&self.logs).on_action(Message::LogScreenAction)
                    ]
                    .into()
                }
                Screen::ConflictSolver => {
                    let Some(LoadingState::Finished(_, conflicts)) = &self.loading_state else {
                        return text!("Error! No loaded mods!").into();
                    };
                    let selected_conflicts =
                        self.selected_conflict_type.get_conflict_by_type(conflicts);
                    let mut sorted_conflicts = selected_conflicts.iter().collect::<Vec<_>>();
                    sorted_conflicts.sort_by(|a, b| a.0.cmp(b.0));

                    row![
                        container(column![
                            pick_list(
                                ConflictType::iter()
                                    .filter(|t| !t.get_conflict_by_type(conflicts).is_empty())
                                    .collect::<Vec<_>>(),
                                Some(self.selected_conflict_type),
                                Message::ConflictTypeSelected
                            )
                            .width(Length::Fill),
                            scrollable(Column::with_children(
                                sorted_conflicts
                                    .iter()
                                    .enumerate()
                                    .map(|(i, (identifier, _))| {
                                        Into::<Element<'_, Message>>::into(
                                            button(text!("{}", identifier)).on_press_maybe(
                                                if self
                                                    .selected_conflict_index
                                                    .is_none_or(|v| v != i)
                                                {
                                                    Some(Message::ConflictButtonPressed(i))
                                                } else {
                                                    None
                                                },
                                            ),
                                        )
                                    })
                            ))
                            .width(Length::Fill),
                        ])
                        .width(Length::FillPortion(1)),
                        if let Some(selected_conflict_index) = self.selected_conflict_index {
                            let (will_be_loaded_from, conflict_data) =
                                &sorted_conflicts[selected_conflict_index];
                            Into::<Element<'_, Message>>::into(
                                column![
                                    scrollable(Row::with_children(
                                        conflict_data.added_by.iter().enumerate().map(
                                            |(i, package)| {
                                                Into::<Element<'_, Message>>::into(
                                                    button(text!("{}", package.package_id()))
                                                        .on_press_maybe(
                                                            if self
                                                                .selected_conflict_file_index
                                                                .is_none_or(|v| v != i)
                                                            {
                                                                Some(
                                                                Message::ConflictFileButtonPressed(
                                                                    i,
                                                                ),
                                                            )
                                                            } else {
                                                                None
                                                            },
                                                        ),
                                                )
                                            },
                                        )
                                    ))
                                    .direction(
                                        scrollable::Direction::Horizontal(Default::default())
                                    ),
                                    row![
                                        text_editor(&self.conflict1_text)
                                            .height(Length::Fill)
                                            .on_action(Message::Conflict1EditorAction)
                                            .highlight("xml", self.xml_highlight_theme,),
                                        text_editor(&self.conflict2_text)
                                            .height(Length::Fill)
                                            .on_action(Message::Conflict2EditorAction)
                                            .highlight("xml", self.xml_highlight_theme,)
                                    ]
                                ]
                                .width(Length::FillPortion(5)),
                            )
                        } else {
                            Space::new(Length::FillPortion(5), Length::Fill).into()
                        }
                    ]
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
            Message::Conflict1EditorAction(action) => {
                if !action.is_edit() {
                    self.conflict1_text.perform(action);
                }
            }
            Message::Conflict2EditorAction(action) => {
                self.conflict2_text.perform(action);
            }
            Message::StartParsing => {
                let game_path = PathBuf::from(&self.args.game_path);
                let config_player_path = self.args.config_player_path();

                self.selected_conflict_file_index = None;
                self.selected_conflict_index = None;
                self.selected_conflict_type = ConflictType::Item;

                self.loading_state = Some(LoadingState::Started);

                let task = Task::stream(asset_parser::loading::load(game_path, config_player_path));

                return Task::done(Message::ScreenChanged(Screen::LoadingMods))
                    .chain(task.map(|progress| Message::LoadProgress(progress)))
                    .chain(Task::done(Message::LoadPatchMod));
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
            Message::ConflictTypeSelected(t) => {
                self.selected_conflict_file_index = None;
                self.selected_conflict_index = None;
                self.selected_conflict_type = t;
                self.conflict1_text.perform(text_editor::Action::SelectAll);
                self.conflict1_text
                    .perform(text_editor::Action::Edit(text_editor::Edit::Backspace));
            }
            Message::ConflictButtonPressed(i) => {
                self.selected_conflict_file_index = None;
                self.selected_conflict_index = Some(i);
                self.conflict1_text.perform(text_editor::Action::SelectAll);
                self.conflict1_text
                    .perform(text_editor::Action::Edit(text_editor::Edit::Backspace));
            }
            Message::ConflictFileButtonPressed(i) => {
                let Some(LoadingState::Finished(loaded_content_files, conflicts)) =
                    &self.loading_state
                else {
                    return Task::none();
                };
                let Some(selected_conflict_index) = &self.selected_conflict_index else {
                    return Task::none();
                };
                let selected_conflicts =
                    self.selected_conflict_type.get_conflict_by_type(conflicts);
                let mut sorted_conflicts = selected_conflicts.iter().collect::<Vec<_>>();
                sorted_conflicts.sort_by(|a, b| a.0.cmp(b.0));

                let conflict = &sorted_conflicts[*selected_conflict_index];
                let package = &conflict.1.added_by[i];
                let files = loaded_content_files
                    .iter()
                    .find(|(v, _)| Arc::ptr_eq(v, package))
                    .expect("Content package added by selected conflict wasn't found in loaded content packages! This is a developer error! If you know what causes this, please open an issue on https://github.com/FoLZer/barotrauma_conflict_finder/issues",);
                let file_path = self
                    .selected_conflict_type
                    .get_conflict_file_by_type(&files.1, conflict.0)
                    .expect(
                        "Selected conflict was not found in the files! This is a developer error! If you know what causes this, please open an issue on https://github.com/FoLZer/barotrauma_conflict_finder/issues",
                    );

                let text = std::fs::read_to_string(file_path).unwrap();

                self.conflict1_text.perform(text_editor::Action::SelectAll);
                self.conflict1_text
                    .perform(text_editor::Action::Edit(text_editor::Edit::Paste(
                        Arc::new(text),
                    )));
                //TODO: put cursor at the start of the prefab OR only paste the part with the prefab
                self.conflict1_text.perform(text_editor::Action::Move(
                    text_editor::Motion::DocumentStart,
                ));

                self.selected_conflict_file_index = Some(i)
            }
            Message::XMLHighlighterThemeSelected(theme) => {
                self.xml_highlight_theme = theme;
            }
            Message::PatchModLoaded(patch_mod_package) => {
                self.patch_mod = Some(patch_mod_package);
            }
            Message::PatchModPathChanged(s) => {
                self.args.patch_mod_path = Some(s);
            }
            Message::LoadPatchMod => {
                let Some(LoadingState::Finished(loaded_content_files, conflicts)) =
                    &self.loading_state
                else {
                    return Task::none();
                };

                let patch_mod_path = self.args.patch_mod_path();
                if !patch_mod_path.exists() {
                    log::info!("Patch Mod doesn't exist on the path provided, it will be created.");
                    std::fs::create_dir_all(&patch_mod_path)
                        .expect("Failed to create directories for patch mod");
                }
                let patch_mod_filelist_path = patch_mod_path.join("filelist.xml");
                let patch_mod_manifest_path = patch_mod_path.join("manifest.json");

                let (package, manifest) = if !patch_mod_filelist_path.exists() {
                    log::info!(
                        "Patch Mod's filelist.xml does not exist, it will be created from scratch."
                    );
                    let mut package = ContentPackage::<Regular>::default();
                    package.name = Some("Conflict Finder Patch Mod".to_owned());
                    package.game_version = Some(CURRENT_GAME_VERSION);

                    let manifest = ModManifest {
                        dependencies: loaded_content_files.iter().map(|f| Arc::new(ModIdentifier {
                            identifier: f.0.package_id_prefer_ugc_id(),
                            mod_hash: match f.0.expected_hash() {
                                Some(v) => Some(v.clone()),
                                None => {
                                    log::warn!("Mod {} doesn't have an expected hash, changes in this mod will not be detected! (The conflict resolutions relying on this mod will never be unresolved)", f.0.package_id());
                                    None
                                }
                            }
                        })).collect(),
                        ..Default::default()
                    };

                    package.save(&patch_mod_filelist_path).unwrap();
                    manifest.save(&patch_mod_manifest_path).unwrap();

                    (package, manifest)
                } else {
                    if !patch_mod_manifest_path.exists() {
                        log::error!(
                            "Patch Mod's manifest.json does not exist, the Patch Mod is invalid without one!"
                        );
                        panic!();
                    }
                    let package = ContentPackage::<Regular>::load(
                        &std::fs::read_to_string(patch_mod_filelist_path)
                            .expect("Failed to load patchmod filelist.xml"),
                    )
                    .expect("Failed to parse patchmod filelist.xml");

                    let manifest = ModManifest::load(&patch_mod_manifest_path).unwrap();

                    (package, manifest)
                };

                return Task::done(Message::PatchModLoaded(Arc::new((package, manifest))))
                    .chain(Task::done(Message::ScreenChanged(Screen::ConflictSolver)));
            }
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
                selected_conflict_index: None,
                selected_conflict_file_index: None,
                conflict1_text: Default::default(),
                conflict2_text: Default::default(),
                xml_highlight_theme: iced::highlighter::Theme::SolarizedDark,
                patch_mod: None,
            };
            (state, Task::none())
        })
}
