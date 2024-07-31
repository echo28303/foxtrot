use actix_web::{App, HttpServer, web};
use std::sync::Mutex;
mod routes;
mod handlers;
use crate::core::blockchain::Blockchain;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let blockchain = web::Data::new(Mutex::new(Blockchain::new(
        "./db",
        10, // block_reward
        1,  // initial_difficulty
        1_000_000, // max_block_size
        8, // block_time (in seconds)
        2016, // difficulty_adjustment_window (number of blocks)
        String::from("testnet"),
        125_000_000_000_000, // transition_block
    )));

    HttpServer::new(move || {
        App::new()
            .app_data(blockchain.clone())
            .configure(routes::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

