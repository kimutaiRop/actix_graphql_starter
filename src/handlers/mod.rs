mod graphql;

use actix_web::{web::{self, Data}, HttpResponse};
use diesel::{r2d2::ConnectionManager, PgConnection};
use graphql::{create_schema, Context, Schema};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use r2d2::Pool;

async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn app_config(config: &mut web::ServiceConfig) {
    let schema = Data::new(create_schema());
    config
        .app_data(schema)
        .service(web::resource("/graphql").route(web::post().to(graphql)))
        .service(web::resource("/graphiql").route(web::get().to(graphiql)))
        .service(web::resource("/").route(web::get().to(health)));
}

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    data: web::Json<GraphQLRequest>,
    schema: web::Data<Schema>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    println!("graphql");
    let pool = pool.into_inner();
    let ctx = Context { pool };
    let value = data.execute(&schema, &ctx).await;
    HttpResponse::Ok()
        .content_type("application/json")
        .json(value)
}
