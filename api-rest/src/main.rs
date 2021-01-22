/// Example of api - rest via http
/// Based on Weather Station Prj
/// Víctor M. - CIRCE

/// ***************************** Modules *****************************************
use std::fs;
use futures::StreamExt;
use json::JsonValue;
use serde::{Deserialize, Serialize};
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Error};



/// ****************** Functions **************************************************


/// ******************   Uris  ****************************************************

/// Index File
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(fs::read_to_string("templates/index.html").unwrap())
}

/// status json
#[get("/status")]
async fn state() -> impl Responder {
    HttpResponse::Ok().json(String::from("{temp:44}"))
}

/// Dashboard File
#[get("/dashboard.html")]
async fn dashboard() -> impl Responder {
    HttpResponse::Ok().body(fs::read_to_string("templates/dashboard.html").unwrap())
}

/// Downloads File
#[get("/downloads.html")]
async fn downloads() -> impl Responder {
    HttpResponse::Ok().body(fs::read_to_string("templates/downloads.html").unwrap())
}

/// Location File
#[get("/location.html")]
async fn location() -> impl Responder {
    HttpResponse::Ok().body(fs::read_to_string("templates/location.html").unwrap())
}

/// Music File
#[get("/music.html")]
async fn music() -> impl Responder {
    HttpResponse::Ok().body(fs::read_to_string("templates/music.html").unwrap())
}

/// About File
#[get("/about.html")]
async fn about() -> impl Responder {
    HttpResponse::Ok().body(fs::read_to_string("templates/about.html").unwrap())
}

/// ******************   Icons and images  ******************************************
/// About File
#[get("/static/brackgound_pic.jpg")]
async fn pic1() -> impl Responder {
    HttpResponse::Ok().body(fs::read_to_string("templates/about.html").unwrap())
}

/// Server APP 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(dashboard)
            .service(downloads)
            .service(location)
            .service(music)
            .service(about)


    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}