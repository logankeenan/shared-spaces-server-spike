import {app, AppRequest, webrtc_on_connect} from '/node_modules/@logankeenan/shared-space-app/shared_space_app.js';

async function processIncomingData(data, peer) {
    let dataAsJson = JSON.parse(data);

    if (dataAsJson.type === "request") {
        var webrtcRequest = JSON.parse(dataAsJson.message);

        let appRequest = new AppRequest(webrtcRequest.path, webrtcRequest.method);
        appRequest.body = webrtcRequest.body;

        let appResponse = await app(appRequest);

        peer.send(JSON.stringify({
            type: 'response',
            request_id: dataAsJson.request_id,
            message: appResponse.as_json_string()
        }));

    } else if (dataAsJson.type === "response") {
        const resolver = window.simplePeerAdapter.activeRequestsResolver[dataAsJson.request_id];
        resolver(dataAsJson.message);

        delete window.simplePeerAdapter.activeRequestsResolver[dataAsJson.request_id];
    }
}

function setup() {

    window.simplePeerAdapter = {
        peers: {},
        activeRequestsResolver: {},
        createSimplePeer: async function (initiator, device_id, offer) {
            return new Promise((resolveCreatePeerConnection) => {
                let peer = new SimplePeer({
                    initiator: initiator === "true"
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
                    webrtc_on_connect(device_id);
                });

                peer.on('data', async data => {
                    await processIncomingData(data, peer);
                });

                setTimeout(function () {
                    resolveCreatePeerConnection(JSON.stringify(signalData));
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
            return new Promise((resolve) => {
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
        }
    }
}

setup();