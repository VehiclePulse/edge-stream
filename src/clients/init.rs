use reqwest::Client;
use std::time::Duration;

pub fn init() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .pool_max_idle_per_host(10)
        .build()
        .expect("Failed to build global HTTP client")
}