use crate::engine::amount::Amount;
use crate::engine::order::Order;
use crate::engine::price::Price;
use crate::engine::side::Side;
use crate::engine::trade::Trade;
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
    trades: Vec<Trade>,
    trade_count: u64,
}

impl Orderbook {
    pub fn new() -> Orderbook {
        Orderbook {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
            trades: Vec::new(),
            trade_count: 0,
        }
    }

    fn add_trade(
        &mut self,
        price: &Price,
        amount: &Amount,
        buy_order_id: u64,
        sell_order_id: u64,
        buy_account_id: u64,
        sell_account_id: u64
    ) {
        let trade_id_one = self.trade_count;
        let trade_id_two = self.trade_count + 1;
        self.trade_count += 2;
        let buy_trade = Trade::new(price.clone(), amount.clone(), trade_id_one, buy_order_id, sell_order_id, buy_account_id, Side::BUY);
        let sell_trade = Trade::new(price.clone(), amount.clone(), trade_id_two, buy_order_id, sell_order_id, sell_account_id, Side::SELL);

        self.trades.push(buy_trade);
        self.trades.push(sell_trade);
    }


    fn on_new_buy(&mut self, order: &mut Order) {
        while let Some((price, mut asks_queue)) = self.asks.pop_first() {
            if price > order.limit_price {
                self.asks.insert(price, asks_queue);
                break;
            }

            while let Some(mut ask_order) = asks_queue.pop_front() {
                let trade_amnt = order.amount.min(ask_order.amount);

                self.add_trade(&price, &trade_amnt, order.order_id, ask_order.order_id, order.order_id, ask_order.order_id);
                println!(
                    "Matching order price: {} amount: {}",
                    ask_order.limit_price, trade_amnt
                );

                order.amount = order.amount - trade_amnt;
                ask_order.amount = ask_order.amount - trade_amnt;

                if ask_order.amount > Amount(0) {
                    asks_queue.push_front(ask_order);
                }

                if order.amount == Amount(0) {
                    break;
                }
            }

            if asks_queue.is_empty() {
                self.asks.remove(&price);
            } else {
                self.asks.insert(price, asks_queue);
            }

            if order.amount == Amount(0) {
                return;
            }
        }

        if order.amount > Amount(0) {
            self.bids
                .entry(Reverse(order.limit_price.clone()))
                .or_default()
                .push_back(order.clone());
        }
    }

    fn on_new_sell(&mut self, order: &mut Order) {
        while let Some((price, mut bids_queue)) = self.bids.pop_first() {
            if price.0 > order.limit_price {
                self.bids.insert(price, bids_queue);
                break;
            }

            while let Some(mut bid_order) = bids_queue.pop_front() {
                let trade_amnt = order.amount.min(bid_order.amount);
                self.add_trade(&price.0, &trade_amnt, bid_order.order_id, order.order_id, bid_order.order_id, order.order_id);
                println!(
                    "Matching order price: {} amount: {}",
                    bid_order.limit_price, trade_amnt
                );

                order.amount = order.amount - trade_amnt;
                bid_order.amount = bid_order.amount - trade_amnt;

                if bid_order.amount > Amount(0) {
                    bids_queue.push_front(bid_order);
                }

                if order.amount == Amount(0) {
                    break;
                }
            }

            if bids_queue.is_empty() {
                self.bids.remove(&price);
            } else {
                self.bids.insert(price, bids_queue);
            }

            if order.amount == Amount(0) {
                return;
            }
        }

        if order.amount > Amount(0) {
            self.asks
                .entry(order.limit_price.clone())
                .or_default()
                .push_back(order.clone());
        }

    }

    pub fn on_new_order(&mut self, order: &mut Order) {
        if order.side == Side::BUY {
            self.on_new_buy(order);
        } else {
            self.on_new_sell(order);
        }
    }

    pub fn remove_order(&mut self, order: &Order) -> bool {
        let is_bid = order.side == Side::BUY;
        let mut removed = false;

        if is_bid {
            let reversed_price = Reverse(order.limit_price.clone());
            let optional_queue = self.bids.get_mut(&reversed_price);

            if optional_queue != None {
                let queue = optional_queue.unwrap();

                if let Some(index) = queue.iter().position(|x| x.order_id == order.order_id) {
                    queue.remove(index);
                    removed = true;
                }

                if queue.len() == 0 {
                    self.bids.remove(&reversed_price);
                }
            }
        } else {
            let price = order.limit_price.clone();
            let optional_queue = self.asks.get_mut(&price);

            if optional_queue != None {
                let queue = optional_queue.unwrap();

                if let Some(index) = queue.iter().position(|x| x.order_id == order.order_id) {
                    queue.remove(index);
                    removed = true;
                }

                if queue.len() == 0 {
                    self.asks.remove(&price);
                }
            }
        }

        println!("Removed order {} price: {} amount: {}", order.order_id, order.limit_price, order.amount);
        return removed;
    }

    pub fn save_json(&self) {
        let orderbook_json = serde_json::to_string(&self).unwrap();
        let trades_json = serde_json::to_string(&self.trades).unwrap();
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let orderbook_filename = format!("orderbook_{}.json", timestamp);
        let trades_filename = format!("trades_{}.json", timestamp);
        let mut trades_file = File::create(&trades_filename).unwrap();
        let mut orderbook_file = File::create(&orderbook_filename).unwrap();
        writeln!(orderbook_file, "{}", orderbook_json).unwrap();
        writeln!(trades_file, "{}", trades_json).unwrap();
    }

    pub fn get_trades_json(&self) -> String {
        serde_json::to_string(&self.trades).unwrap()
    }
}

impl Serialize for Orderbook {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let asks_mapper = |val: (&Price, &VecDeque<Order>)| {
            let price_serialized = val.0;

            let orders: Vec<Amount> = val.1.iter().map(|order| order.amount.clone()).collect();

            (price_serialized.clone(), orders)
        };

        let bids_mapper = |val: (&Reverse<Price>, &VecDeque<Order>)| {
            let price_serialized = val.0.0;

            let orders: Vec<Amount> = val.1.iter().map(|order| order.amount.clone()).collect();

            (price_serialized.clone(), orders)
        };

        let ask_list: Vec<(Price, Vec<Amount>)> = self.asks.iter().map(asks_mapper).collect();
        let bid_list: Vec<(Price, Vec<Amount>)> = self.bids.iter().map(bids_mapper).collect();
        let mut state = serializer.serialize_struct("Result", 2).unwrap();
        state.serialize_field("asks", &ask_list).unwrap();
        state.serialize_field("bids", &bid_list).unwrap();
        state.end()
    }
}
