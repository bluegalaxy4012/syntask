use std::sync::Mutex;

use actix::Addr;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;

struct AppState {
    clients: Mutex<Vec<Addr<MyWs>>>,
    board: Mutex<i32>,
}

struct MyWs;

impl actix::Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl actix::StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = item {
            ctx.text(format!("ai scris {}", text));
        }
    }
}

async fn ws_route(
    request: actix_web::HttpRequest,
    stream: web::Payload,
) -> actix_web::Result<HttpResponse> {
    ws::start(MyWs, &request, stream)
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("salut")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
    .route("/", web::get().to(index))
    .route("/ws", web::get().to(ws_route)))
        .bind("127.0.0.1:8777")?
        .run()
        .await
}
