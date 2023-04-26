use crate::models::users::User;
use crate::models::users::{ChangePassword, UserLogin, UserRegister};
use crate::schema::users;
use crate::utils::{extract_email, generate_jwt, verify_token};
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use juniper::{graphql_value, FieldError, GraphQLObject};
use r2d2::Pool;
use std::sync::Arc;
use tera::Tera;

pub struct UserRepository {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

#[derive(GraphQLObject)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
    pub refresh_token: String,
}

#[derive(GraphQLObject)]
pub struct SuccessMessage {
    pub message: String,
    pub success: bool,
}

impl UserRepository {
    pub fn new(pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> UserRepository {
        UserRepository { pool }
    }

    pub async fn get(&self, id: i32) -> Result<User, FieldError> {
        let conn = &mut *self.pool.get()?;
        let result = users::table
            .filter(users::id.eq(id))
            .first::<User>(conn)
            .map_err(|_e| {
                FieldError::new(
                    "User not found",
                    graphql_value!("internal_error".to_string()),
                )
            })?;
        Ok(result)
    }
    pub async fn all_users(&self) -> Result<Vec<User>, FieldError> {
        let conn = &mut *self.pool.get()?;
        let users = users::table.load::<User>(conn)?;
        Ok(users)
    }

    pub async fn register(&self, user: UserRegister, tera: Arc<Tera>) -> Result<User, FieldError> {
        let mut connection = &mut *self.pool.get()?;
        user.validate(&mut connection)?;

        let password = bcrypt::hash(&user.password1, 10)?;
        let sql = "INSERT INTO users (username, email, password) VALUES ($1, $2, $3)";

        diesel::sql_query(sql)
            .bind::<diesel::sql_types::Text, _>(&user.username)
            .bind::<diesel::sql_types::Text, _>(&user.email)
            .bind::<diesel::sql_types::Text, _>(&password)
            .execute(connection)
            .map_err(|_e| {
                FieldError::new(
                    "User not found",
                    graphql_value!("internal_error".to_string()),
                )
            })?;

        let mut mail_context = tera::Context::new();
        mail_context.insert("username", &user.username);
        mail_context.insert("email", &user.email);
        mail_context.insert("domain", "http://localhost:3000");
        mail_context.insert("logo", "https://www.elegal.ascendth.com/_next/image?url=https%3A%2F%2Felegal-ascend.s3.amazonaws.com%2Fpublic%2Flogo.png&w=256&q=75");
        mail_context.insert("company", "drgz");
        mail_context.insert(
            "link",
            &format!("http://localhost:3000/verify/{}", verify_token(&user.email)),
        );
        crate::mailer::send_html_email(
            &user.email,
            "info@ascendth.com",
            "Account Activation",
            "emails/register.html",
            &mail_context,
            tera,
        )
        .await;
        let result = users::table
            .filter(users::email.eq(&user.email))
            .first::<User>(connection)
            .map_err(|_e| {
                FieldError::new(
                    "User not found",
                    graphql_value!("internal_error".to_string()),
                )
            })?;
        Ok(result)
    }

    pub async fn verify_email(&self, email: String) -> Result<SuccessMessage, FieldError> {
        let connection = &mut *self.pool.get()?;
        let sql = "UPDATE users SET email_verified = true WHERE email = $1";
        diesel::sql_query(sql)
            .bind::<diesel::sql_types::Text, _>(&email)
            .execute(connection)
            .map_err(|_e| {
                FieldError::new(
                    "User not found",
                    graphql_value!("internal_error".to_string()),
                )
            })?;
        Ok(SuccessMessage {
            message: "Email verified".to_string(),
            success: true,
        })
    }

    pub async fn request_password_reset(
        &self,
        email: String,
        tera: Arc<Tera>,
    ) -> Result<SuccessMessage, FieldError> {
        let connection = &mut *self.pool.get()?;

        // check if user exists
        let result = users::table
            .filter(users::email.eq(&email))
            .first::<User>(connection)
            .optional()
            .map_err(|_e| {
                FieldError::new(
                    "User not found",
                    graphql_value!("internal_error".to_string()),
                )
            })?;
        let message =  SuccessMessage {
            message: "Password reset instruction sent".to_string(),
            success: true,
        };
        if result.is_none() {
            return Ok(message);
        } else {
            let user = result.unwrap();
            let mut mail_context = tera::Context::new();
            mail_context.insert("username", &user.username);
            mail_context.insert("email", &user.email);
            mail_context.insert("domain", "http://localhost:3000");
            mail_context.insert("logo", "https://www.elegal.ascendth.com/_next/image?url=https%3A%2F%2Felegal-ascend.s3.amazonaws.com%2Fpublic%2Flogo.png&w=256&q=75");
            mail_context.insert("company", "drgz");
            mail_context.insert(
                "link",
                &format!(
                    "http://localhost:3000/reset-password/{}",
                    verify_token(&user.email)
                ),
            );
            crate::mailer::send_html_email(
                &user.email,
                "info@ascendth.com",
                "Password reset",
                "emails/password-reset.html",
                &mail_context,
                tera,
            )
            .await;

            return Ok(message);
        }
    }
    // login
    pub async fn login(&self, user: UserLogin) -> Result<LoginResponse, FieldError> {
        let connection = &mut *self.pool.get()?;

        let result = users::table
            .filter(users::email.eq(&user.email))
            .first::<User>(connection)
            .map_err(|_e| {
                FieldError::new(
                    "invalid credentials",
                    graphql_value!("internal_error".to_string()),
                )
            })?;

        // check that email is verified
        if !result.email_verified {
            return Err(FieldError::new(
                "Email not verified",
                graphql_value!("email_error".to_string()),
            ));
        }

        let password_str = if let Some(password) = &result.password {
            password.clone()
        } else {
            "".to_string()
        };
        let is_valid = bcrypt::verify(&user.password, &password_str)?;
        if is_valid {
            let token = generate_jwt(&result.id);
            Ok(LoginResponse {
                token,
                user: result,
                refresh_token: "".to_string(),
            })
        } else {
            Err(FieldError::new(
                "invalid credentials",
                graphql_value!("internal_error".to_string()),
            ))
        }
    }

    pub async fn change_password(
        &self,
        input: ChangePassword,
    ) -> Result<SuccessMessage, FieldError> {
        let connection = &mut *self.pool.get()?;
        input.validate()?;
        let email = extract_email(&input.token);
        let result = users::table
            .filter(users::email.eq(email))
            .first::<User>(connection)
            .map_err(|_e| {
                FieldError::new(
                    "User not found",
                    graphql_value!("internal_error".to_string()),
                )
            })?;

        let password = bcrypt::hash(&input.password1, 10)?;
        let sql = "UPDATE users SET password = $1 WHERE id = $2";
        diesel::sql_query(sql)
            .bind::<diesel::sql_types::Text, _>(&password)
            .bind::<diesel::sql_types::Integer, _>(&result.id)
            .execute(connection)
            .map_err(|_e| {
                FieldError::new(
                    "User not found",
                    graphql_value!("internal_error".to_string()),
                )
            })?;
        Ok(SuccessMessage {
            message: "Password changed".to_string(),
            success: true,
        })
    }
}
