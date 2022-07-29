use crate::ws::WsConn;
use crate::lobby::Lobby;
use actix::Addr;
use actix_web::{get, web::Data, web::Path, web::Payload, Error, HttpResponse, HttpRequest, HttpMessage, http::header::HeaderValue};
use actix_web_actors::ws;
use uuid::Uuid;

#[get("/{group_id}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    group_id: Path<Uuid>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let ws = WsConn::new(
        group_id.into_inner(),
        srv.get_ref().clone(),
    );

    let resp = ws::start(ws, &req, stream)?;
    if let Some(origin) = req.headers().get("origin") {
        println!("WebSocket handshake successfuly performed with {:?}", origin);
    }
    Ok(resp)
}