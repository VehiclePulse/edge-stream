use vehicle_simulator::model::{InitialTelemetry, TirePressure, shared::state::VehicleMode};
use crate::{clients::{model::TelemetryRequest}, config::model::AppState};

pub async fn init(app_state: &AppState) {
    let vehicles = app_state.http_client.vehicle_client.get_vehicles().await.unwrap();

    let mut telemetry_request: Vec<String> = Vec::new();
    for vehicle in vehicles {      
        telemetry_request.push(vehicle.registration_number.clone());
    }

    let telemetries = app_state.http_client.vehicle_client.get_telemetry(&TelemetryRequest {
        registration_numbers: telemetry_request
    }).await.unwrap();

    for telemetry in telemetries {

        let res = app_state.simulator.start_vehicle(InitialTelemetry {
            vehicle_id: telemetry.vehicle_id,
            registration_number: telemetry.registration_number.clone(),
            latitude: telemetry.latitude,
            longitude: telemetry.longitude,
            speed: telemetry.speed,
            heading: telemetry.heading,
            fuel_level: telemetry.fuel_level,
            engine_temperature: telemetry.engine_temperature,
            battery_level: telemetry.battery_level,
            tire_pressure: TirePressure {
                front_left: telemetry.tire_pressure.front_left,
                front_right: telemetry.tire_pressure.front_right,
                rear_left: telemetry.tire_pressure.rear_left,
                rear_right: telemetry.tire_pressure.rear_right
            },
            mode: VehicleMode::Driving
        })
        .await;

        match res {
            Ok(()) => println!("vehicle started successfully"),
            Err(error) => println!("Failed to start the vehicle due to {error}")
        }
    }




}