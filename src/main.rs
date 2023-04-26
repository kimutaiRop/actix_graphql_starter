extern crate diesel;
mod db;
mod handlers;
mod middlewares;
mod models;
mod repositories;
mod utils;
use dotenvy::dotenv;
use std::env;
mod mailer;
mod schema;
use crate::handlers::app_config;
use actix_cors::Cors;
use actix_web::{http::header, http::Method, middleware, web::Data, App, HttpServer};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let tera = match tera::Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let server_addr = "localhost:8080";
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    println!("Starting server at: http://{}", server_addr);
    dotenv().ok();
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_origin("https://studio.apollographql.com")
            .allowed_methods(vec![Method::GET, Method::OPTIONS, Method::POST])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials();

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(Data::new(db::establish_connection()))
            .app_data(Data::new(secret_key.clone()))
            .app_data(Data::new(tera.clone()))
            .configure(app_config)
    })
    .bind(server_addr)?
    .run()
    .await
}
