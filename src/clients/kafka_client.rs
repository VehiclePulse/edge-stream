use rdkafka::{
    config::ClientConfig,
    producer::FutureProducer
};

pub fn init_producer(bootstrap_servers: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", bootstrap_servers)
        .create()
        .expect("Failed to create kafaka producer")
}