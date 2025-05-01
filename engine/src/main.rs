mod order;

fn main() {
    let new_order = order::Order::new();
    println!("Order details: {} {} {}", new_order.side, new_order.type_op, new_order.limit_price);
}
