use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Clone, Serialize, Deserialize, PostgresMapper, GraphQLObject)]
#[pg_mapper(table = "users")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub city: String,
    pub state: String,
    pub country: String,
    #[graphql(skip)]
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
}
