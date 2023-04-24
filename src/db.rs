use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use r2d2::Pool;
use std::env;

pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let migr = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(migr)
        .expect("Failed to create pool.")
    //         .get()
    //         .expect("Failed to get connection from pool.")
}
