mod config;
mod clients;
use config::app_config;
use crate::{clients::model::{TelemetryRequest, TelemetryUpdateRequest, TirePressure}, config::model::{AppState, HttpClient, TypedClient}};

#[tokio::main]
async fn main() {
    let configuration = app_config::load_config();
    let http_client = clients::init::init();
    let vehicle_client = clients::vehicle_client::VehicleClient::new(http_client.clone(), configuration.base_url.clone());

    let app_state = AppState {
        config: configuration,
        http_client: HttpClient { 
            global_client: http_client, 
            typed_client: TypedClient {
                vehicle_client: vehicle_client
            } 
        }
    };

    let vehicles = app_state.http_client.typed_client.vehicle_client.get_vehicles().await;
    //println!("vehicles: {:#?}", vehicles);
    let reg_numbers: Vec<String> = vehicles.unwrap()
                .iter()
                .map(|vehicle| vehicle.registration_number.clone())
                .collect();
    println!("registration numbers: {:#?}", reg_numbers);

    let telemetry = app_state.http_client.typed_client.vehicle_client.get_telemetry(&TelemetryRequest{
        registration_numbers: reg_numbers
    }).await;
    println!("telemetry data: {:#?}", telemetry);


    for tel in telemetry.unwrap() {
        println!("telemetry update call for vehicle: {}", tel.registration_number.clone());

        let payload = TelemetryUpdateRequest {
            latitude: tel.latitude,
            longitude: tel.longitude,
            speed: tel.speed + 20.0,
            heading: tel.heading + 2.0,
            fuel_level: tel.fuel_level,
            engine_temperature: tel.engine_temperature + 40.0,
            battery_level: tel.battery_level,
            tire_pressure: TirePressure {
                front_left: tel.tire_pressure.front_left,
                front_right: tel.tire_pressure.front_right,
                rear_left: tel.tire_pressure.rear_left,
                rear_right: tel.tire_pressure.rear_right,
            },
        };

        let updated_telemetry = app_state.http_client.typed_client.vehicle_client
            .update_telemetry(tel.registration_number, payload).await;
        print!("updated telemetry: {:#?}", updated_telemetry);

    }



    print!("hi");
    
}
