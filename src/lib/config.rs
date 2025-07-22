use std::env;

pub mod config_enums{
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

    #[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
    pub enum LogLevel {
        #[default]
        Info,
        Dbug,
        Warn,
        Error
    }
}
#[derive(Debug)]
pub struct Config{
    pub port:String,
    pub time_format:config_enums::TimeFormat,
    pub date_format:config_enums::DateFormat,
    pub log_level:config_enums::LogLevel
}

impl Default for Config{
    fn default() -> Self{
        Self{
            port:"8080".to_string(),
            time_format:config_enums::TimeFormat::default(),
            date_format:config_enums::DateFormat::default(),
            log_level:config_enums::LogLevel::default()
        }
    }
}

mod config_constants {
    pub const DBUG: &str = "debug";
    pub const INFO: &str = "info";
    pub const WARN: &str = "warn";
    pub const ERROR: &str = "error";
    pub const ASIAN: &str = "asian";
    pub const EUROPE: &str = "european";
    pub const ISO : &str = "iso";
    pub const US: &str = "us";
    pub const H12FORMAT: &str = "h12";
    pub const H24FORMAT: &str = "h24";
}

impl Config{
    fn from_env() -> Self {
        Self{
             date_format: match env::var("DATE_FORMAT")
            .unwrap_or("".to_string())
            .as_str() 
            {
                config_constants::ISO       =>  config_enums::DateFormat::ISO8601,
                config_constants::ASIAN     =>  config_enums::DateFormat::Asian,
                config_constants::EUROPE    =>  config_enums::DateFormat::Europe,
                config_constants::US        =>  config_enums::DateFormat::US,
                _ => config_enums::DateFormat::default()
            },
            time_format: match env::var("TIME_FORMAT")
            .unwrap_or("".to_string())
            .as_str() 
            {
                config_constants::H12FORMAT => config_enums::TimeFormat::H12Format,
                config_constants::H24FORMAT => config_enums::TimeFormat::H24Format,
                _ => config_enums::TimeFormat::default()
            },
            log_level: match env::var("LOG_LEVEL")
            .unwrap_or("".to_string())
            .as_str() 
            {
                config_constants::DBUG  =>  config_enums::LogLevel::Dbug,
                config_constants::INFO  =>  config_enums::LogLevel::Info,
                config_constants::WARN  =>  config_enums::LogLevel::Warn,
                config_constants::ERROR =>  config_enums::LogLevel::Error,
                _                       =>  config_enums::LogLevel::default(),
            },
            port:std::env::var("PORT").unwrap_or("8080".to_string())
        }
    }
}

