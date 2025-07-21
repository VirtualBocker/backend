use std::env;

pub mod logger_utils {

    pub enum MessageType {
        Debug,
        Info,
        Warn,
        Error,
        Critical,
    }
    
    #[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
    pub enum LogLevel {
        #[default]
        All,
        ImporatantOnly,
    }
    
    impl MessageType {
        pub fn prefix(&self) -> String {
            match self {
                Self::Debug => "[ DBUG ]".to_string(),
                Self::Info => "[ INFO ]".to_string(),
                Self::Warn => "[ WARN ]".to_string(),
                Self::Error => "[ ERRO ]".to_string(),
                Self::Critical => "[ CRIT ]".to_string(),
            }
        }
    }
    
    #[derive(Default, Copy, Clone, Debug)]
    pub enum TimeFormat {
        H12Format, // 12 часовой формат
        #[default]
        H24Format, // 24 часовой формат
    }
    
    #[derive(Default, Copy, Clone, Debug)]
    pub enum DateFormat {
        Asian,  // yyyy/mm/dd
        US,     // mm/dd/yyyy
        Europe, // dd.mm.yyyy
        #[default]
        ISO8601, // yyyy-mm-dd
    }
}

mod time_date_utils {
    use crate::lib::logger::logger_utils;
    use chrono::Local;
    
    pub fn time_string(time_format: logger_utils::TimeFormat) -> String {
        let now = Local::now();
        match time_format {
            logger_utils::TimeFormat::H12Format => now.time().format("%I:%M:%S %p").to_string(),
            logger_utils::TimeFormat::H24Format => now.time().format("%H:%M:%S").to_string(),
        }
    }
    
    pub fn date_string(date_format: logger_utils::DateFormat) -> String {
        let now = Local::now();
        match date_format {
            logger_utils::DateFormat::Asian => now.date_naive().format("%Y/%m/%d").to_string(),
            logger_utils::DateFormat::Europe => now.date_naive().format("%d.%m.%Y").to_string(),
            logger_utils::DateFormat::ISO8601 => now.date_naive().format("%Y-%m-%d").to_string(),
            logger_utils::DateFormat::US => now.date_naive().format("%m/%d/%Y").to_string(),
        }
    }
}

mod logger_constants {
    pub const IMPORATANT_ONLY: &str = "important_only";
    pub const MOTD: &str = r"
    ____             __            
   / __ )____  _____/ /_____  _____
  / __  / __ \/ ___/ //_/ _ \/ ___/
 / /_/ / /_/ / /__/ ,< /  __/ /    
/_____/\____/\___/_/|_|\___/_/
";
}

#[derive(Default, Debug)]
pub struct Logger {
    pub times: logger_utils::TimeFormat,
    pub dates: logger_utils::DateFormat,
    pub levels:logger_utils::LogLevel,
}

impl Logger {
    pub fn message(&self, _type: logger_utils::MessageType, _msg: &String) {
        let date = time_date_utils::date_string(self.dates);
        let time = time_date_utils::time_string(self.times);
        match _type {
            logger_utils::MessageType::Critical => eprintln!(
                "☠️ \x1b[30m\x1b[41m{}\x1b[0m {date} {time} :: {_msg}",
                _type.prefix()
            ),
            logger_utils::MessageType::Debug => println!(
                "🛠️  \x1b[36m{}\x1b[0m {date} {time} :: {_msg}",
                _type.prefix()
            ),
            logger_utils::MessageType::Error => eprintln!(
                "💥 \x1b[91m{}\x1b[0m {date} {time} :: {_msg}",
                _type.prefix()
            ),
            logger_utils::MessageType::Info =>  if self.levels != logger_utils::LogLevel::ImporatantOnly { println!(
                "🚬 \x1b[35m{}\x1b[0m {date} {time} :: {_msg}",
                _type.prefix()
            ) },
            logger_utils::MessageType::Warn =>  if self.levels != logger_utils::LogLevel::ImporatantOnly { println!(
                "⚠️ \x1b[33m{}\x1b[0m {date} {time} :: {_msg}",
                _type.prefix()
            ) },
            
        };
    }

    pub fn critical(&self, _msg: &String) {
        self.message(logger_utils::MessageType::Critical, _msg);
    }

    pub fn debug(&self, _msg: &String) {
        self.message(logger_utils::MessageType::Debug, _msg);
    }

    pub fn error(&self, _msg: &String) {
        self.message(logger_utils::MessageType::Error, _msg);
    }

    pub fn info(&self, _msg: &String) {
        self.message(logger_utils::MessageType::Info, _msg);
    }

    pub fn warn(&self, _msg: &String) {
        self.message(logger_utils::MessageType::Warn, _msg);
    }

    pub fn motd(&self) {
        println!("{}\tVersion \x1b[32m{}\x1b[0m.\x1b[31m{}\x1b[0m\n\n" , logger_constants::MOTD, env!("CARGO_PKG_VERSION_MAJOR"), env!("CARGO_PKG_VERSION_MINOR"));
    }
}

impl Logger {
    pub fn new() -> Self {

        let date_format = env::var("DATE_FORMAT");
        let time_format = env::var("TIME_FORMAT");

        let log_lel: logger_utils::LogLevel;
        if let Ok(s) = env::var("LOG_LEVEL") {
            match s.as_str() {
                logger_constants::IMPORATANT_ONLY => 
                {
                    log_lel = logger_utils::LogLevel::ImporatantOnly;
                }
                _ => log_lel = logger_utils::LogLevel::All,
            }
        } else {
            log_lel = logger_utils::LogLevel::All
        }
        Self {
            dates:match date_format{
                Ok(s) => {
                    match s.as_str() {
                        "ISO8601" => logger_utils::DateFormat::ISO8601,
                        "Asia" => logger_utils::DateFormat::Asian,
                        "Europe" => logger_utils::DateFormat::Europe,
                        "US" => logger_utils::DateFormat::US,
                        _ => logger_utils::DateFormat::default()
                    }
                },
                Err(_) => logger_utils::DateFormat::default()
            },
            times: match time_format{
                Ok(s) =>{
                    match s.as_str() {
                        "H12Format" => logger_utils::TimeFormat::H12Format,
                        "H24Format" => logger_utils::TimeFormat::H24Format,
                        _ => logger_utils::TimeFormat::default()
                    }
                }
                Err(_) => logger_utils::TimeFormat::default()
            },
            levels:log_lel
        }
    }
}
