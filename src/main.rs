#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

use actix_web::{web, App, HttpServer};

use crate::{
    app_state::AppStateWithCounter,
    handlers::{auth_provider, serve_readme, stats, verify},
};

mod app_state;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        auth_counter: Mutex::new(0),
        verify_counter: Mutex::new(0),
        auth_time: Mutex::new(0),
        verify_time: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .service(auth_provider)
            .service(serve_readme)
            .service(verify)
            .service(stats)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
