use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use tokio::sync::mpsc;
use vehicle_simulator::model::TelemetryMessage;

pub async fn run(
    mut receiver: mpsc::Receiver<TelemetryMessage>,
    producer: FutureProducer,
    topic: String
) {
    while let Some(message) = receiver.recv().await {
        let payload = match serde_json::to_vec(&message) {
            Ok(payload) => payload,
            Err(error) => {
                eprintln!("Failed to serialize telemetry: {error}");
                continue;
            }
        };

        let record = FutureRecord::<(), Vec<u8>>::to(&topic)
            .payload(&payload);

        match producer
            .send(record, Timeout::Never)
            .await 
        {
            Ok(delivery) => {
                println!("Telemetry delivered to kafka successfully: {delivery:?}");
            }
            Err((error, _)) => {
                eprintln!("kafka delivery failed: {error}");
            } 
        }
    }

    println!("Telemetry worker stopped");
}