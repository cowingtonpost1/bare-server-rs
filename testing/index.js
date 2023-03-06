import BareClient from "@tomphttp/bare-client";

let client = new BareClient("http://127.0.0.1:8080");

setTimeout(async () => {
  // only now will the BareClient request the manifest
  const response = await client.fetch("http://www.google.com/");

  console.log(response.status);

  // 2nd call will reuse the first manifest
  await client.fetch("https://www.google.com/");
}, 1000);
