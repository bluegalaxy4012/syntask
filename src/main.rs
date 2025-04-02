use std::sync::Mutex;

use actix::Addr;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use actix::prelude::*;
use actix_files::Files;
use actix::Message;


mod model;
use model::{Action, Board, Card};

struct AppState {
    clients: Mutex<Vec<Addr<MyWs>>>,
    board: Mutex<Board>,
}

struct MyWs {
    app_state: web::Data<AppState>,
    username: Option<String>,
}

#[derive(Message)]
#[rtype(result="()")]
struct MyWsMessage(String);

impl actix::Handler<MyWsMessage> for MyWs {
    type Result = ();

    fn handle(&mut self, msg: MyWsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl actix::Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.text("enter username");

    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        self.app_state.clients.lock().unwrap().retain(|addr| addr.connected());
    }
}

impl actix::StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = item {
            //ctx.text(format!("ai scris {}", text));

            if self.username.is_none() {
                let username = text.trim().to_string();
                if username.is_empty() {
                    ctx.text("enter a non-empty username");
                }
                else {
                    self.username = Some(username);

                    let addr = ctx.address();
                    let mut clients = self.app_state.clients.lock().unwrap();
                    clients.push(addr);                    
                }

                return;
            }

            let action: Action = match serde_json::from_str(&text) {
                Ok(action) => action,
                Err(_) => return,
            };

            let mut board = self.app_state.board.lock().unwrap();
            match action {
                Action::AddCard => {
                    board.add_card();
                }
                Action::RemoveCard { id } => {
                    board.remove_card(id);
                }
                Action::AddText { card_id, text } => {
                    board.add_text(card_id, text);
                }
                Action::EditText { card_id, text, text_index } => {
                    board.edit_text(card_id, text, text_index);
                }
                Action::EditTitle { card_id, text } => {
                    board.edit_title(card_id, text);
                }
            }
            let board_json = serde_json::to_string(&*board).unwrap_or_else(|e| {
                eprintln!("serialization error: {}", e);
                "{}".to_string()
            });
            drop(board);
            //self.broadcast_update();
            
            let sendername = self.username.as_ref().unwrap();
            let msg = format!("BROADCASTM:{}|{}", sendername, board_json);
            for client in self.app_state.clients.lock().unwrap().iter() {
                if client != &ctx.address() {
                    let _ =client.try_send(MyWsMessage(msg.clone()));
                }
            }
            
            
        }
    }

}



async fn ws_route(
    request: actix_web::HttpRequest,
    stream: web::Payload,
    data: web::Data<AppState>,
) -> actix_web::Result<HttpResponse> {
    ws::start(MyWs {username:None, app_state: data.clone()}, &request, stream)
}

// async fn index() -> impl Responder {
//     HttpResponse::Ok().body("salut")
// }



async fn get_board(data: web::Data<AppState>) -> impl Responder {
    let board = data.board.lock().unwrap();
    match serde_json::to_string(&*board) {
        Ok(json) => HttpResponse::Ok().content_type("application/json").body(json),
        Err(err) => {
            eprintln!("Failed to serialize board: {}", err);
            HttpResponse::InternalServerError().body("Failed to retrieve board")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        clients: Mutex::new(Vec::new()),
        board: Mutex::new(Board::new()),
    });

    HttpServer::new(move || App::new()
    .app_data(data.clone())
    .route("/ws", web::get().to(ws_route))
    .route("/board", web::get().to(get_board))
    .service(Files::new("/", "./static").index_file("index.html")))
        .bind("127.0.0.1:8777")?
        .run()
        .await
}
