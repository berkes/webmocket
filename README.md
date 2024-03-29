# WebMocket

Websocket server for (integration)testing and development of a websocket client.

* Send custom messages to a client.
* Record messages sent by client.

This server is controlled with HTTP-requests. The client being tested, will
connect to it with a websocket and then we can interact with this client by
sending HTTP requests at an API. That same API can be used to check what
messages this client has sent to the server.

I needed this for e2e and integration testing a bot that listened on
a websocket.

[![Build Status](https://github.com/berkes/webmocket/actions/workflows/rust.yml/badge.svg)](https://github.com/berkes/webmocket/actions)
[![Crates.io](https://img.shields.io/crates/d/webmocket)](https://crates.io/crates/webmocket)

## webmocket

1. Start the webmocket webserver.
    ```bash
    webmocket # or "cargo run"
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

3. Check messages received from client:
    ```bash
    curl http://127.0.0.1:3000/messages | jq
    # [
    #   "hello, this is the first message",
    #   "my name is Foo and I am a Bar",
    #   "{ \"message\": \"this would be JSON\" }"
    # ]
    ```

4. Fire websocket messages to the client using a normal HTTP library. e.g. `curl`.
    ```
    wscat --connect http://127.0.0.1:3000/ws
    curl -X POST -H"Content-type: text/plain; charset=UTF-8" \
      --data 'Hello from the server 👋' \
      http://127.0.0.1:3000/messages
    ```
    We should see wscat print the message to the console.

6. Check ping/pong
    ```
    wscat --connect http://127.0.0.1:3000/ws --show-ping-pong
    curl -X POST -H"Content-type: text/plain; charset=UTF-8" \
      --data 'Hello from the server 👋' \
      http://127.0.0.1:3000/ping
    curl http://127.0.0.1:3000/messages
    ```
    We should see wscat replying to the Ping from the server with a Pong.
    We can then check that the server recieved a pong.

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
WEBMOCKET_ADDR=127.0.0.2 WEBMOCKET_PORT=8080 WEBMOCKET_WS_PATH="/messages/user" cargo run
```

### Docker

Alternative is to run webmocket in a container. Usefull in e.g. a CI pipeline,
or when you don't want or don't have cargo around.

`docker run --rm --detach --expose 3000:3000 berkes/wemocket:latest`

Environment variables as described under *Configure* can be passed in to
configure the service. For example:

`docker run --rm --detach --expose 3000:3000 --env WEBMOCKET_WS_PATH=/chat/stream`

This will run the webmocket service with the websocket endpoint on
http://0.0.0.0:3000/chat/stream.

Note: The port 3000 is hardcoded in the image so changing the `WEBMOCKET_PORT`
to anything other than 3000 won't work.

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

* [ ] Make it a library to include in tests rather than running it as standalone service.
* [ ] Proxy to an actual websocket and record messages as they would be sent server to client.
* [ ] Proxy to an actual websocket and record messages as they would be sent client to server.
* [ ] Store these recorded messages [VCR](https://github.com/vcr/vcr) style (though not nessecarily that format).
* [ ] Send those recorded server-to-client messages to a client on-demand.
* [ ] Set expectations on recorded client-to-server messages, to match that client sends what is expected.
