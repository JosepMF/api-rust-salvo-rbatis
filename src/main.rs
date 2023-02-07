#[macro_use]
extern crate rbatis;
extern crate rbdc_mysql;

use salvo::logging::Logger;
use salvo::prelude::{Server, Router, TcpListener};

mod routes;
mod database;

#[tokio::main]
async fn main() {
    
    //database
    database::connect_db();
    
    // main server configuration
    tracing_subscriber::fmt().init();
    let router = Router::new().hoop(Logger).handle(routes::posts);
    tracing::info!("Listening 127.0.0.1:8080");
    Server::new(TcpListener::bind("127.0.0.1:8080")).serve(router).await;

    println!("Hello, world!");
}
