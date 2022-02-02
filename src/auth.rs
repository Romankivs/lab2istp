use diesel::prelude::*;
use rocket::request::{self, FromRequest, Request};
use rocket::outcome::Outcome;
use rocket::http::Status;

use super::models::LibraryDbConn;
use super::models::StaffEntity;

#[derive(FromForm)]
pub struct Login<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug)]
pub enum LoginError {
    InvalidData,
    EmailDoesntExist,
    WrongPassword,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for StaffEntity {
    type Error = LoginError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<StaffEntity, Self::Error> {
        let email_ = request.cookies().get_private("user_email");
        let password = request.cookies().get_private("user_password");
        match (email_, password) {
            (Some(e), Some(p)) => {
                let e = e.clone();
                let p = p.clone();
                let conn = LibraryDbConn::get_one(request.rocket())
                    .await
                    .expect("Couldn`t get DB connection");
                use super::schema::staff::dsl::*;
                let staff_user = conn
                    .run(move |c| {
                        staff
                            .filter(email.eq(e.value()))
                            .get_result::<StaffEntity>(c)
                    })
                    .await;
                match staff_user {
                    Ok(user) => {
                        if user.password == p.value() {
                            Outcome::Success(user)
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
