use reqwest::Client;

use crate::config::model::AppState;
use super::model::{ VehicleResponse, TelemetryRequest, TelemetryResponse, TelemetryUpdateRequest };

#[derive(Debug, Clone)]
pub struct VehicleClient {
    client: Client,
    base_url: String
}

impl VehicleClient {
    pub fn new(app_state: AppState) -> Self {
        VehicleClient {
            client: app_state.http_client.global_client,
            base_url: app_state.config.base_url
        }
    }

    pub async fn get_vehicles(&self) -> Result<Vec<VehicleResponse>, reqwest::Error> {
        let url = format!("{}/vehicles", self.base_url);

        let response = self.client.get(&url)
            .query(&[("enabled", "true")])
            .send()
            .await?
            .json::<Vec<VehicleResponse>>()
            .await?;

        Ok(response)
    }

    pub async fn get_telemetry(&self, payload: &TelemetryRequest) -> Result<Vec<TelemetryResponse>, reqwest::Error> {
        let url = format!("{}/vehicles/telemetry/batch", self.base_url);

        let response = self.client.get(&url)
            .json(payload)
            .send()
            .await?
            .json::<Vec<TelemetryResponse>>()
            .await?;

        Ok(response)
    }

    pub async fn update_telemetry(&self, registration_number: String, payload: TelemetryUpdateRequest
    ) -> Result<() ,reqwest::Error> {
        let url = format!("{}/vehicles/{}/telemetry", self.base_url, registration_number);

        let response = self.client.put(&url)
            .json(&payload)
            .send()
            .await?
            .json::<Vec<VehicleResponse>>()
            .await?;



        Ok(())
    }
 

}