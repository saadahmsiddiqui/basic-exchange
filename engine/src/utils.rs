use crate::engine::operation_type::OperationType;
use std::{fs::File, path::Path};
use crate::engine::orderbook;
use crate::engine::order;

pub fn read_orders_from_file() -> Vec<order::Order> {
    let json_file_path = Path::new("./orders.json");
    let file = File::open(json_file_path).expect("Error: file could not be opened");
    println!("Reading orders from file: {}...", json_file_path.to_str().unwrap());
    serde_json::from_reader(file).unwrap()
}

pub fn process_orders_from_file(ob: &mut orderbook::Orderbook, orders: Vec<order::Order>) {
    println!("Processing orders read from the file...");
    orders.iter().for_each(|x| {
        if x.type_op == OperationType::DELETE {
            ob.remove_order(x);
        } else {
            ob.on_new_order(&mut x.clone());
        }
    });
}

pub fn save_orderbook_state(ob: &orderbook::Orderbook) {
    ob.save_json();
}