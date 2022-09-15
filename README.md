# WebMocket

Websocket server for testing and development.

* Send custom messages to a client.
* Record messages sent by client.

I needed this for e2e and integration testing a bot that listened on
a websocket.

[![Build Status](https://cloud.drone.io/api/badges/berkes/webmocket/status.svg?ref=refs/heads/main)](https://cloud.drone.io/berkes/webmocket)

## webmocket

1. Start the webmocket webserver.
    ```bash
    RUST_LOG=info cargo run
    # 2022-09-15T14:38:07Z INFO  webmocket] Listening on 127.0.0.1:3000
    ```

2. Connect a websocket client (webpage, bot, client, etc.). e.g.
   [wscat](https://github.com/websockets/wscat). Then send a few messages to
   the server. *Typically this would be the subject under test*.
    ```bash
    wscat --connect http://127.0.0.1:3000/ws
    # Connected (press CTRL-C to quit)
    # > hello, this is the first message
    # > my name is Foo and I am a Bar
    # > { "message": "this would be JSON" }
    ```
    Leave this running for now.

3. Fire websocket messages to the client using a normal HTTP library. e.g. `curl`.
    ```
    curl -X POST -H"Content-type: application/json" \
      --data '{"content": "Hello from the server"}' \
      http://127.0.0.1:3000/messages
    ```
    We should see wscat print the message to the console.

4. Check messages received from client:
    ```bash
    curl http://127.0.0.1:3000/messages | jq
    # [
    #   "hello, this is the first message",
    #   "my name is Foo and I am a Bar",
    #   "{ \"message\": \"this would be JSON\" }"
    # ]
    ```

### Install

In order to install the platform on development machine, run

    cargo install webmocket

TODO: build binaries and make the avaiable for download.

### Run

    webmocket


### Configure

Webmocket is configured through env vars. These must be set before starting.

| var     |          | default |
|---------|----------|---------|
| `WEBMOCKET_ADDR` | IP address of the listening host. Must be a valid IPV4 | 127.0.0.1 |
| `WEBMOCKET_PORT` | Port on which the service will listen. Must be a valid unix port | 3000 |
| `WEBMOCKET_WS_PATH` | Path on which the websocket can be connected. Must start with / | /ws |

E.g. to set all variables, one could run:
```bash
WEBMOCKET_ADDR=127.0.0.2 WEBMOCKET_PORT=8080 WEBMOCKET_WS_PATH="/messages/user" RUST_LOG=info cargo run
```
### Test

Get the source-code, then

    cargo test

This builds the application and runs the tests locally.

### Release

TODO: CI/CD setup

### Deploy

TODO: CI/CD setup

## Limitations

The service is intended for local development or tests (A CI server or such):
it keeps its database in memory. This means that if you're going to send a lot
of data to it, it will need a lot of memory.

The service does not bind to HTTPS (or SSL), because that is hard to achieve.
if the client insists on SSL only, you may need to configure some proxy before
it.

## TODO

If needs arise, or pull-requests arrive, some additional features could be added:

* [ ] Proxy to an actual websocket and record messages as they would be sent server to client.
* [ ] Proxy to an actual websocket and record messages as they would be sent client to server.
* [ ] Store these recorded messages [VCR](https://github.com/vcr/vcr) style (though not nessecarily that format).
* [ ] Send those recorded server-to-client messages to a client on-demand.
* [ ] Set expectations on recorded client-to-server messages, to match that client sends what is expected.
