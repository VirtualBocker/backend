use crate::lib::config;
use std::env;
pub mod logger_utils {

    pub enum MessageType {
        Debug,
        Info,
        Warn,
        Error,
        Critical,
    }

    impl MessageType {
        pub fn prefix(&self) -> &str {
            match self {
                Self::Debug => "[ DBUG ]",
                Self::Info => "[ INFO ]",
                Self::Warn => "[ WARN ]",
                Self::Error => "[ ERRO ]",
                Self::Critical => "[ CRIT ]",
            }
        }
    }
}

mod time_date_utils {
    use crate::lib::config;
    use chrono::{
        Local,
        format::{DelayedFormat, StrftimeItems},
    };

    pub fn time_string(
        time_format: config::config_enums::TimeFormat,
    ) -> DelayedFormat<StrftimeItems<'static>> {
        // 'static так как формат-строки тоже 'static
        let now = Local::now();
        match time_format {
            config::config_enums::TimeFormat::H12Format => now.time().format("%I:%M:%S %p"),
            config::config_enums::TimeFormat::H24Format => now.time().format("%H:%M:%S"),
        }
    }

    pub fn date_string(
        date_format: config::config_enums::DateFormat,
    ) -> DelayedFormat<StrftimeItems<'static>> {
        // 'static так как формат-строки тоже 'static
        let now = Local::now();
        match date_format {
            config::config_enums::DateFormat::Asian => now.date_naive().format("%Y/%m/%d"),
            config::config_enums::DateFormat::Europe => now.date_naive().format("%d.%m.%Y"),
            config::config_enums::DateFormat::ISO8601 => now.date_naive().format("%Y-%m-%d"),
            config::config_enums::DateFormat::US => now.date_naive().format("%m/%d/%Y"),
        }
    }
}

#[derive(Default, Debug)]
pub struct Logger {
    times: config::config_enums::TimeFormat,
    dates: config::config_enums::DateFormat,
    levels: config::config_enums::LogLevel,
}

impl Logger {
    pub fn message(&self, msg_type: logger_utils::MessageType, msg: &String) {
        let date = time_date_utils::date_string(self.dates);
        let time = time_date_utils::time_string(self.times);
        match msg_type {
            logger_utils::MessageType::Critical => eprintln!(
                "☠️  \x1b[30m\x1b[41m{}\x1b[0m {date} {time} :: {msg}",
                msg_type.prefix()
            ),
            logger_utils::MessageType::Error => eprintln!(
                "💥 \x1b[91m{}\x1b[0m {date} {time} :: {msg}",
                msg_type.prefix()
            ),
            logger_utils::MessageType::Warn => {
                if self.levels == config::config_enums::LogLevel::Warn
                    || self.levels == config::config_enums::LogLevel::Dbug
                    || self.levels == config::config_enums::LogLevel::Info
                {
                    println!(
                        "⚠️  \x1b[33m{}\x1b[0m {date} {time} :: {msg}",
                        msg_type.prefix()
                    )
                }
            }
            logger_utils::MessageType::Debug => {
                if self.levels == config::config_enums::LogLevel::Dbug
                    || self.levels == config::config_enums::LogLevel::Info
                {
                    println!(
                        "🛠️  \x1b[36m{}\x1b[0m {date} {time} :: {msg}",
                        msg_type.prefix()
                    )
                }
            }
            logger_utils::MessageType::Info => {
                if self.levels == config::config_enums::LogLevel::Info {
                    println!(
                        "🚬 \x1b[35m{}\x1b[0m {date} {time} :: {msg}",
                        msg_type.prefix()
                    )
                }
            }
        };
    }

    pub fn critical(&self, msg: &String) {
        self.message(logger_utils::MessageType::Critical, msg);
    }

    pub fn debug(&self, msg: &String) {
        self.message(logger_utils::MessageType::Debug, msg);
    }

    pub fn error(&self, msg: &String) {
        self.message(logger_utils::MessageType::Error, msg);
    }

    pub fn info(&self, msg: &String) {
        self.message(logger_utils::MessageType::Info, msg);
    }

    pub fn warn(&self, msg: &String) {
        self.message(logger_utils::MessageType::Warn, msg);
    }

    pub fn motd() {
        println!(
            "{}\tVersion \x1b[32m{}\x1b[0m.\x1b[31m{}\x1b[0m \t\x1b[31mA\x1b[34mB\x1b[32mIS\x1b[0m\n\n",
            logger_constants::MOTD,
            env!("CARGO_PKG_VERSION_MAJOR"),
            env!("CARGO_PKG_VERSION_MINOR")
        );
    }
}

impl Logger {
    pub fn with_config(config: &config::Config) -> Self {
        Self {
            times: config.time_format,
            dates: config.date_format,
            levels: config.log_level,
        }
    }
}

mod logger_constants {
    pub const MOTD: &str = r"
    ____             __            
   / __ )____  _____/ /_____  _____
  / __  / __ \/ ___/ //_/ _ \/ ___/
 / /_/ / /_/ / /__/ ,< /  __/ /    
/_____/\____/\___/_/|_|\___/_/
";
}
