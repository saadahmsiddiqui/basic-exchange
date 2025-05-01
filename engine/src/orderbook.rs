use crate::amount::Amount;
use crate::order::Order;
use crate::price::Price;
use crate::side::Side;
use priority_queue::PriorityQueue;
use serde::Serialize;
use serde::ser::SerializeStruct;
use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::fs::File;

use chrono::Local;
use std::io::Write;

pub struct OrderbookSize {
    pub asks: usize,
    pub bids: usize,
}

pub struct Orderbook {
    asks: BTreeMap<Price, PriorityQueue<Box<Order>, u64>>,
    bids: BTreeMap<Price, PriorityQueue<Box<Order>, u64>>,
    asks_cache: BTreeMap<Reverse<Price>, Amount>,
    bids_cache: BTreeMap<Reverse<Price>, Amount>,
}

impl<'a> Orderbook {
    pub fn new() -> Orderbook {
        Orderbook {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
            asks_cache: BTreeMap::new(),
            bids_cache: BTreeMap::new(),
        }
    }

    fn update_cache(&mut self, price: Price, amount: Amount, is_bid: bool) {
        let map = match is_bid {
            true => &mut self.bids_cache,
            false => &mut self.asks_cache,
        };

        let reversed = Reverse(price.clone());
        let amount = match map.get(&reversed) {
            Some(_amount) => *_amount + amount,
            _ => amount,
        };

        map.insert(Reverse(price), amount);
    }

    pub fn new_order(&mut self, order: Order) {
        let quote = order.limit_price;
        let order_id = order.order_id.clone();
        let amount = order.amount.clone();
        let price = order.limit_price.clone();

        self.update_cache(price, amount, order.side == Side::BUY);

        match order.side {
            Side::BUY => {
                let presence = self.bids.get_mut(&quote);

                match presence {
                    Some(quotes) => {
                        quotes.push(Box::new(order), order_id);
                    }
                    None => {
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
                        quotes.push(Box::new(order), order_id);
                    }
                    None => {
                        let mut pr_queue = PriorityQueue::new();
                        pr_queue.push(Box::new(order), order_id);
                        self.asks.insert(quote, pr_queue);
                    }
                }
            }
        };
    }

    pub fn peek_ask(&self) -> Option<(Reverse<Price>, Amount)> {
        let first_val = self.asks_cache.first_key_value();

        let to_return = match first_val {
            None => None,
            Some((price, amount)) => Some((price.clone(), amount.clone())),
        };

        to_return
    }

    pub fn peek_bid(&self) -> Option<(Reverse<Price>, Amount)> {
        let first_val = self.bids_cache.first_key_value();

        let to_return = match first_val {
            None => None,
            Some((price, amount)) => Some((price.clone(), amount.clone())),
        };

        to_return
    }

    pub fn orderbook_size(&self) -> OrderbookSize {
        let asks: usize = self.asks.len();
        let bids: usize = self.bids.len();

        OrderbookSize { asks, bids }
    }

    // pub fn print_orderbook(&self) {
    //     let asks = &self.asks_cache;
    //     let bids = &self.bids_cache;

    //     asks.iter().for_each(|ask| {
    //         let price = &ask.0.0;
    //         print!("SELL {} {}\n", price, ask.1)
    //     });

    //     println!();

    //     bids.iter().for_each(|bid| {
    //         let price = &bid.0.0;
    //         print!("BUY {} {}\n", price, bid.1)
    //     });
    // }

    pub fn save_json(&self) {
        let json = serde_json::to_string(&self).unwrap();
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = format!("orderbook_{}.json", timestamp);
        let mut file = File::create(&filename).unwrap();
        writeln!(file, "{}", json).unwrap();
    }
}

impl Serialize for Orderbook {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let list_mapper = |val: (&Reverse<Price>, &Amount)| {
            let price_serialized = val.0.0;
            let amount_serialized = val.1;

            (price_serialized.clone(), amount_serialized.clone())
        };

        let ask_list: Vec<(Price, Amount)> = self.asks_cache.iter().map(list_mapper).collect();
        let bid_list: Vec<(Price, Amount)> = self.bids_cache.iter().map(list_mapper).collect();
        let mut state = serializer.serialize_struct("Result", 2).unwrap();
        state.serialize_field("asks", &ask_list).unwrap();
        state.serialize_field("bids", &bid_list).unwrap();
        state.end()
    }
}
