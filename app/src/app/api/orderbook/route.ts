import { OB_ENGINE_ENDPOINT } from "@/app/constants";

const CORS_HEADERS = {
  headers: {
    "Access-Control-Allow-Origin": "*",
    "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE, OPTIONS",
    "Access-Control-Allow-Headers": "Content-Type, Authorization",
  },
};

export async function GET() {
  try {
    const fetchOrderbook = await fetch(`http://${OB_ENGINE_ENDPOINT}`);
    const bodyJson = await fetchOrderbook.json()

    return new Response(JSON.stringify(bodyJson), {
      status: 200,
      headers: CORS_HEADERS.headers,
    });
  } catch (error) {
    console.error(error);

    return new Response(
      JSON.stringify({
        error: "could not reach orderbook server",
      }),
      {
        status: 500,
      }
    );
  }
}
