import {
    webrtc_on_message,
    webrtc_on_connect,
    webrtc_on_signal
} from '/node_modules/@logankeenan/shared-space-app/shared_space_app.js';

function setup() {
    window.simplePeerAdaper = {
        peers: {},
        create: function (initiator = false, device_id) {
            let peer = new SimplePeer({
                initiator: location.hash === '#1',
                trickle: false
            });
            var connectionPromiseResolve;
            var connectionPromise = new Promise((resolve) => {
                connectionPromiseResolve = resolve;
            });
            this.peers[device_id] = {
                connection: peer,
                connectionPromise,
                connectionPromiseResolve
            };
            peer.on('error', (error) => {
                console.log('error:', error);
            });

            peer.on('signal', data => {
                console.log('SIGNAL', JSON.stringify(data))
                webrtc_on_signal(JSON.stringify({
                    from: device_id,
                    data
                }));
            })

            peer.on('connect', () => {
                connectionPromiseResolve();

                webrtc_on_connect(JSON.stringify({
                    from: device_id
                }));
            });

            peer.on('data', data => {
                webrtc_on_message(JSON.stringify({
                    from: device_id,
                    data
                }));
            });
        },
        sendMessage: async (message, device_id) => {
            let connection = this.peers[device_id];
            await connection.connectionPromise;

            connection.send(message);
        }
    }
}

setup();