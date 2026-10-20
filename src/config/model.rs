use std::sync::Arc;

use serde::Deserialize;
use tokio_util::sync::CancellationToken;
use vehicle_simulator::simulator::Simulator;
use crate::clients::vehicle_client::VehicleClient;

#[derive(Clone)]
pub struct AppState {
    pub config: Configuration,
    pub http_client: HttpClient,
    pub simulator: Arc<Simulator>,
    pub cancellation_token: CancellationToken
}

#[derive(Deserialize, Clone)]
pub struct Configuration {
    pub base_url: String,
    pub rabbitmq: RabbitMq
}

#[derive(Clone)]
pub struct HttpClient {
    pub vehicle_client: VehicleClient
}

#[derive(Deserialize, Clone)]
pub struct RabbitMq {
    pub url: String,
    pub queue: String,
    pub prefetch: u16
}