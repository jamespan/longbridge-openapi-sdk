const { HttpClient } = require("longbridge");

let http_cli = HttpClient.fromEnv();
http_cli
  .request("get", "/v1/trade/execution/today")
  .then((resp) => console.log(resp));
