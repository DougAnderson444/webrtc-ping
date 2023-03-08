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

`ListenerClosed` is a `TransportEvent`.
The report is likely coming from `ListenStream.close` function in `webrtc::tokio::Transport`
Not likely caused by `remove_listener` as we didn't call it. Swarm also has a `remove_listener`
`ListenStream` also has a `report_closed` Option, which is None when `ListenStream::new` is called in `transport.listen_on`, but set to Some on `.close` so we get our `TransportEvent::ListenerClosed` event.

The ListenStream is `.close`d is called when there is a UDPMux error:

```rs
// https://github.com/libp2p/rust-libp2p/blob/babf7e375310b651342e782880aadb65c620c00e/transports/webrtc/src/tokio/transport.rs#L351
Poll::Ready(UDPMuxEvent::Error(e)) => {
    self.close(Err(Error::UDPMux(e)));
    continue;
}
```

So the question becomes: Is closing the javascript client triggering the UDPMux error? And should we be closing the listener without checking `keep_alive` first (in case we want to listen for subsequent clients)?
