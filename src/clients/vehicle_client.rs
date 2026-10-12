use reqwest::Client;

use crate::clients::model::TelemetryUpdateResponse;

use super::model::{ VehicleResponse, TelemetryRequest, TelemetryUpdateRequest };

#[derive(Debug, Clone)]
pub struct VehicleClient {
    client: Client,
    base_url: String
}

impl VehicleClient {
    pub fn new(client: Client, base_url: String) -> Self {
        VehicleClient {
            client: client,
            base_url: base_url
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

    pub async fn get_telemetry(&self, payload: &TelemetryRequest) -> Result<Vec<TelemetryUpdateResponse>, reqwest::Error> {
        let url = format!("{}/vehicles/telemetry/batch", self.base_url);

        let response = self.client.get(&url)
            .json(payload)
            .send()
            .await?
            .json::<Vec<TelemetryUpdateResponse>>()
            .await?;

        Ok(response)
    }

    // pub async fn update_telemetry(&self, registration_number: String, payload: TelemetryUpdateRequest
    // ) -> Result<TelemetryUpdateResponse ,reqwest::Error> {
    //     let url = format!("{}/vehicles/{}/telemetry", self.base_url, registration_number);

    //     let response = self.client.put(&url)
    //         .json(&payload)
    //         .send()
    //         .await?
    //         .json::<TelemetryUpdateResponse>()
    //         .await?;

    //     println!("updated telemetry response: {:#?}", response);

    //     Ok(response)
    // }

    pub async fn update_telemetry(&self, registration_number: String, payload: TelemetryUpdateRequest
    ) -> Result<() ,reqwest::Error> {
        let url = format!("{}/vehicles/{}/telemetry", self.base_url, registration_number);

        let response = self.client.put(&url)
            .json(&payload)
            .send()
            .await?
            // .json::<TelemetryUpdateResponse>()
            // .await?
            .error_for_status()?;

        println!("updated telemetry response: {:#?}", response);

        Ok(())
    }
 

}