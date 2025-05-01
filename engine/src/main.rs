use std::{fs::File, path::Path};

mod order;

fn main() {
    let json_file_path = Path::new("./orders.json");
    let file = File::open(json_file_path).expect("Error: file could not be opened");

    let orders: Vec<order::Order> = serde_json::from_reader(file).expect("Error, json parsing issue");
    // let orders: Vec<order::Order> = 
    // let file = File::open(json_file_path)

    orders.iter().for_each(
        |x| {
            println!("{}", x.pair)
        }
    );

    let new_order = order::Order::new();
    println!("Order details: {} {} {}", new_order.side, new_order.type_op, new_order.limit_price);
}
