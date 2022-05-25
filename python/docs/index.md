# Longbridge OpenAPI SDK for Python

`longbridge` provides an easy-to-use interface for invokes [`Longbridge OpenAPI`](https://open.longbridgeapp.com/en/).

## Quickstart

_Install Longbridge OpenAPI SDK_

```bash
pip install longbridge
```

_Setting environment variables(MacOS/Linux)_

```bash
export LONGBRIDGE_APP_KEY="App Key get from user center"
export LONGBRIDGE_APP_SECRET="App Secret get from user center"
export LONGBRIDGE_ACCESS_TOKEN="Access Token get from user center"
```

_Setting environment variables(Windows)_

```powershell
setx LONGBRIDGE_APP_KEY "App Key get from user center"
setx LONGBRIDGE_APP_SECRET "App Secret get from user center"
setx LONGBRIDGE_ACCESS_TOKEN "Access Token get from user center"
```

## Quote API _(Get basic information of securities)_

```python
from longbridge import Config
from longbridge.quote import QuoteContext

# Load configuration from environment variables
config = Config.from_env()

# Create a context for quote APIs
ctx = QuoteContext(config)

# Get basic information of securities
resp = ctx.quote(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"])
print(resp)
```

## Quote API _(Subscribe quotes)_

```python
from time import sleep
from longbridge import Config
from longbridge.quote import QuoteContext, SubType, PushQuote

# Load configuration from environment variables
config = Config.from_env()


class EventHandler:
    """
    An event handler to receive push events
    """

    def on_event(self, symbol: str, msg):
        """
        Handle push events
        """
        if isinstance(msg, PushQuote):
            print(symbol, msg)


# Create a context for quote APIs
ctx = QuoteContext(config, EventHandler())

# Subscribe
resp = ctx.subscribe(["700.HK"], [SubType.Quote], is_first_push=True)

# Receive push duration to 30 seconds
sleep(30)
```

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license [LICENSE-MIT](http://opensource.org/licenses/MIT) at your option.