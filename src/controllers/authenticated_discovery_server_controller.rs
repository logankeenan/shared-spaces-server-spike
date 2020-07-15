use actix_web::{HttpRequest, web, HttpResponse, Error, get};
use actix_identity::Identity;
use actix::{Addr, Actor, WrapFuture, AsyncContext, ActorFuture, ActorContext, fut, ContextFutureSpawner, Running, Handler, StreamHandler};
use crate::web_sockets::authenticated_discovery_server::{AuthenticatedDiscoveryServer, Device, ClientMessage, Disconnect, Connect, Message};
use std::time::{Instant, Duration};
use actix_web::web::Query;
use uuid::Uuid;
use actix_web_actors::ws;

struct WsChatSession {
    /// unique session id
    device: Device,
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    /// joined room
    /// Chat server
    addr: Addr<AuthenticatedDiscoveryServer>,

    user_id: i64
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.addr
            .send(Connect {
                recipient: addr.recipient(),
                device: self.device.clone(),
                user_id: self.user_id
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => {
                        ()
                    }
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send(Disconnect { device: self.device.clone(), user_id: self.user_id });
        Running::Stop
    }
}

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);


#[derive(Deserialize)]
pub struct DeviceQueryParts {
    pub name: String,
    pub id: Uuid,
}

pub async fn authenticated_discover_server_router(
    query: Query<DeviceQueryParts>,
    request: HttpRequest,
    identity: Identity,
    stream: web::Payload,
    authenticated_discover_server: web::Data<Addr<AuthenticatedDiscoveryServer>>,
) -> Result<HttpResponse, Error> {
    print!("here");

    let user_id: i64 = identity.identity().unwrap().parse().unwrap();
    let address = authenticated_discover_server.get_ref().clone();
    let device = Device {
        name: query.name.clone(),
        id: query.id.clone(),
    };

    ws::start(
        WsChatSession {
            device,
            hb: Instant::now(),
            addr: address,
            user_id
        },
        &request,
        stream,
    )
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<Message> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        let device = self.device.clone();

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                self.addr.do_send(ClientMessage {
                    device,
                    message: text,
                    user_id: self.user_id
                })
            }
            ws::Message::Binary(bin) => {
                print!("binary");
                ctx.binary(bin)
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl WsChatSession {
    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // notify chat server


                act.addr.do_send(Disconnect { device: act.device.clone(), user_id: act.user_id });

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}


