use tokio::sync::mpsc;
use vehicle_simulator::model::TelemetryMessage;

pub async fn run(
    mut receiver: mpsc::Receiver<TelemetryMessage>
) {
    while let Some(message) = receiver.recv().await {
        println!("Received telemetry: {:?}", message);
    }
}