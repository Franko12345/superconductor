use crate::resources;
use crate::state::State;
use actix::Addr;
use actix_web::web::Payload;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Error};
use actix_web_actors::ws;
use resources::*;
use std::borrow::Cow;
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;

mod sock;
use actix_web::middleware::Logger;
pub use sock::*;

async fn assets(path: web::Path<String>) -> HttpResponse {
    use log::info;

    let path = path.into_inner();
    info!("Path is {}", path);
    match Resources::get(&path) {
        Some(content) => {
            let body = match content.data {
                Cow::Borrowed(bytes) => bytes.to_vec(),
                Cow::Owned(bytes) => bytes,
            };
            HttpResponse::Ok()
                .content_type(mime_guess::from_path(path).first_or_octet_stream().as_ref())
                .body(body)
        }
        // Cascading is required because react makes the two views less unified than they were with elm
        // This is for when the stdout page is loading and needs to find its own javascript and other assets
        None => match StdoutResources::get(&path) {
            Some(content) => {
                let body = match content.data {
                    Cow::Borrowed(bytes) => bytes.to_vec(),
                    Cow::Owned(bytes) => bytes,
                };
                HttpResponse::Ok()
                    .content_type(mime_guess::from_path(path).first_or_octet_stream().as_ref())
                    .body(body)
            }
            None => HttpResponse::NotFound().body("404 Not Found"),
        },
    }
}

async fn index(_req: HttpRequest) -> HttpResponse {
    let contents = Resources::get("index.html").unwrap();
    let body = match contents.data {
        Cow::Borrowed(bytes) => bytes.to_vec(),
        Cow::Owned(bytes) => bytes,
    };

    HttpResponse::Ok().content_type("text/html").body(body)
}

async fn stdout(_req: HttpRequest) -> HttpResponse {
    let contents = StdoutResources::get("index.html").unwrap();
    let body = match contents.data {
        Cow::Borrowed(bytes) => bytes.to_vec(),
        Cow::Owned(bytes) => bytes,
    };

    HttpResponse::Ok().content_type("text/html").body(body)
}

async fn main_sock(
    req: HttpRequest,
    stream: Payload,
    state: web::Data<Arc<RwLock<State>>>,
    tx: web::Data<Mutex<mpsc::Sender<Addr<WebsocketHandler>>>>,
) -> Result<HttpResponse, Error> {
    // Start the websocket connection, get the address of the actor
    let (addr, res) =
        ws::start_with_addr(WebsocketHandler::new((**state).clone()), &req, stream)?;
    // Notify the main app of the websocket Addr to be able to send messages to the frontend
    tx.lock().unwrap().send(addr).unwrap();
    Ok(res)
}

async fn stdout_sock(
    req: HttpRequest,
    stream: Payload,
    tx: web::Data<Mutex<mpsc::Sender<Addr<StdoutHandler>>>>,
) -> Result<HttpResponse, Error> {
    let (addr, res) = ws::start_with_addr(StdoutHandler, &req, stream)?;
    tx.lock().unwrap().send(addr).unwrap();
    Ok(res)
}

pub fn launch_webserver(
    state: Arc<RwLock<State>>,
    addr_sender: mpsc::Sender<Addr<WebsocketHandler>>,
    stdout_sender: mpsc::Sender<Addr<StdoutHandler>>,
) -> u16 {
    let (port_tx, port_rx) = mpsc::channel();

    thread::spawn(move || {
        let sys = actix_web::rt::System::new();

        let server = HttpServer::new(move || {
            // Love redundant cloning to abide by Fn limitations
            App::new()
                .app_data(web::Data::new(state.clone()))
                .app_data(web::Data::new(Mutex::new(addr_sender.clone())))
                .app_data(web::Data::new(Mutex::new(stdout_sender.clone())))
                .wrap(Logger::default())
                .route("/", web::get().to(index))
                .route("/stdout", web::get().to(stdout))
                .route("/ws/index", web::get().to(main_sock))
                .route("/ws/stdout", web::get().to(stdout_sock))
                .route("/{path:.*}", web::get().to(assets))
        })
        .bind("127.0.0.1:0")
        .unwrap();

        let port = server.addrs().first().unwrap().port();
        port_tx.send(port).unwrap();

        sys.block_on(server.run()).unwrap();
    });

    port_rx.recv().unwrap()
}
