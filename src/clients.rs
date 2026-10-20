use crate::{clients::vehicle_client::VehicleClient, config::model::Configuration};

pub mod init;
pub mod vehicle_client;
pub mod model;


pub fn init_client(config: &Configuration) -> VehicleClient {
    let http_client = init::init();
    vehicle_client::VehicleClient::new(http_client.clone(), config.base_url.clone())
}