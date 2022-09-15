# WebMocket

Websocket server for testing and development.

* Send custom messages to a client.
* Record messages sent by client.

I needed this for e2e and integration testing a bot that listened on
a websocket.

## webmocket

1. Start the webmocket webserver.
    ```bash
    RUST_LOG=info cargo run
    # 2022-09-15T14:38:07Z INFO  webmocket] Listening on 127.0.0.1:3000
    ```

2. Connect a websocket client (webpage, bot, client, etc.). e.g. [wscat](https://github.com/websockets/wscat). Then send a few messages to the server.
    ```bash
    wscat --connect http://127.0.0.1:3000
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

### Test

Get the source-code, then

    cargo test

This builds the application and runs the tests locally.

### Release

TODO: CI/CD setup

### Deploy

TODO: CI/CD setup

## TODO

If needs arise, or pull-requests arrive, some additional features could be added:

* [ ] Proxy to an actual websocket and record messages as they would be sent server to client.
* [ ] Proxy to an actual websocket and record messages as they would be sent client to server.
* [ ] Store these recorded messages [VCR](https://github.com/vcr/vcr) style (though not nessecarily that format).
* [ ] Send those recorded server-to-client messages to a client on-demand.
* [ ] Set expectations on recorded client-to-server messages, to match that client sends what is expected.
