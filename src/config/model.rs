use serde::Deserialize;
use reqwest::Client;
use crate::clients::vehicle_client::VehicleClient;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Configuration,
    pub http_client: HttpClient
}

#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    pub base_url: String
}

#[derive(Debug, Clone)]
pub struct HttpClient {
    pub global_client: Client,
    pub typed_client: TypedClient
}

#[derive(Debug, Clone)]
pub struct TypedClient {
    pub vehicle_client: VehicleClient
}