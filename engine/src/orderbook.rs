use crate::order::Order;
use crate::price::Price;
use crate::side::Side;
use priority_queue::PriorityQueue;
use std::collections::BTreeMap;

pub struct OrderbookSize {
    pub asks: usize,
    pub bids: usize,
}

pub struct Orderbook {
    asks: BTreeMap<Price, PriorityQueue<Box<Order>, u64>>,
    bids: BTreeMap<Price, PriorityQueue<Box<Order>, u64>>,
}

impl<'a> Orderbook {
    pub fn new() -> Orderbook {
        Orderbook {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
        }
    }

    pub fn new_order(&mut self, order: Order) {
        let quote = order.limit_price;
        let order_id = order.order_id.clone();

        match order.side {
            Side::BUY => {
                let presence = self.bids.get_mut(&quote);

                match presence {
                    Some(quotes) => {
                        println!("Again Buy for price: {}", order.limit_price.clone());
                        quotes.push(Box::new(order), order_id);
                    }
                    None => {
                        println!("Buy for price: {}", order.limit_price.clone());
                        let mut pr_queue = PriorityQueue::new();
                        pr_queue.push(Box::new(order), order_id);
                        self.bids.insert(quote, pr_queue);
                    }
                }
            }
            Side::SELL => {
                let presence = self.asks.get_mut(&quote);

                match presence {
                    Some(quotes) => {
                        println!("Again Sell for price: {}", order.limit_price.clone());
                        quotes.push(Box::new(order), order_id);
                    }
                    None => {
                        println!("Sell for price: {}", order.limit_price.clone());
                        let mut pr_queue = PriorityQueue::new();
                        pr_queue.push(Box::new(order), order_id);
                        self.asks.insert(quote, pr_queue);
                    }
                }
            }
        };
    }

    pub fn orderbook_size(self) -> OrderbookSize {
        let asks: usize = self.asks.len();
        let bids: usize = self.bids.len();

        OrderbookSize { asks, bids }
    }
}
