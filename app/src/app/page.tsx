import OrderBook from "@/components/orderbook";
import Image from "next/image";

export default async function Home() {
  let orderbook = null;

  try {
    const data = await fetch("http://localhost:3001/api/orderbook");
    orderbook = await data.json();    
  } catch (error) {
    console.error(error)
  }


  return (
    <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <main className="flex flex-col gap-[32px] row-start-2 items-center sm:items-start">
        {orderbook && <OrderBook orderbook={orderbook}></OrderBook>}
        {!orderbook && <p>Please enable the engine server!</p>}
      </main>
      <footer className="row-start-3 flex gap-[24px] flex-wrap items-center justify-center">
        <a
          className="flex items-center gap-2 hover:underline hover:underline-offset-4"
          href="https://saadahmsiddiqui.github.io/saadahm.siddiqui/"
          target="_blank"
          rel="noopener noreferrer"
        >
          <Image
            aria-hidden
            src="/file.svg"
            alt="File icon"
            width={16}
            height={16}
          />
          Find my details here
        </a>
      </footer>
    </div>
  );
}
