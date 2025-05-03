use std::sync::Mutex;

use crate::engine::orderbook;

pub struct ServerState {
    pub orderbook: Mutex<orderbook::Orderbook>
}