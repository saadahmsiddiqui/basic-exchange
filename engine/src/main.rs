use std::{fs::File, path::Path};

use operation_type::OperationType;
mod price;
mod operation_type;
mod side;
mod order;
mod orderbook;
mod amount;

fn main() {
    let json_file_path = Path::new("./orders.json");
    let file = File::open(json_file_path).expect("Error: file could not be opened");
    let mut ob = orderbook::Orderbook::new();
    let orders: Vec<order::Order> = serde_json::from_reader(file).expect("Error, json parsing issue");

    println!("Total orders: {}", orders.len());
    let mut total_delete = 0;

    orders.iter().for_each(
        |x| {
            if x.type_op.eq(&OperationType::DELETE) {
                total_delete += 1;
            }

            ob.new_order(x.clone());
        }
    );

    println!("Delete orders {}", total_delete);
    let size = ob.orderbook_size();
    println!("Orderbook asks: {} Orderbook bids: {}", size.asks, size.bids);

}
