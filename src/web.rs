use std::path::Path;
use std::sync::Mutex;
use actix_web::{App, get, HttpResponse, HttpResponseBuilder, HttpServer, Responder, web};
use actix_files::NamedFile;
use actix_web::http::{header, StatusCode};
use actix_web::web::{Json, service};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use crate::metadata::Track;
use crate::scanner::traverse_dir;

struct AppState {
    app_name: String,
    tracklist: Vec<String>,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let mut body = "<html>\
                                <head>\
                                <title> Magpie</title>\
                                </head>\
                                <body>\
                                    <ul>".to_string();

    for track in &data.tracklist {
        body.push_str(&("<li>".to_string() + track + "</li>"));
    }

    body = body + "</ul></body>";

    let resp = HttpResponse::Ok()
        .content_type("text/html")
        .body(body);

    resp
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("Hello {} from {}!", &name, &app_name)
}

//#[get("/test/{path}")]
#[get("/test")]
async fn musictest() -> impl Responder {
    NamedFile::open_async("./test_data/music/Bring Me The Horizon/Bring Me The Horizon - 2022 - sTraNgeRs/01. sTraNgeRs.flac").await
    //NamedFile::open_async(path.to_string()).await
}

#[actix_web::main]
pub(crate) async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1";
    let port = 8000;

    let test_path = Path::new("test_data/music");

    let mut tracks = vec![];
    let mut tracklist = vec![];

    println!("Hello, {}!", test_path.display());

    tracks.append(&mut traverse_dir(test_path).unwrap());
    for track in tracks {
        println!("{:?}", track);
        let path = track.path.to_str().unwrap();
        tracklist.push(path.clone().to_string());
    }

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("test_data/key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("test_data/cert.pem").unwrap();

    println!("Starting Webserver on {host}: {port}");
    HttpServer::new(move || App::new()
        .app_data(web::Data::new(AppState {
            app_name: String::from("Actix Web"),
            tracklist: tracklist.clone(),
        }))
        .service(hello)
        .service(musictest)
        .service(index)
    )
        .bind_openssl((host, port), builder)?
        .run()
        .await
}
