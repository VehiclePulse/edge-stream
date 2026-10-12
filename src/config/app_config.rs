use super::model::Configuration;
use config::{Config, File };

pub fn load_config() -> Configuration {
    let configurations = Config::builder()
        .add_source(File::with_name("config").required(true))
        .build()
        .expect("Failed to read the configuration from the file");

    configurations.try_deserialize().expect("failed to deserialize the data")
}