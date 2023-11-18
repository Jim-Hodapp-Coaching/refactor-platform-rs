use clap::Parser;

#[derive(Clone, Parser)]
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

    /// Turn on different tracing levels [0 = Warn, 1 = Info, 2 = Debug, 3 = Trace]
    #[arg(short, long, default_value_t = 0)]
    pub trace_level: u8,
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
