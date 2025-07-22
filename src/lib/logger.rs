use std::env;
use crate::lib::config;
pub mod logger_utils {

    pub enum MessageType {
        Debug,
        Info,
        Warn,
        Error,
        Critical,
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
}

mod time_date_utils {
    use crate::lib::config;
    use chrono::Local;
    
    pub fn time_string(time_format: config::config_enums::TimeFormat) -> String {
        let now = Local::now();
        match time_format {
            config::config_enums::TimeFormat::H12Format => now.time().format("%I:%M:%S %p").to_string(),
            config::config_enums::TimeFormat::H24Format => now.time().format("%H:%M:%S").to_string(),
        }
    }
    
    pub fn date_string(date_format: config::config_enums::DateFormat) -> String {
        let now = Local::now();
        match date_format {
            config::config_enums::DateFormat::Asian => now.date_naive().format("%Y/%m/%d").to_string(),
            config::config_enums::DateFormat::Europe => now.date_naive().format("%d.%m.%Y").to_string(),
            config::config_enums::DateFormat::ISO8601 => now.date_naive().format("%Y-%m-%d").to_string(),
            config::config_enums::DateFormat::US => now.date_naive().format("%m/%d/%Y").to_string(),
        }
    }
}

#[derive(Default, Debug)]
pub struct Logger {
    times: config::config_enums::TimeFormat,
    dates: config::config_enums::DateFormat,
    levels:config::config_enums::LogLevel,
}

impl Logger {
    pub fn message(&self, _type: logger_utils::MessageType, _msg: &String) {
        let date = time_date_utils::date_string(self.dates);
        let time = time_date_utils::time_string(self.times);
        match _type {
            logger_utils::MessageType::Critical => eprintln!(
                "☠️  \x1b[30m\x1b[41m{}\x1b[0m {date} {time} :: {_msg}",
                _type.prefix()
            ),
            logger_utils::MessageType::Error => eprintln!(
                "💥 \x1b[91m{}\x1b[0m {date} {time} :: {_msg}",
                _type.prefix()
            ),
            logger_utils::MessageType::Warn =>  if self.levels == config::config_enums::LogLevel::Warn || 
            self.levels == config::config_enums::LogLevel::Dbug ||
            self.levels == config::config_enums::LogLevel::Info { 
            println!(
                "⚠️  \x1b[33m{}\x1b[0m {date} {time} :: {_msg}",
                _type.prefix()
            ) },
            logger_utils::MessageType::Debug => if self.levels == config::config_enums::LogLevel::Dbug ||
            self.levels == config::config_enums::LogLevel::Info{
            println!(
                "🛠️  \x1b[36m{}\x1b[0m {date} {time} :: {_msg}",
                _type.prefix()
            )},
            logger_utils::MessageType::Info =>  if self.levels == config::config_enums::LogLevel::Info {
            println!(
                "🚬 \x1b[35m{}\x1b[0m {date} {time} :: {_msg}",
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
        println!("{}\tVersion \x1b[32m{}\x1b[0m.\x1b[31m{}\x1b[0m \t\x1b[31mA\x1b[34mB\x1b[32mIS\x1b[0m\n\n" , 
        logger_constants::MOTD, 
        env!("CARGO_PKG_VERSION_MAJOR"), 
        env!("CARGO_PKG_VERSION_MINOR"));
    }
}


impl Logger {
    pub fn with_config(config: &config::Config) -> Self{
        Self {
            times:config.time_format,
            dates:config.date_format,
            levels:config.log_level
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

