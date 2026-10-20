use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct InitialTelemetryDto {
    pub vehicle_id: Uuid,
    pub registration_number: String,
    pub latitude: f64,
    pub longitude: f64,
    pub speed: f64,
    pub heading: f64,
    pub fuel_level: f64,
    pub engine_temperature: f64,
    pub battery_level: f64,
    pub tire_pressure: TirePressureDto,
    pub mode: VehicleModeDto,
}

#[derive(Debug, Deserialize)]
pub struct TirePressureDto {
    pub front_left: f64,
    pub front_right: f64,
    pub rear_left: f64,
    pub rear_right: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VehicleModeDto {
    Driving,
    Stopping,
}