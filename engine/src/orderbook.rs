use std::collections::BTreeMap;
use crate::order::Order;
use crate::price::Price;
use crate::side::Side;

pub struct OrderbookSize {
    pub asks: usize,
    pub bids: usize
}

pub struct Orderbook {
    asks: BTreeMap<Price, Vec<Order>>,
    bids: BTreeMap<Price, Vec<Order>>
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

        match order.side {
            Side::BUY => {
                let presence = self.bids.get_mut(&quote);

                match presence {
                    Some(quotes) => {
                        quotes.push(order);
                    },
                    None => {
                        let mut new_vec = Vec::new();
                        new_vec.push(order);
                        self.bids.insert(quote, new_vec);
                    }
                }
            },
            Side::SELL => {
                let presence = self.asks.get_mut(&quote);

                match presence {
                    Some(quotes) => {
                        quotes.push(order);
                    },
                    None => {
                        let mut new_vec = Vec::new();
                        new_vec.push(order);
                        self.asks.insert(quote, new_vec);
                    }
                }

            }
        };
    }

    pub fn orderbook_size(self) -> OrderbookSize {
        let asks: usize = self.asks.len();
        let bids: usize = self.bids.len();

        OrderbookSize{
            asks,
            bids
        }
    }

}