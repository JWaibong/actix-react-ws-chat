
use crate::ws::WsConn;
use crate::lobby::Lobby;
use crate::messages::ClientRequestRoomUUid;
use actix::{Addr, WrapFuture, ActorFutureExt};
use actix_web::{get, Responder, web::{self, Json}, Error, HttpResponse, HttpRequest};
use actix_web_actors::ws;
use uuid::Uuid;


#[get("/hello")]
pub async fn hello() -> impl Responder {
    web::Json("Hello from Server")
}




#[get("/current_rooms")]
pub async fn current_rooms(srv: web::Data<Addr<Lobby>>) -> Json<Vec<String>> {
    let rooms = srv
        .get_ref()
        .clone()
        .send(ClientRequestRoomUUid {})
        .await;
    
        match rooms {
            Ok(res) => Json(res),
            _ => Json(vec![])
        }
}