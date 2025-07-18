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

    #[derive(Default, Copy, Clone)]
    pub enum TimeFormat {
        H12Format, // 12 часовой формат
        #[default]
        H24Format, // 24 часовой формат
    }

    #[derive(Default, Copy, Clone)]
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
#[derive(Default)]
pub struct Logger {
    pub times: logger_utils::TimeFormat,
    pub dates: logger_utils::DateFormat,
}

impl Logger {
    pub fn message(&self, _type: logger_utils::MessageType, _msg: &String) {
        let date = time_date_utils::date_string(self.dates);
        let time = time_date_utils::time_string(self.times);
        match _type {
            logger_utils::MessageType::Critical => eprintln!(
                "\x1b[30m\x1b[41m{}\x1b[0m {date} {time} : {_msg}",
                _type.prefix()
            ),
            logger_utils::MessageType::Debug => {
                println!("\x1b[36m{}\x1b[0m {date} {time} : {_msg}", _type.prefix())
            }
            logger_utils::MessageType::Error => {
                eprintln!("\x1b[91m{}\x1b[0m {date} {time} : {_msg}", _type.prefix())
            }
            logger_utils::MessageType::Info => {
                println!("\x1b[35m{}\x1b[0m {date} {time} : {_msg}", _type.prefix())
            }
            logger_utils::MessageType::Warn => {
                println!("\x1b[33m{}\x1b[0m {date} {time} : {_msg}", _type.prefix())
            }
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
}

impl Logger {
    pub fn new(
        date_format: logger_utils::DateFormat,
        time_format: logger_utils::TimeFormat,
    ) -> Self {
        Self {
            times: time_format,
            dates: date_format,
        }
    }
}
