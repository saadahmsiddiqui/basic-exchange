use axum::{Router, routing::get};
use utils::{process_orders_from_file, read_orders_from_file, save_orderbook_state};
mod amount;
mod constants;
mod operation_type;
mod order;
mod orderbook;
mod price;
mod side;
mod trade;
mod utils;

struct ServerState<'a> {
    orderbook: &'a mut orderbook::Orderbook
}

#[tokio::main]
async fn main() {
    let mut ob = orderbook::Orderbook::new();
    let file_orders = read_orders_from_file();
    process_orders_from_file(&mut ob, file_orders);
    save_orderbook_state(&ob);


    let app = Router::new().route("/", get(|| async { "Hello, world!" }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
