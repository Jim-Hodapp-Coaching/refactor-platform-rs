use clap::builder::TypedValueParser as _;
use clap::Parser;
use log::LevelFilter;

#[derive(Clone, Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Sets the Postgresql database URI to connect to
    #[arg(
        short,
        long,
        env,
        default_value = "postgres://refactor:password@localhost:5432/refactor_platform"
    )]
    database_uri: Option<String>,

    /// The host interface to listen for incoming connections
    #[arg(short, long, default_value = "127.0.0.1")]
    pub interface: Option<String>,

    /// The host TCP port to listen for incoming connections
    #[arg(short, long, default_value_t = 4000)]
    pub port: u16,

    /// Set the log level verbosity threshold (level) to control what gets displayed on console output
    #[arg(
        short,
        long,
        default_value_t = LevelFilter::Warn,
        value_parser = clap::builder::PossibleValuesParser::new(["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"])
            .map(|s| s.parse::<LevelFilter>().unwrap()),
        )]
    pub log_level_filter: LevelFilter,
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

    pub fn database_uri(&self) -> &str {
        self.database_uri
            .as_ref()
            .expect("No Database URI Provided")
    }
}
