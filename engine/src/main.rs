use std::sync::{Arc, Mutex};

use axum::{routing::{delete, get, post}, Extension, Router};
use utils::{process_orders_from_file, read_orders_from_file, save_orderbook_state};
mod engine;
mod server;
mod utils;

#[tokio::main]
async fn main() {
    let mut ob = engine::orderbook::Orderbook::new();
    let file_orders = read_orders_from_file();
    process_orders_from_file(&mut ob, file_orders);
    save_orderbook_state(&ob);

    let shared_state = Arc::new(server::state::ServerState { orderbook: Mutex::new(ob) });

    let app = Router::new()
        .route("/", get(server::routes::get_orderbook))
        .route("/order", delete(server::routes::remove_order))
        .route("/trades", get(server::routes::get_trades))
        .route("/order", post(server::routes::new_order))
        .layer(Extension(shared_state));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();


    println!("Orderbook server on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
