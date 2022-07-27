use actix_web::{get, Responder, web};

#[get("/hello")]
pub async fn hello() -> impl Responder {
    web::Json("Hello from Server")
}