use crate::amount::Amount;
use crate::main;
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

    pub fn remove_order(&mut self, order: &Order) {
        let order_price = Reverse(order.limit_price.clone());
        let order_amount = order.amount.clone();
        let is_bid = order.side == Side::BUY;

        let cache = match is_bid {
            true => &mut self.bids_cache,
            false => &mut self.asks_cache
        };

        let order_map = match is_bid {
            true => &mut self.bids,
            false => &mut self.asks
        };

        let cache_amount = match cache.get(&order_price) {
            None => Amount(0),
            Some(amount) => amount.clone() - order_amount
        };

        if cache_amount == Amount(0) {
            cache.remove(&order_price);
        } else {
            cache.insert(order_price, cache_amount);
        }


        let optional_queue = order_map.get_mut(&order.limit_price.clone());
        if optional_queue != None {
            let queue = optional_queue.unwrap();
            queue.remove(order);

            if queue.len() == 0 {
                order_map.remove(&order.limit_price.clone());
            }
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

    pub fn save_json(&self) {
        let json = serde_json::to_string(&self).unwrap();
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = format!("orderbook_{}.json", timestamp);
        let mut file = File::create(&filename).unwrap();
        writeln!(file, "{}", json).unwrap();
    }

    pub fn print_len(&self) {
        println!("Bids: {} Asks: {}", &self.bids_cache.len(), &self.asks_cache.len());
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
