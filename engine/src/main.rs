use std::{fs::File, path::Path};

use operation_type::OperationType;

mod price;
mod operation_type;
mod side;
mod order;
mod constants;
mod orderbook;
mod amount;
mod trade;

fn main() {
    let mut ob = orderbook::Orderbook::new();
    // Opening JSON and reading the JSON
    let json_file_path = Path::new("./orders.json");
    let file = File::open(json_file_path).expect("Error: file could not be opened");
    let orders: Vec<order::Order> = serde_json::from_reader(file).expect("Error, json parsing issue");
    println!("Total orders: {}", orders.len());

    orders.iter().for_each(
        |x| {
            if x.type_op == OperationType::DELETE {
                ob.remove_order(x);
            } else {
                ob.on_new_order(&mut x.clone());
            }
        }
    );

    ob.save_json();
}
