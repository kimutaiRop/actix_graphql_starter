use std::sync::Arc;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use juniper::{EmptyMutation, EmptySubscription, RootNode};

#[derive(Clone)]
pub struct Context {
    pub pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    pub async fn apiVersion() -> &str {
        "1.0"
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;
pub fn create_schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}
