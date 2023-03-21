# Libp2p WebRTC Server Ping

A simple Ping server to demonstrate a Rust Ping server connecting with a Javascript Web Client.

## Usage

1. Start the server: `cargo run --package ping-server`
2. In a new termial, start the web client:

```cli
$ cd webclient
$ npm run dev
```

Open web browser to [http://localhost:5173/](http://localhost:5173/).

Paste the `multiaddr` from the server console log into the browser, click connect.
