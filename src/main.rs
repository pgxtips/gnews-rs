use env_logger;
use dotenv::dotenv;

#[tokio::main]
async fn main(){
    dotenv().ok();
    env_logger::init();
}

