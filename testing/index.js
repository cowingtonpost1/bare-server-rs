import BareClient from "@tomphttp/bare-client";

let client = new BareClient("http://127.0.0.1:8080/bare/");

setTimeout(async () => {
  const response = await client.fetch("http://httpforever.com/");

  console.log(response.status);

}, 1000);
