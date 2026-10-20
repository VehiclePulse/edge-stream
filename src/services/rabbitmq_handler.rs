use serde::de::DeserializeOwned;
use vehicle_simulator::model::shared::state::VehicleMode;
use vehicle_simulator::model::{InitialTelemetry, TirePressure};
use std::error::Error;
use uuid::Uuid;

use crate::config::model::AppState;
use crate::services::models::InitialTelemetryDto;

type HandlerError = Box<dyn Error + Send + Sync>;

pub async fn handle(
    routing_key: &str,
    payload: &[u8],
    app_state: &AppState
) -> Result<(), HandlerError> {
    match routing_key {
        "vehicle.started" => {
            let dto: InitialTelemetryDto = deserialize(payload)?;

            app_state.simulator
                .start_vehicle(InitialTelemetry{
                    vehicle_id: dto.vehicle_id,
                    registration_number: dto.registration_number.clone(),
                    latitude: dto.latitude,
                    longitude: dto.longitude,
                    speed: dto.speed,
                    heading: dto.heading,
                    fuel_level: dto.fuel_level,
                    engine_temperature: dto.engine_temperature,
                    battery_level: dto.battery_level,
                    tire_pressure: TirePressure {
                        front_left: dto.tire_pressure.front_left,
                        front_right: dto.tire_pressure.front_right,
                        rear_left: dto.tire_pressure.rear_left,
                        rear_right: dto.tire_pressure.rear_right
                    },
                    mode: match dto.mode {
                        super::models::VehicleModeDto::Driving => VehicleMode::Driving,
                        super::models::VehicleModeDto::Stopping => VehicleMode::Stopping
                    }
                })
                .await
                .map_err(|err| std::io::Error::other(err))?;
        }

        "vehicle.stopped" => {
            let vehicle_id: Uuid = deserialize(payload)?;

            app_state.simulator
                .stop_vehicle(vehicle_id)
                .await
                .map_err(|err| std::io::Error::other(err))?;
        }

        _ => {
            return Err(format!("Unsupported routing key: {routing_key}").into());
        }
    }

    Ok(())
}

fn deserialize<T: DeserializeOwned>(payload: &[u8]) -> Result<T, HandlerError> {
    Ok(serde_json::from_slice(payload)?)
}