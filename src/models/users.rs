use chrono::NaiveDateTime;
use diesel::{PgConnection, Queryable};
use juniper::{graphql_value, FieldError, GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

use crate::schema::users::dsl::*;
use diesel::prelude::*;

#[derive(Clone, Serialize, Deserialize, PostgresMapper, GraphQLObject, Queryable)]
#[pg_mapper(table = "users")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    #[graphql(skip)]
    pub password: Option<String>,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub deleted: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(GraphQLInputObject)]
pub struct UserRegister {
    pub username: String,
    pub email: String,
    pub password1: String,
    pub password2: String,
}

impl UserRegister {
    pub fn validate(&self, conn: &mut PgConnection) -> Result<(), FieldError> {
        // errors arr
        if self.password1 != self.password2 {
            return Err(FieldError::new(
                "Passwords do not match",
                graphql_value!("passwords_do_not_match".to_string()),
            ));
        }
        // password length
        if self.password1.len() < 5 {
            return Err(FieldError::new(
                "Password is too short",
                graphql_value!("password_too_short".to_string()),
            ));
        }

        // validate email too weak
        let re_alpha = regex::Regex::new(r"[a-zA-Z]").unwrap();
        let re_digit = regex::Regex::new(r"\d").unwrap();

        if !(re_alpha.find(&self.password1).is_some() && re_digit.find(&self.password1).is_some()) {
            return Err(FieldError::new(
                "Password is too weak",
                graphql_value!("password_too_weak".to_string()),
            ));
        }

        // email length
        if self.email.len() < 5 {
            return Err(FieldError::new(
                "Email is too short",
                graphql_value!("email_too_short".to_string()),
            ));
        }
        // validate email regex
        let re = regex::Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
        if !re.is_match(&self.email) {
            return Err(FieldError::new(
                "Email is invalid",
                graphql_value!("email_invalid".to_string()),
            ));
        }

        // // query db for email
        let result = users
            .filter(email.eq(&self.email))
            .first::<User>(conn)
            .optional()
            .map_err(|_e| {
                FieldError::new(
                    "Database error",
                    graphql_value!("internal_error".to_string()),
                )
            })?;

        if result.is_some() {
            return Err(FieldError::new(
                "user with email already exists",
                graphql_value!("email_already_exists".to_string()),
            ));
        }

        // username length
        if self.username.len() < 3 {
            return Err(FieldError::new(
                "Username is too short",
                graphql_value!("username_too_short".to_string()),
            ));
        }

        // // query db for username
        let result = users
            .filter(username.eq(&self.username))
            .first::<User>(conn)
            .optional()
            .map_err(|_e| {
                FieldError::new(
                    "Database error",
                    graphql_value!("internal_error".to_string()),
                )
            })?;

        if result.is_some() {
            return Err(FieldError::new(
                "username already taken",
                graphql_value!("username_already_exists".to_string()),
            ));
        }

        Ok(())
    }
}

#[derive(GraphQLInputObject)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(GraphQLInputObject)]
pub struct ChangePassword {
    pub token: String,
    pub password1: String,
    pub password2: String,
}

impl ChangePassword {
    pub fn validate(&self) -> Result<(), FieldError> {
        // errors arr
        if self.password1 != self.password2 {
            return Err(FieldError::new(
                "Passwords do not match",
                graphql_value!("passwords_do_not_match".to_string()),
            ));
        }
        // password length
        if self.password1.len() < 5 {
            return Err(FieldError::new(
                "Password is too short",
                graphql_value!("password_too_short".to_string()),
            ));
        }

        // validate email too weak
        let re_alpha = regex::Regex::new(r"[a-zA-Z]").unwrap();
        let re_digit = regex::Regex::new(r"\d").unwrap();

        if !(re_alpha.find(&self.password1).is_some() && re_digit.find(&self.password1).is_some()) {
            return Err(FieldError::new(
                "Password is too weak",
                graphql_value!("password_too_weak".to_string()),
            ));
        }

        Ok(())
    }
}
