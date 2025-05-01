use std::{fs::File, path::Path};

use operation_type::OperationType;

mod price;
mod operation_type;
mod side;
mod order;
mod constants;
mod orderbook;
mod amount;

fn main() {
    let mut ob = orderbook::Orderbook::new();
    // Opening JSON and reading the JSON
    let json_file_path = Path::new("./orders.json");
    let file = File::open(json_file_path).expect("Error: file could not be opened");
    let orders: Vec<order::Order> = serde_json::from_reader(file).expect("Error, json parsing issue");
    println!("Total orders: {}", orders.len());

    orders.iter().for_each(
        |x| {
            let ask = ob.peek_ask();
            let bid = ob.peek_bid();

            if x.type_op == OperationType::DELETE {
                ob.remove_order(x);
            } else {
                ob.new_order(x.clone());
            }

            println!("{} {} {} {}", &x.type_op, &x.side, &x.amount, &x.limit_price);

            match ask {
                None => {},
                Some(ask_tuple) => {
                    println!("OB SELL Price: {} Amount: {}", ask_tuple.0.0, ask_tuple.1);
                }
            }

            match bid {
                None => {},
                Some(bid_tuple) => {
                    println!("OB BUY Price: {} Amount: {}", bid_tuple.0.0, bid_tuple.1);
                }
            }

            ob.print_len();
            println!();
        }
    );

    ob.save_json();
}
