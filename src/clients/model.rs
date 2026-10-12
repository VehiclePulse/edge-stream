use core::str;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct VehicleResponse {
    registration_number: String,
    make: String,
    model: String,
    year: u16,
    simulation: Simulation
}

#[derive(Debug, Deserialize)]
struct Simulation {
    initial_latitude: f64,
    initial_longitude: f64,
    initial_heading: f64
}


#[derive(Debug, Serialize)]
pub struct TelemetryRequest {
    pub registration_numbers: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct TelemetryResponse {
    pub vehicles: Vec<TelemetryUpdateResponse>
}


#[derive(Debug, Serialize)]
pub struct TelemetryUpdateRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub speed: f64,
    pub heading: f64,

    pub fuel_level: f64,
    pub engine_temperature: f64,
    pub battery_level: f64,

    pub tire_pressure: TirePressure,
}

#[derive(Debug, Deserialize)]
pub struct TelemetryUpdateResponse {
    pub registration_number: String,
    pub latitude: f64,
    pub longitude: f64,
    pub speed: f64,
    pub heading: f64,

    pub fuel_level: f64,
    pub engine_temperature: f64,
    pub battery_level: f64,

    pub tire_pressure: TirePressure,
    pub last_updated_at: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TirePressure {
    pub front_left: f64,
    pub front_right: f64,
    pub rear_left: f64,
    pub rear_right: f64,
}