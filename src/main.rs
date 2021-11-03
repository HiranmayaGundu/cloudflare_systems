use std::fs;
use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/auth/{username}")]
async fn auth_provider(web::Path((username)): web::Path<String>) -> actix_web::Result<NamedFile> {
    let path: PathBuf = PathBuf::from("./public.pem");
    Ok(NamedFile::open(path)?)
}

#[get("/README.txt")]
async fn serve_readme() -> actix_web::Result<NamedFile> {
    let path: PathBuf = PathBuf::from("./README.txt");
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(auth_provider)
            .service(serve_readme)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
