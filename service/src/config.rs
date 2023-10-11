pub struct Config {
    database_uri: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self { database_uri: None }
    }

    pub fn set_database_uri(mut self, database_uri: String) -> Self {
        self.database_uri = Some(database_uri);
        self
    }

    pub fn database_uri(&self) -> String {
        self.database_uri.clone().expect("No Database URI Provided")
    }
}
