use lapin::{
    Connection, ConnectionProperties, options::{BasicAckOptions, BasicConsumeOptions, BasicQosOptions, BasicRejectOptions}, types::{
        FieldTable, ShortString
    }
};
use std::sync::Arc;

use crate::config::model::{AppState};
use crate::services::rabbitmq_handler;

pub async fn run(
    app_state: Arc<AppState>
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = app_state.config.rabbitmq.clone();
    let connection = 
        Connection::connect(
            &config.url, 
            ConnectionProperties::default()
        ).await?;
    println!("connected to rabbitmq");

    let channel = connection.create_channel().await?;

    channel.basic_qos(
        config.prefetch, 
        BasicQosOptions::default()
    ).await?;

    let mut consumer = channel
        .basic_consume(
            ShortString::from(&*config.queue), 
            ShortString::from("edge-stream"), 
            BasicConsumeOptions::default(), 
            FieldTable::default()
        )
        .await?;
    println!("COnsuming messages from queue: {}", config.queue);

    loop {
        tokio::select! {
            _ = app_state.cancellation_token.cancelled() => {
                println!("RabbitMq worker stopping");
                break;
            }

            delivery = futures_util::StreamExt::next(&mut consumer) => {
                match delivery {
                    Some(Ok(delivery)) => {
                        let result = rabbitmq_handler::handle(
                            &delivery.routing_key.as_str(),
                            &delivery.data,
                            &app_state
                        )
                        .await;

                        match result {
                            Ok(()) => {
                                delivery
                                    .ack(BasicAckOptions::default())
                                    .await?;
                            }

                            Err(err) => {
                                eprintln!("Failed to process rabbitmq message: {err}");

                                delivery.reject(BasicRejectOptions {
                                        requeue: true,
                                        ..Default::default()
                                    })
                                    .await?;
                            }
                        }
                        
                    }

                    Some(Err(error)) => {
                        eprintln!("rabbitMq consumer error: {error}");
                        return Err(Box::new(error));
                    }

                    None => {
                        println!("RabbitMQ consumer stream ended");
                        break;
                    }
                }
            }
        }
    }
    

    Ok(())
}