use std::sync::Arc;

use iced::futures::channel::{
    self,
    mpsc::{UnboundedReceiver, UnboundedSender},
};
use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};

/// https://github.com/borntyping/rust-simple_logger
pub struct SimpleLogger {
    /// The default logging level
    default_level: LevelFilter,

    /// The specific logging level for each module
    ///
    /// This is used to override the default value for some specific modules.
    ///
    /// This must be sorted from most-specific to least-specific, so that [`enabled`](#method.enabled) can scan the
    /// vector for the first match to give us the desired log level for a module.
    module_levels: Vec<(String, LevelFilter)>,

    /// Whether to include thread names (and IDs) or not
    ///
    /// This field is only available if the `threads` feature is enabled.
    //#[cfg(feature = "threads")]
    //threads: bool,

    /// Control how timestamps are displayed.
    ///
    /// This field is only available if the `timestamps` feature is enabled.
    //#[cfg(feature = "timestamps")]
    //timestamps: Timestamps,
    //#[cfg(feature = "timestamps")]
    //timestamps_format: Option<&'static [FormatItem<'static>]>,
    tx: Arc<std::sync::Mutex<UnboundedSender<String>>>,
    pub rx: Arc<iced::futures::lock::Mutex<UnboundedReceiver<String>>>,
}

impl SimpleLogger {
    /// Initializes the global logger with a SimpleLogger instance with
    /// default log level set to `Level::Trace`.
    ///
    /// ```no_run
    /// use simple_logger::SimpleLogger;
    /// SimpleLogger::new().env().init().unwrap();
    /// log::warn!("This is an example message.");
    /// ```
    ///
    /// [`init`]: #method.init
    #[must_use = "You must call init() to begin logging"]
    pub fn new() -> SimpleLogger {
        let (tx, rx) = channel::mpsc::unbounded();
        SimpleLogger {
            default_level: LevelFilter::Trace,
            module_levels: Vec::new(),

            //#[cfg(feature = "threads")]
            //threads: false,

            //#[cfg(feature = "timestamps")]
            //timestamps: Timestamps::Utc,

            //#[cfg(feature = "timestamps")]
            //timestamps_format: None,
            tx: Arc::new(std::sync::Mutex::new(tx)),
            rx: Arc::new(iced::futures::lock::Mutex::new(rx)),
        }
    }

    /// Configure the logger
    pub fn max_level(&self) -> LevelFilter {
        let max_level = self
            .module_levels
            .iter()
            .map(|(_name, level)| level)
            .copied()
            .max();
        max_level
            .map(|lvl| lvl.max(self.default_level))
            .unwrap_or(self.default_level)
    }

    /// 'Init' the actual logger and instantiate it,
    /// this method MUST be called in order for the logger to be effective.
    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_max_level(self.max_level());
        log::set_boxed_logger(Box::new(self))
    }
}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        &metadata.level().to_level_filter()
            <= self
                .module_levels
                .iter()
                /* At this point the Vec is already sorted so that we can simply take
                 * the first match
                 */
                .find(|(name, _level)| metadata.target().starts_with(name))
                .map(|(_name, level)| level)
                .unwrap_or(&self.default_level)
            && metadata.target() == "barotrauma_conflict_finder"
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level_string = format!("{:<5}", record.level().to_string());

            let target = if !record.target().is_empty() {
                record.target()
            } else {
                record.module_path().unwrap_or_default()
            };

            let thread = {
                /*
                #[cfg(feature = "threads")]
                if self.threads {
                    let thread = std::thread::current();

                    format!("@{}", {
                        #[cfg(feature = "nightly")]
                        {
                            thread.name().unwrap_or(&thread.id().as_u64().to_string())
                        }

                        #[cfg(not(feature = "nightly"))]
                        {
                            thread.name().unwrap_or("?")
                        }
                    })
                } else {
                    "".to_string()
                }

                #[cfg(not(feature = "threads"))]
                */
                ""
            };

            let timestamp = {
                /*
                #[cfg(feature = "timestamps")]
                match self.timestamps {
                    Timestamps::None => "".to_string(),
                    Timestamps::Local => format!(
                        "{} ",
                        OffsetDateTime::now_local()
                            .expect(concat!(
                                "Could not determine the UTC offset on this system. ",
                                "Consider displaying UTC time instead. ",
                                "Possible causes are that the time crate does not implement \"local_offset_at\" ",
                                "on your system, or that you are running in a multi-threaded environment and ",
                                "the time crate is returning \"None\" from \"local_offset_at\" to avoid unsafe ",
                                "behaviour. See the time crate's documentation for more information. ",
                                "(https://time-rs.github.io/internal-api/time/index.html#feature-flags)"
                            ))
                            .format(&self.timestamps_format.unwrap_or(TIMESTAMP_FORMAT_OFFSET))
                            .unwrap()
                    ),
                    Timestamps::Utc => format!(
                        "{} ",
                        OffsetDateTime::now_utc()
                            .format(&self.timestamps_format.unwrap_or(TIMESTAMP_FORMAT_UTC))
                            .unwrap()
                    ),
                    Timestamps::UtcOffset(offset) => format!(
                        "{} ",
                        OffsetDateTime::now_utc()
                            .to_offset(offset)
                            .format(&self.timestamps_format.unwrap_or(TIMESTAMP_FORMAT_OFFSET))
                            .unwrap()
                    ),
                }

                #[cfg(not(feature = "timestamps"))]
                */
                ""
            };

            let message = format!(
                "{}{} [{}{}] {}",
                timestamp,
                level_string,
                target,
                thread,
                record.args()
            );

            self.tx.lock().unwrap().unbounded_send(message).unwrap();
        }
    }

    fn flush(&self) {}
}
