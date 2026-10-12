use super::model::{ AppState, Configuration };
use config::{Config, File };

pub fn load_config() -> AppState {
    let configurations = config::Config::builder()
        .add_source(config::File::with_name("config").required(true))
        .build()
        .expect("Failed to read the configuration from the file");

    let configuration: Configuration = configurations.try_deserialize().expect("failed to deserialize the data");

    AppState {
        config: configuration
    }

}