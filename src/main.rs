#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
use diesel::{delete, insert_into, prelude::*, update};
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::outcome::{IntoOutcome, Outcome};
use rocket::request::{self, FlashMessage, FromRequest, Request};
use rocket::response::{status::Created, Debug, Flash, Redirect};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use std::path::PathBuf;

use rocket_sync_db_pools::database;

#[database("pg_library")]
struct LibraryDbConn(diesel::PgConnection);

mod schema;
use schema::users;

#[derive(Queryable, Serialize)]
struct UserEntity {
    id: i32,
    name: String,
    email: String,
    age: i32,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
struct User {
    name: String,
    email: String,
    age: i32,
}

#[derive(FromForm)]
struct Login<'a> {
    email: &'a str,
    password: &'a str,
}

struct AuthUser(i32);

#[derive(Debug)]
enum LoginError {
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
                use schema::users::dsl::*;
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

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/data/<uid>")]
async fn data(conn: LibraryDbConn, uid: i32, _user: AuthUser) -> Option<Json<UserEntity>> {
    use schema::users::dsl::*;
    conn.run(move |c| users.filter(id.eq(uid)).first(c))
        .await
        .map(Json)
        .ok()
}

#[post("/data", format = "json", data = "<new_user>")]
async fn new_data(conn: LibraryDbConn, new_user: Json<User>) -> Result<Created<Json<UserEntity>>> {
    use schema::users::dsl::*;
    let insert_res: UserEntity = conn
        .run(move |c| insert_into(users).values(&*new_user).get_result(c))
        .await?;
    Ok(Created::new("/data").body(Json(insert_res)))
}

#[put("/data/<uid>", format = "json", data = "<updated_user>")]
async fn update_data(
    conn: LibraryDbConn,
    uid: i32,
    updated_user: Json<User>,
) -> Option<Json<UserEntity>> {
    use schema::users::dsl::*;
    let target = update(users).filter(id.eq(uid));
    conn.run(move |c| target.set(&*updated_user).get_result(c))
        .await
        .map(Json)
        .ok()
}

#[delete("/data/<uid>")]
async fn delete_data(conn: LibraryDbConn, uid: i32) -> Result<Option<()>> {
    use schema::users::dsl::*;
    let affected_rows = conn
        .run(move |c| delete(users).filter(id.eq(uid)).execute(c))
        .await?;

    Ok((affected_rows == 1).then(|| ()))
}

#[get("/")]
fn index() -> Template {
    let test_1: UserEntity = UserEntity {
        id: 1,
        name: "Ivan".to_string(),
        email: "funny@gmail.com".to_string(),
        age: 42,
    };
    let test_2: UserEntity = UserEntity {
        id: 2,
        name: "Vanya".to_string(),
        email: "sad@gmail.com".to_string(),
        age: 24,
    };

    let mut context: HashMap<String, Vec<UserEntity>> = HashMap::new();
    context.insert("users".to_string(), vec![test_1, test_2]);
    Template::render("index", context)
}

#[get("/login")]
fn login(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render("login", &flash)
}

#[post("/login", data = "<login>")]
async fn post_login(
    conn: LibraryDbConn,
    jar: &CookieJar<'_>,
    login: Form<Login<'_>>,
) -> Result<Redirect, Flash<Redirect>> {
    use schema::users::dsl::*;
    let email_clone = login.email.to_string();
    let user_password = conn
        .run(|c| {
            users
                .select(email)
                .filter(name.eq(email_clone))
                .get_result::<String>(c)
        })
        .await;
    match user_password {
        Ok(pwd) => {
            if pwd == login.password {
                jar.add_private(Cookie::new("user_email", login.email.to_string()));
                jar.add_private(Cookie::new("user_password", pwd));
                Ok(Redirect::to(uri!(index)))
            } else {
                Err(Flash::error(Redirect::to(uri!(login)), "Wrong password"))
            }
        }
        Err(_) => Err(Flash::error(Redirect::to(uri!(login)), "Email not found."))
    }
}

#[get("/public/<file..>")]
async fn public_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(&format!("public/{}", file.to_str()?))
        .await
        .ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                login,
                post_login,
                public_file,
                data,
                new_data,
                update_data,
                delete_data
            ],
        )
        .attach(Template::fairing())
        .attach(LibraryDbConn::fairing())
}
