use std::sync::Arc;
use axum::{Extension, Json};
use crate::{engine::{operation_type::OperationType, order::Order}, server::state};

pub async fn get_orderbook(Extension(state): Extension<Arc<state::ServerState>>) -> String {
    serde_json::to_string(&state.orderbook).unwrap()
}

pub async fn remove_order(Extension(state): Extension<Arc<state::ServerState>>, Json(payload): Json<Order>) -> String {
    let mut orderbook = state.orderbook.lock().unwrap();
    let removal_status = orderbook.remove_order(&payload);
    state.orderbook.clear_poison();

    if payload.type_op != OperationType::DELETE {
        let message = String::from("Invalid operation");
        return message;
    }

    let message = match removal_status {
        true => format!("Removed order: {}", payload.order_id),
        false => format!("Order not found or invalid order id: {}", payload.order_id),
    };

    return message;
}

pub async fn get_trades(Extension(state): Extension<Arc<state::ServerState>>) -> String {
    let mut ob = &state.orderbook.lock().unwrap();
    let trades = ob.get_trades_json();
    state.orderbook.clear_poison();
    trades
}

pub async fn new_order(Extension(state): Extension<Arc<state::ServerState>>, Json(payload): Json<Order>) -> String {
    let mut orderbook = state.orderbook.lock().unwrap();

    if payload.type_op == OperationType::CREATE {
        orderbook.on_new_order(&mut payload.clone());
        state.orderbook.clear_poison();
        return String::from("Order created successfully");
    } else {
        return String::from("Invalid operation type");
    }
}