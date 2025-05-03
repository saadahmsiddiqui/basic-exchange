import { Orderbook } from "@/types";
import React from "react";


type OrderRowProps = {
  price: number;
  amount: number;
  type: "BUY" | "SELL";
};

const OrderRow = ({ price, amount, type }: OrderRowProps) => (
  <div
    className={`flex justify-between px-4 py-2 text-sm ${
      type === "BUY" ? "text-green-500" : "text-red-500"
    }`}
  >
    <span>{price.toFixed(2)}</span>
    <span>{amount}</span>
  </div>
);


type OrderbookProps = { orderbook: Orderbook }

const OrderBook = ({ orderbook }: OrderbookProps) => {

  return (
    <div className="w-[500px] max-w-xl mx-auto p-4 bg-zinc-900 text-white rounded-2xl shadow-lg">
      <h2 className="text-xl font-bold mb-4 text-center">Order Book</h2>
      <div className="grid grid-cols-2 gap-2">
        <div>
          <h3 className="text-center text-green-400 mb-2">Bids</h3>
          <div className="space-y-1">
            {orderbook.bids.slice(0, 10).map((order, idx) => (
              <OrderRow
                key={idx}
                price={Number(order[0])}
                amount={order[1].map(i => Number(i)).reduce((agg, curr) => agg + curr,0)}
                type="BUY"
              />
            ))}
          </div>
        </div>
        <div>
          <h3 className="text-center text-red-400 mb-2">Asks</h3>
          <div className="space-y-1">
            {orderbook.asks.slice(0, 10).map((order, idx) => (
              <OrderRow
                key={idx}
                price={Number(order[0])}
                amount={order[1].map(i => Number(i)).reduce((agg, curr) => agg + curr,0)}
                type="SELL"
              />
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};

export default OrderBook;
