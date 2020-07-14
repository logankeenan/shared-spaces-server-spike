import {
    webrtc_on_message,
    webrtc_on_connect,
    webrtc_on_signal,
    app,
    AppRequest
} from '/node_modules/@logankeenan/shared-space-app/shared_space_app.js';

function setup() {

    window.simplePeerAdapter = {
        peers: {},
        activeRequestsResolver: {},
        createSimplePeer: async function (initiator, device_id, offer) {
            return new Promise((resolve) => {
                let peer = new SimplePeer({
                    initiator: initiator === "true"
                });
                var connectionPromiseResolve;
                var connectionPromise = new Promise((resolve) => {
                    connectionPromiseResolve = resolve;
                });

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
                });

                peer.on('close', () => {
                    window.simplePeerAdapter[device_id] = undefined
                })

                peer.on('signal', data => {
                    signalData.push(data)
                })

                peer.on('connect', () => {
                    connectionPromiseResolve();

                    webrtc_on_connect(device_id);
                });

                peer.on('data', async data => {

                    let parsedData = JSON.parse(data);

                    if (parsedData.type === "request") {
                        var request = JSON.parse(parsedData.message);

                        let app_request = new AppRequest(request.path, request.method);
                        app_request.body = request.body;

                        let response = await app(app_request);
                        // TODAY - this needs to be a json message so I can send it

                        let chunk = JSON.stringify({
                            type: 'response',
                            request_id: parsedData.request_id,
                            message: response.as_json_string()
                        });
                        peer.send(chunk);

                    } else if (parsedData.type === "response") {
                        const resolver = window.simplePeerAdapter.activeRequestsResolver[parsedData.request_id];
                        resolver(parsedData.message);

                        delete window.simplePeerAdapter.activeRequestsResolver[parsedData.request_id];
                    }
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
        sendSimplePeerMessage: function (message, device_id) {
            let promise = new Promise((resolve) => {
                let peer = window.simplePeerAdapter.peers[device_id];
                let request_id = uuidv4();

                window.simplePeerAdapter.activeRequestsResolver[request_id] = resolve;
                let chunk = JSON.stringify({
                    type: 'request',
                    request_id,
                    device_id,
                    message
                });
                peer.connection.send(chunk);
            });

            return promise;
        }
    }
}

setup();