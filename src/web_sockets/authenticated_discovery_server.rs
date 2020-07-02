use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use std::collections::{HashMap, HashSet};
use std::borrow::{ToOwned, Borrow};
use uuid::Uuid;
use std::collections::hash_map::RandomState;

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
pub struct Device {
    pub user_id: i64,
    pub name: String,
    pub id: Uuid,
}

/// New chat session is created
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub recipient: Recipient<Message>,
    pub device: Device,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub device: Device,
}

/// Send message to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub device: Device,
    /// Peer message
    pub message: String,
}



/// `ChatServer` manages chat rooms and responsible for coordinating chat
/// session. implementation is super primitive
pub struct AuthenticatedDiscoveryServer {
    // ids from the rooms link to the session
    sessions: HashMap<Device, Recipient<Message>>,

    // The key is the id of the logged in user.
    // the values are all the devices that currently exist
    user_devices: HashMap<i64, HashSet<Device>>,
    rng: ThreadRng,
}

impl Default for AuthenticatedDiscoveryServer {
    fn default() -> AuthenticatedDiscoveryServer {

        AuthenticatedDiscoveryServer {
            sessions: HashMap::new(),
            user_devices: HashMap::new(),
            // TODO idk what this is for
            rng: rand::thread_rng(),
        }
    }
}

impl AuthenticatedDiscoveryServer {
    /// Send message to all users in the room

    fn send_message(&self, device_sender: &Device, message: &str) {

        if let Some(devices_for_user) = self.user_devices.get(&device_sender.user_id) {
            for device in devices_for_user {
                if device_sender.ne(device) {
                    if let Some(recipient) = self.sessions.get(&device) {
                        recipient.do_send(Message(message.to_owned()));
                    }
                }
            }
        }
    }
}

/// Make actor from `ChatServer`
impl Actor for AuthenticatedDiscoveryServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

impl Handler<Connect> for AuthenticatedDiscoveryServer {
    type Result = ();

    fn handle(&mut self, connect: Connect, ctx: &mut Context<Self>) -> Self::Result {
        // add that
        let recipient = connect.recipient;
        let device = connect.device;


        // Add new device to the session
        self.sessions.insert(device.clone(), recipient);

        // Add new device to the set of devices for a user
        let user_devices_with_new_device = match self.user_devices.get(&device.clone().user_id) {
            None => {
                let mut user_devices: HashSet<Device> = HashSet::new();
                user_devices.insert(device.clone());
                user_devices
            }
            Some(user_devices) => {
                let mut set = user_devices.clone();
                set.insert(device.clone());
                set
            }
        };
        self.user_devices.insert(device.user_id.clone(), user_devices_with_new_device);


        let connected_message = format!("/connected {}", json!(device).to_string());
        self.send_message(&device, connected_message.as_str());

        ()
    }
}

impl Handler<Disconnect> for AuthenticatedDiscoveryServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        let mut rooms: Vec<String> = Vec::new();

        // remove device from session
        self.sessions.remove(&msg.device);

        // remove device from user devices
        if let Some(user_devices) = self.user_devices.get(&msg.device.user_id) {
            let mut user_devices_with_device_removed = user_devices.clone();
            user_devices_with_device_removed.remove(&msg.device);

            self.user_devices.insert(msg.device.user_id, user_devices_with_device_removed);
        }

        ()
        // should I notify that that the device disconnected?
    }
}

/// Handler for Message message.
impl Handler<ClientMessage> for AuthenticatedDiscoveryServer {
    type Result = ();

    fn handle(&mut self, client_message: ClientMessage, _: &mut Context<Self>) {
        self.send_message(&client_message.device, client_message.message.as_str());
    }
}