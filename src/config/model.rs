use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppState {
    pub config: Configuration
}

#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    pub vehicle_management: VehicleManagement
}

#[derive(Debug, Deserialize, Clone)]
pub struct VehicleManagement {
    pub vehicle_url: String,
    pub telemetry_url: String
}