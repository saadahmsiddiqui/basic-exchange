use std::sync::Arc;

use axum::{routing::get, Extension, Router};
use utils::{process_orders_from_file, read_orders_from_file, save_orderbook_state};
mod trade;
mod utils;
mod server;
mod engine;
mod state;


#[tokio::main]
async fn main() {
    let mut ob = engine::orderbook::Orderbook::new();
    let file_orders = read_orders_from_file();
    process_orders_from_file(&mut ob, file_orders);
    save_orderbook_state(&ob);


    let shared_state = Arc::new(state::ServerState {
        orderbook: ob
    });

    let app = Router::new().route("/", get(server::routes::get_orderbook)).layer(Extension(shared_state));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
