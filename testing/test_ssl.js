import BareClient from "@tomphttp/bare-client";

let client = new BareClient("http://127.0.0.1:8080/bare/");

setTimeout(async () => {
  const response = await client.fetch("https://duckduckgo.com/");

  console.log(response.status, await response.text());

}, 1000);
