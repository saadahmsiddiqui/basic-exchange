use std::sync::Arc;

use axum::Extension;

use crate::state;

pub async fn get_orderbook(Extension(state): Extension<Arc<state::ServerState>>) -> String {
    serde_json::to_string(&state.orderbook).unwrap()
}
