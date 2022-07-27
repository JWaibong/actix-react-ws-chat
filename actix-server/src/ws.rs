use actix::{fut, ActorContext, ActorFutureExt};

use crate::messages::{Disconnect, Connect, WsMessage, ClientActorMessage}; 
use crate::lobby::Lobby; 

use actix::{Actor, Addr, Running, StreamHandler, WrapFuture, ActorFuture, ContextFutureSpawner};
use actix::{AsyncContext, Handler};
use actix_web_actors::ws;
use actix_web_actors::ws::Message::Text;
use std::time::{Duration, Instant};
use uuid::Uuid;


const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsConn {
    room: Uuid, // each room is a hashmap that maps its id to a list of socket ids
    lobby_addr: Addr<Lobby>, // lobby is an actor that consists of rooms
    hb: Instant, // send the web socket a heart beat every N seconds. If no response, terminate socket
    id: Uuid, // id for identifying the socket. Can be used for commands such as whisper id
}

impl WsConn {
    pub fn new(room: Uuid, lobby_addr: Addr<Lobby>) ->  Self {
        Self {
            room,
            lobby_addr,
            hb: Instant::now(),
            id: Uuid::new_v4()
        }
    }
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Disconnecting failed heartbeat");
                act.lobby_addr.do_send(Disconnect { id: act.id, room_id: act.room });
                ctx.stop();
                return;
            }

            ctx.ping(b"PING");
        });
    }
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;
    //actor is allowed to perform web socket actions, such as listen on a port
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx); // set up heartbeat loop

        let addr = ctx.address();
        self.lobby_addr
            .send(Connect {
                addr: addr.recipient(),
                lobby_id: self.room,
                self_id: self.id,
            }) // asynchronously send a connect message to the lobby actor we want to join 
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(), // in case we dont receive a response back (something wrong with lobby actor),
                    // just terminate the actor
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.lobby_addr.do_send(Disconnect { id: self.id, room_id: self.room });
        // synchronously send lobby actor a disconnect message, it doesn't matter if it receives it or not
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now(); // reset heartbeat clock in response to client ping
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now(); // when client receives server pong, just reset heart beat
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin), // binary message, let ws context deal with it. Should ideally never happen
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason); 
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop(); // web socket message that couldn't fit into one message. Just close and don't deal with this case in this project
            }
            Ok(ws::Message::Nop) => (), // no op => no op
            Ok(Text(s)) => self.lobby_addr.do_send(ClientActorMessage {
                id: self.id,
                msg: s.to_string(),
                room_id: self.room
            }), // on a text message, send it to the lobby
            Err(e) => panic!(),
        }
    }
}
impl Handler<WsMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx : &mut Self::Context) {
        ctx.text(msg.0); 
        // if server puts a WsMessage into the web socket's mailbox, send it to the client
        // reading mail from the mailbox
        // impl Handler<MailType> for ACTOR
        // handle needs to return type Result if the mail is sent using send()
    }
}
