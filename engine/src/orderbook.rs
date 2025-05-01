use crate::amount::Amount;
use crate::order::Order;
use crate::price::Price;
use crate::side::Side;
use serde::Serialize;
use serde::ser::SerializeStruct;
use std::cmp::Reverse;
use std::collections::{BTreeMap, VecDeque};
use std::fs::File;

use chrono::Local;
use std::io::Write;

pub struct Orderbook {
    asks: BTreeMap<Price, VecDeque<Order>>,
    bids: BTreeMap<Reverse<Price>, VecDeque<Order>>,
    asks_cache: BTreeMap<Price, Amount>,
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
        let order_amount = order.amount.clone();
        let is_bid = order.side == Side::BUY;

        if is_bid {
            let reversed_price = Reverse(order.limit_price.clone());
            let cache_amount = match self.bids_cache.get(&reversed_price) {
                None => Amount(0),
                Some(amount) => amount.clone() - order_amount,
            };

            if cache_amount == Amount(0) {
                self.bids_cache.remove(&reversed_price);
            } else {
                self.bids_cache.insert(reversed_price, cache_amount);
            }

            let optional_queue = self.bids.get_mut(&reversed_price);

            if optional_queue != None {
                let queue = optional_queue.unwrap();

                if let Some(index) = queue.iter().position(|x| x.order_id == order.order_id) {
                    queue.remove(index);
                }

                if queue.len() == 0 {
                    self.bids.remove(&reversed_price);
                }

            }
        } else {
            let price = order.limit_price.clone();

            let cache_amount = match self.asks_cache.get(&price) {
                None => Amount(0),
                Some(amount) => amount.clone() - order_amount,
            };

            if cache_amount == Amount(0) {
                self.asks_cache.remove(&price);
            } else {
                self.asks_cache.insert(price, cache_amount);
            }

            let optional_queue = self.asks.get_mut(&price);

            if optional_queue != None {
                let queue = optional_queue.unwrap();

                if let Some(index) = queue.iter().position(|x| x.order_id == order.order_id) {
                    queue.remove(index);
                }

                if queue.len() == 0 {
                    self.asks.remove(&price);
                }

            }

        }
    }

    fn update_cache(&mut self, price: Price, amount: Amount, is_bid: bool) {
        if is_bid {
            let reversed = Reverse(price.clone());
            let new_amount = match self.bids_cache.get(&reversed) {
                Some(_amount) => *_amount + amount,
                None => amount
            };

            self.bids_cache.insert(reversed, new_amount);
        } else {
            let price = price.clone();
            let new_amount = match self.asks_cache.get(&price) {
                Some(_amount) => *_amount + amount,
                None => amount
            };

            self.asks_cache.insert(price, new_amount);
        }

    }

    pub fn new_order(&mut self, order: Order) {
        let quote = order.limit_price;
        let amount = order.amount.clone();
        let price = order.limit_price.clone();
        self.update_cache(price, amount, order.side == Side::BUY);

        match order.side {
            Side::BUY => {
                let presence = self.bids.get_mut(&Reverse(price));

                match presence {
                    Some(quotes) => {
                        quotes.push_back(order);
                    }
                    None => {
                        let mut pr_queue = VecDeque::new();
                        pr_queue.push_back(order);
                        self.bids.insert(Reverse(price), pr_queue);
                    }
                }
            }
            Side::SELL => {
                let presence = self.asks.get_mut(&quote);

                match presence {
                    Some(quotes) => {
                        quotes.push_back(order);
                    }
                    None => {
                        let mut pr_queue = VecDeque::new();
                        pr_queue.push_back(order);
                        self.asks.insert(quote, pr_queue);
                    }
                }
            }
        };
    }

    pub fn peek_ask(&self) -> Option<(Price, Amount)> {
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
        println!(
            "Bids: {} Asks: {}",
            &self.bids_cache.len(),
            &self.asks_cache.len()
        );
    }
}

impl Serialize for Orderbook {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let asks_mapper = |val: (&Price, &Amount)| {
            let price_serialized = val.0;
            let amount_serialized = val.1;

            (price_serialized.clone(), amount_serialized.clone())
        };

        let bids_mapper = |val: (&Reverse<Price>, &Amount)| {
            let price_serialized = val.0.0;
            let amount_serialized = val.1;

            (price_serialized.clone(), amount_serialized.clone())
        };

        let ask_list: Vec<(Price, Amount)> = self.asks_cache.iter().map(asks_mapper).collect();
        let bid_list: Vec<(Price, Amount)> = self.bids_cache.iter().map(bids_mapper).collect();
        let mut state = serializer.serialize_struct("Result", 2).unwrap();
        state.serialize_field("asks", &ask_list).unwrap();
        state.serialize_field("bids", &bid_list).unwrap();
        state.end()
    }
}
