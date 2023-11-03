// Copyright (c) 2023 Jim Hodapp & Caleb Bourg
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use service::{config::Config, AppState};

extern crate simplelog;

#[tokio::main]
async fn main() {
    let config = get_config();

    init_logger(&config);

    let mut app_state = AppState::new(config);
    app_state = service::init_database(app_state).await.unwrap();

    entity_api::seed_database(app_state.database_connection.as_ref().unwrap()).await;

    web::init_server(app_state).await.unwrap();
}

fn get_config() -> Config {
    Config::new()
}

fn init_logger(config: &Config) {
    let log_level = match config.trace_level {
        0 => simplelog::LevelFilter::Warn,
        1 => simplelog::LevelFilter::Info,
        2 => simplelog::LevelFilter::Debug,
        3 => simplelog::LevelFilter::Trace,
        _ => simplelog::LevelFilter::Trace,
    };

    simplelog::TermLogger::init(
        log_level,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .expect("Failed to start simplelog");

    simplelog::info!("<b>Starting up...</b>.");
}
