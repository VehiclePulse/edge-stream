mod config;

use config::app_config;

#[tokio::main]
async fn main() {
    let app_state = app_config::load_config();

    println!("app state: {:#?}", app_state);
}
