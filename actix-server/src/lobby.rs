use crate::messages::{ClientActorMessage, Connect, Disconnect, WsMessage, ClientRequestRoomUUid};
use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;


type Socket = Recipient<WsMessage>;

struct Room {
    name: String,
    users: HashSet<Uuid>,
}
impl Default for Room {
    fn default() -> Self {
        Self { 
            name: String::new(),
            users: HashSet::new() 
        }
    }
}

impl Room {
    fn new(name: String) -> Self {
        Self {
            name,
            users: HashSet::new()
        }
    }
}

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>, //self id to self
    rooms: HashMap<Uuid, Room>,      //room id  to list of users id
}

impl Default for Lobby {
    fn default() -> Self {
        let mut rooms = HashMap::new();
        let main_uuid = Uuid::new_v4();
        let main_room = Room::new(String::from("Main"));
        rooms.insert(main_uuid, main_room);

        Self {
            sessions: HashMap::new(),
            rooms,
        }
    }
}

impl Lobby {
    fn send_message(&self, message: &str, id_to: &Uuid) { // helper function to send message to a socket from lobby
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient
                .do_send(WsMessage(message.to_owned()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
    fn get_rooms(&self) -> Vec<String> {
        let rooms : Vec<String> = self.rooms.iter().map(|(k, v)| -> String {
            format!("{} {}", k.to_string(), v.name.to_string())
        }).collect();
        rooms
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;


}

impl Handler<ClientRequestRoomUUid> for Lobby {
    type Result =  Vec<String>;

    fn handle(&mut self, msg: ClientRequestRoomUUid, _: &mut Context<Self>) -> Self::Result {
        self.get_rooms()
    }
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.id).is_some() {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .users
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != msg.id)
                .for_each(|user_id| self.send_message(&format!("{} disconnected.", &msg.id), user_id));
            if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
                if lobby.users.len() > 1 {
                    lobby.users.remove(&msg.id);
                } else {
                    //only one in the lobby, remove it entirely
                    self.rooms.remove(&msg.room_id);
                }
            }
        }
    }
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        self.rooms
            .entry(msg.lobby_id)
            .or_insert_with(Room::default).users.insert(msg.self_id);

        self
            .rooms
            .get(&msg.lobby_id)
            .unwrap()
            .users
            .iter()
            .filter(|conn_id| *conn_id.to_owned() != msg.self_id)
            .for_each(|conn_id| self.send_message(&format!("{} just joined!", msg.self_id), conn_id));

        self.sessions.insert(
            msg.self_id,
            msg.addr,
        );

        self.send_message(&format!("your id is {}", msg.self_id), &msg.self_id);
    }
}

impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _ctx: &mut Context<Self>) -> Self::Result {
        if msg.msg.starts_with("\\w") {
            if let Some(id_to) = msg.msg.split(' ').collect::<Vec<&str>>().get(1) {
                self.send_message(&msg.msg, &Uuid::parse_str(id_to).unwrap());
            }
        } else {
            self.rooms.get(&msg.room_id).unwrap().users.iter().for_each(|client| self.send_message(&msg.msg, client));
        }
    }
}