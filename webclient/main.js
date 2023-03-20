// https://github.com/libp2p/js-libp2p-webrtc/tree/main/examples/browser-to-server
import { createLibp2p } from "libp2p"
import { noise } from "@chainsafe/libp2p-noise"
import { multiaddr } from "@multiformats/multiaddr"
import { pipe } from "it-pipe"
import { fromString, toString } from "uint8arrays"
import { webRTC } from "@libp2p/webrtc"
import { pushable } from "it-pushable"

main()

async function main() {
    let stream
    let pingIntervalID

    const output = document.getElementById("output")
    const sendSection = document.getElementById("send-section")
    const appendOutput = (line) => {
        const div = document.createElement("div")
        div.appendChild(document.createTextNode(line))
        output.append(div)
    }
    const clean = (line) => line.replaceAll("\n", "")
    const sender = pushable()

    const libp2p = await createLibp2p({
        transports: [webRTC()],
        connectionEncryption: [noise()],
    })

    await libp2p.start()

    libp2p.connectionManager.addEventListener("peer:connect", (connection) => {
        appendOutput(
            `Peer connected '${libp2p
                .getConnections()
                .map((c) => c.remoteAddr.toString())}'`
        )
        sendSection.style.display = "block"
    })

    window.connect.onclick = async () => {
        const ma = multiaddr(window.peer.value)
        appendOutput(`Dialing '${ma}'`)
        stream = await libp2p.dialProtocol(ma, ["/ipfs/ping/1.0.0"]) // , "/floodsub/1.0.0"
        pipe(sender, stream, async (src) => {
            for await (const buf of src) {
                const response = toString(buf.subarray())
                appendOutput(`Received message '${clean(response)}'`)
            }
        })

        // also ping the Server
        const doPing = async () => {
            const latency = await libp2p.ping(ma)
            console.log({ latency })
        }
        doPing()
        pingIntervalID = setInterval(doPing, 5000)
    }

    window.send.onclick = async () => {
        const message = `${window.message.value}\n`
        appendOutput(`Sending message '${clean(message)}'`)
        sender.push(fromString(message))
    }
}
