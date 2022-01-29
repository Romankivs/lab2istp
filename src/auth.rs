use diesel::prelude::*;
use rocket::request::{self, FromRequest, Request};
use rocket::outcome::Outcome;
use rocket::http::Status;

use super::models::LibraryDbConn;

#[derive(FromForm)]
pub struct Login<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

pub struct AuthUser(i32);

#[derive(Debug)]
pub enum LoginError {
    InvalidData,
    EmailDoesntExist,
    WrongPassword,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = LoginError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<AuthUser, Self::Error> {
        let email_ = request.cookies().get_private("user_email");
        let password = request.cookies().get_private("user_password");
        match (email_, password) {
            (Some(e), Some(p)) => {
                let e = e.clone();
                let p = p.clone();
                let conn = LibraryDbConn::get_one(request.rocket())
                    .await
                    .expect("Couldn`t get DB connection");
                use super::schema::users::dsl::*;
                let user_password = conn
                    .run(move |c| {
                        users
                            .select(email)
                            .filter(name.eq(e.value()))
                            .get_result::<String>(c)
                    })
                    .await;
                match user_password {
                    Ok(pwd) => {
                        if pwd == p.value() {
                            Outcome::Success(AuthUser(1))
                        } else {
                            Outcome::Failure((Status::Unauthorized, LoginError::WrongPassword))
                        }
                    }
                    Err(_) => {
                        Outcome::Failure((Status::Unauthorized, LoginError::EmailDoesntExist))
                    }
                }
            }
            _ => Outcome::Failure((Status::Unauthorized, LoginError::InvalidData)),
        }
    }
}
