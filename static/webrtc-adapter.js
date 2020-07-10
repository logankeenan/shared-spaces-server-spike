import {
    webrtc_on_message,
    webrtc_on_connect,
    webrtc_on_signal
} from '/node_modules/@logankeenan/shared-space-app/shared_space_app.js';

function setup() {

    window.simplePeerAdapter = {
        peers: {},
        activeRequestsResolver: {},
        createSimplePeer: async function (initiator, device_id, offer) {
            console.log('initiator:', initiator);
            return new Promise((resolve) => {
                let peer = new SimplePeer({
                    initiator: initiator === "true"
                });
                var connectionPromiseResolve;
                var connectionPromise = new Promise((resolve) => {
                    connectionPromiseResolve = resolve;
                });

                console.log('offer:', offer);
                if (offer !== "") {
                    let offerAndIce = JSON.parse(offer);
                    offerAndIce.forEach(function (data) {
                        peer.signal(data)
                    });
                }
                var signalData = [];
                window.simplePeerAdapter.peers[device_id] = {
                    connection: peer,
                    connectionPromise,
                    connectionPromiseResolve,
                    signalData,
                };

                peer.on('error', (error) => {
                    console.log('error:', error);
                });

                peer.on('close', () => {
                    console.log('webrtc connection closed');
                    window.simplePeerAdapter[device_id] = undefined
                })

                peer.on('signal', data => {
                    signalData.push(data)
                })

                peer.on('connect', () => {
                    console.log('webrtc connected!!!');
                    connectionPromiseResolve();

                    webrtc_on_connect(device_id);
                });

                peer.on('data', data => {
                    let parsedData = JSON.parse(data);
                    const resolver = window.activeRequestsResolver[parsedData];
                    resolver(parsedData.message);

                    delete window.activeRequestsResolver[parsedData];
                });

                setTimeout(function () {
                    resolve(JSON.stringify(signalData));
                }, 3000);
            });
        },
        signalToSimplePeer: (datas, device_id) => {
            let peer = window.simplePeerAdapter.peers[device_id];

            JSON.parse(datas).forEach((data) => {
                peer.connection.signal(data);
            });
        },
        sendSimplePeerMessage: async (message, device_id) => {
            let peer = window.simplePeerAdapter.peers[device_id];

            await peer.connectionPromise;
            let request_id = uuidv4();

            return new Promise((resolve) => {

                peer.activeRequestsResolver[request_id] = resolve;

                peer.connection.send(JSON.stringify({
                    request_id,
                    message
                }));
            });
        }
    }
}

setup();