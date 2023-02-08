#[macro_use]
extern crate rbatis;
extern crate rbdc_mysql;

use rust_embed::RustEmbed;
use salvo::logging::Logger;
use salvo::prelude::{Router, Server, TcpListener};
use salvo::serve_static::static_embed;

mod database;
mod routes;

#[derive(RustEmbed)]
#[folder = "img"]
struct Assets;

#[tokio::main]
async fn main() {
    //database
    database::connect_db();

    // main server configuration
    tracing_subscriber::fmt().init();
    let router = Router::new()
        .push(Router::new().hoop(Logger).handle(routes::posts))
        .push(Router::with_path("/img/<**path>").get(static_embed::<Assets>()));
    tracing::info!("Listening 127.0.0.1:8080");
    Server::new(TcpListener::bind("127.0.0.1:8080"))
        .serve(router)
        .await;

    println!("Hello, world!");
}
