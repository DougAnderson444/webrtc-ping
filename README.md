# Libp2p WebRTC Server Ping

1. Start the server: `cargo run --package ping-server`
2. In a new termial, start the web client:

```cli
$ cd webclient
$ npm run dev
```

Open web browser to [http://localhost:5173/](http://localhost:5173/).

Paste the `multiaddr` from the server console log into the browser, click connect.

Issue: When the browser disconnects, the server listener is closed, even though [`keep_alive`](https://docs.rs/libp2p/latest/libp2p/swarm/keep_alive/struct.Behaviour.html) is enabled.

```
Event: ListenerClosed { listener_id: ListenerId(16861393669339383903), addresses: ["/ip6/::1/udp/42069/webrtc/certhash/uEiDHVx-evVihAOvIsWCX0Za1fcwbaHMBZKPbkyeUodTV2A"], reason: Err(Custom { kind: Other, error: UDPMux(Os { code: 10054, kind: ConnectionReset, message: "An existing connection was forcibly closed by
the remote host." }) }) }
```
