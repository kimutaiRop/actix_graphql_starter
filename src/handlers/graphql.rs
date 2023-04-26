use crate::middlewares::auth::AuthenticationToken;
use crate::repositories::user::{LoginResponse, SuccessMessage, UserRepository};
use crate::utils::extract_email;
use std::sync::Arc;

use crate::models::users::User;
use crate::models::users::{ChangePassword, UserLogin, UserRegister};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use juniper::{graphql_value, EmptySubscription, FieldError, RootNode};
use tera::Tera;
#[derive(Clone)]
pub struct Context {
    pub pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    pub token_auth: AuthenticationToken,
    pub tera: Arc<Tera>,
}

impl juniper::Context for Context {}

impl Context {
    pub fn user_repository(&self) -> UserRepository {
        UserRepository::new(self.pool.clone())
    }
}

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    pub async fn apiVersion() -> &str {
        "1.0"
    }

    pub async fn users(context: &Context) -> Result<Vec<User>, FieldError> {
        context.user_repository().all_users().await
    }

    pub async fn me(context: &Context) -> Result<User, FieldError> {
        // get authtoken id
        let id = context.token_auth.id;
        if !context.token_auth.authenticated {
            return Err(FieldError::new(
                "User not found",
                graphql_value!("internal_error".to_string()),
            ));
        }
        let id = id.unwrap();
        context.user_repository().get(id).await
    }
}

pub struct Mutation;

#[juniper::graphql_object(
    Context = Context,
)]
impl Mutation {
    pub async fn register(input: UserRegister, context: &Context) -> Result<User, FieldError> {
        let tera = context.tera.clone();
        context.user_repository().register(input, tera).await
    }
    pub async fn login(context: &Context, input: UserLogin) -> Result<LoginResponse, FieldError> {
        context.user_repository().login(input).await
    }
    pub async fn verify_email(
        context: &Context,
        token: String,
    ) -> Result<SuccessMessage, FieldError> {
        let email = extract_email(&token);
        context.user_repository().verify_email(email).await
    }

    pub async fn request_password_reset(
        context: &Context,
        email: String,
    ) -> Result<SuccessMessage, FieldError> {
        let tera = context.tera.clone();
        context
            .user_repository()
            .request_password_reset(email, tera)
            .await
    }

    pub async fn change_password(
        context: &Context,
        input: ChangePassword,
    ) -> Result<SuccessMessage, FieldError> {
        context.user_repository().change_password(input).await
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;
pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
