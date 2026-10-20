use std::time::Duration;

use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use vehicle_simulator::simulator::Simulator;
use vehicle_simulator::model::{InitialTelemetry, TelemetryMessage};

use uuid::Uuid;

pub struct BackgroundManager {
    pub cancellation_token: CancellationToken,
    pub simulator: Simulator,
    telemetry_receiver: Option<mpsc::Receiver<TelemetryMessage>>,
}

impl BackgroundManager {
    pub fn initialize() -> Self {
        
        let cancellation_token = CancellationToken::new();
        let (simulator, telemetry_receiver) = 
            Simulator::new(Duration::from_secs(1));
        println!("Simulator initialized");

        Self { 
            cancellation_token, 
            simulator, 
            telemetry_receiver: Some(telemetry_receiver) 
        }
    }

    pub async fn add_vehicle(
        &self,
        telemetry: InitialTelemetry
    ) -> Result<(), String> {
        self.simulator.start_vehicle(telemetry).await
    }

    pub async fn remove_vehicle(
        &self,
        vehicle_id: Uuid
    ) -> Result<(), String> {
        self.simulator.stop_vehicle(vehicle_id).await
    }

    pub async fn take_telemetry_receiver(
        &mut self
    ) -> Option<tokio::sync::mpsc::Receiver<vehicle_simulator::model::TelemetryMessage>> {
        self.telemetry_receiver.take()
    }
}