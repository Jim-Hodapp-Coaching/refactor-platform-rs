use clap::builder::TypedValueParser as _;
use clap::Parser;
use std::fmt;

#[derive(Clone, Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Sets the Postgresql database URI to connect to
    #[arg(
        short,
        long,
        env,
        default_value = "postgres://refactor_rs:password@localhost:5432/refactor_platform_rs"
    )]
    pub database_uri: Option<String>,

    /// The host interface to listen for incoming connections
    #[arg(short, long, default_value = "127.0.0.1")]
    pub interface: Option<String>,

    /// The host TCP port to listen for incoming connections
    #[arg(short, long, default_value_t = 4000)]
    pub port: u16,

    /// Turn on log level verbosity threshold to control what gets displayed on console output
    #[arg(
        short,
        long,
        default_value_t = LogLevel::Warn,
        value_parser = clap::builder::PossibleValuesParser::new(["error", "warn", "info", "debug", "trace"])
            .map(|s| s.parse::<LogLevel>().unwrap()),
        )]
    pub log_level: LogLevel,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Config::parse()
    }

    pub fn set_database_uri(mut self, database_uri: String) -> Self {
        self.database_uri = Some(database_uri);
        self
    }

    pub fn database_uri(&self) -> String {
        self.database_uri.clone().expect("No Database URI Provided")
    }
}

#[derive(Clone, Debug, Default)]
#[repr(u8)]
pub enum LogLevel {
    Error = 0,
    #[default]
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for u8 {
    fn from(trace_level: LogLevel) -> u8 {
        match trace_level {
            LogLevel::Error => 0,
            LogLevel::Warn => 1,
            LogLevel::Info => 2,
            LogLevel::Debug => 3,
            LogLevel::Trace => 4,
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level_string = match self {
            LogLevel::Error => "error",
            LogLevel::Warn => "warn",
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
            LogLevel::Trace => "trace",
        };

        level_string.fmt(f)
    }
}

impl std::str::FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "trace" => Ok(Self::Trace),
            "debug" => Ok(Self::Debug),
            "info" => Ok(Self::Info),
            "warn" => Ok(Self::Warn),
            "error" => Ok(Self::Error),
            _ => Err(format!("Unknown trace level: {s}")),
        }
    }
}
