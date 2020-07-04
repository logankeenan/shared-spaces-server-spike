import {
    webrtc_on_message,
    webrtc_on_connect,
    webrtc_on_signal
} from '/node_modules/@logankeenan/shared-space-app/shared_space_app.js';

function setup() {
    window.simplePeerAdapter = {
        peers: {},
        createSimplePeer: async function (initiator = false, device_id, offer) {
            console.log('initiator:', initiator);
            console.log('device_id:', device_id);
            console.log('offer:', offer);

            return new Promise((resolve) => {
                let peer = new SimplePeer({
                    initiator,
                    trickle: false
                });
                var connectionPromiseResolve;
                var connectionPromise = new Promise((resolve) => {
                    connectionPromiseResolve = resolve;
                });

                if (offer !== "") {
                    peer.signal(JSON.parse(offer))
                }

                this.peers[device_id] = {
                    connection: peer,
                    connectionPromise,
                    connectionPromiseResolve
                };

                peer.on('error', (error) => {
                    console.log('error:', error);
                });

                peer.on('signal', data => {
                    console.log('webrtc signal');
                    console.log('data:', data);
                    resolve(JSON.stringify(data));
                })

                peer.on('connect', () => {
                    connectionPromiseResolve();
                    console.log('webrtc connected!!!');

                    webrtc_on_connect(JSON.stringify({
                        from: device_id
                    }));
                });

                peer.on('data', data => {
                    console.log('peer on data');
                    console.log('data:', data);
                    webrtc_on_message(JSON.stringify({
                        from: device_id,
                        data
                    }));
                });
            });
        },
        signalToSimplePeer: (data, device_id) => {
            let connection = this.peers[device_id];

            console.log('signalToSimplePeer');
            console.log('data:', data);
            connection.signal(JSON.parse(data));
        },
        sendSimplePeerMessage: async (message, device_id) => {
            let connection = this.peers[device_id];
            await connection.connectionPromise;

            connection.send(message);
        }
    }
}

setup();