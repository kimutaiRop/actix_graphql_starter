extern crate diesel;
mod db;
mod handlers;
mod models;

use crate::handlers::app_config;
use actix_cors::Cors;
use actix_web::{http::header, http::Method, middleware, App, HttpServer, web::Data};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let server_addr = "localhost:8080";
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    println!("Starting server at: http://{}", server_addr);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec![Method::GET, Method::OPTIONS, Method::POST])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials();

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(Data::new(db::establish_connection()))
            .configure(app_config)
    })
    .bind(server_addr)?
    .run()
    .await
}
