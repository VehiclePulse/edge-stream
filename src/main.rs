mod config;
mod clients;

use config::app_config;
use crate::config::model::{AppState, HttpClient};

#[tokio::main]
async fn main() {
    let configuration = app_config::load_config();
    let http_client = clients::init::init();


    print!("hi");
    
}
