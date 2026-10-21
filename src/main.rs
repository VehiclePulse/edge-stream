mod background;
mod config;
mod clients;
mod services;
use config::app_config;
use tokio_util::sync::CancellationToken;
use vehicle_simulator::simulator::Simulator;
use std::{sync::Arc, time::Duration};
use crate::{background::{rabbitmq_worker, shutdown::wait_for_shutdown_signal}, config::model::{AppState, HttpClient}, services::vehicle_handler};

#[tokio::main]
async fn main() {
    let cancellation_token = CancellationToken::new();

    let configuration = app_config::load_config();

    let vehicle_client = clients::init_client(&configuration);
    
    let (simulator, telemetry_receiver) = 
        Simulator::new(Duration::from_secs(1));


    let app_state = Arc::new(AppState {
        config: configuration,
        http_client: HttpClient { 
            vehicle_client: vehicle_client
        },
        simulator: Arc::new(simulator),
        cancellation_token: cancellation_token
    });

    //kafka
    let kafka_producer = clients::kafka_client::init_producer(
        &app_state.config.kafka.bootstrap_servers
    );

    let kafka_topic = app_state.config.kafka.topic.clone(); 

    //mpsc listener
    let telemetry_handle = tokio::spawn(async move {
        background::telemetry_worker::run(
                telemetry_receiver,
                kafka_producer,
                kafka_topic
            ).await;
       }
    );

    vehicle_handler::init(&app_state).await;

    //rabbit mq
    let rabbitmq_state = Arc::clone(&app_state);
    let rabbitmq_handle = tokio::spawn(async move {
        if let Err(error) = rabbitmq_worker::run(
            rabbitmq_state
        ).await {
            eprintln!("rabbitmq worker failed with fatal error: {error}");
        }
    });


    //current shutdown listener
    if let Err(error) = wait_for_shutdown_signal(app_state.cancellation_token.clone()).await {
        eprintln!("Failed to register the shutdown singal handlers: {error}");
    }

    if let Err(error) = rabbitmq_handle.await {
        eprintln!("rabbitmq worker task failed: {error}");
    }

    drop(app_state);

    if let Err(error) = telemetry_handle.await {
        eprintln!("Telemetry worker task failed: {error}");
    }    
}
