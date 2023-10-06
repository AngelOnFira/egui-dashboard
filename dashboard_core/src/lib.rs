use std::sync::atomic::{AtomicU32, self};

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mime_guess::from_path;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "dist/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[actix_web::get("/")]
async fn index() -> impl Responder {
    handle_embedded_file("index.html")
}

#[actix_web::get("/{_:.*}")]
async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(path.as_str())
}

async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(dist))
        .bind("0.0.0.0:8002")?
        .run()
        .await
}

pub fn init() {
    println!("Hello, world!");

    // Start an async tokio thread for the server
    std::thread::spawn(|| {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { start_server().await.unwrap() })
    });
}

// Create an atomic for IDs
static ID: AtomicU32 = AtomicU32::new(0);

pub enum ObjKind {
    Button,
}

pub struct Object {
    kind: ObjKind,
    id: u32,
}

impl Object {
    pub fn new(kind: ObjKind) -> Self {
        let id = ID.fetch_add(1, atomic::Ordering::SeqCst);
        Self { kind, id }
    }
}
