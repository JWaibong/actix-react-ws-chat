mod ws;
mod lobby;

mod routes;
use lobby::Lobby;
mod messages;
mod start_connection;
use start_connection::start_connection as start_connection_route;
use actix::Actor;

use actix_web::{App, HttpServer, Responder, HttpResponse, get, web, };
//use actix_cors::Cors;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_server = Lobby::default().start(); //create and spin up a lobby

    HttpServer::new(move || {
        //let cors = Cors::permissive();
            //default()
            //.allowed_origin("http://localhost:3000");
        App::new() //register the lobby
            .app_data(web::Data::new(chat_server.clone()))
            .service(start_connection_route) //register our route. rename with "as" import or naming conflict
            .service(
                web::scope("/api")
                    .service(routes::hello)    
                    //.route("/hello", web::get().to(hello)), // same way to do without macro
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}