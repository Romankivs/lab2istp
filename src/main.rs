#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
use diesel::{delete, insert_into, prelude::*, update};
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::http::{Cookie, CookieJar};
use rocket::request::FlashMessage;
use rocket::response::{status::Created, Debug, Flash, Redirect};
use rocket::serde::json::Json;
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use std::path::PathBuf;

mod schema;
mod models;
use models::*;
mod auth;
use auth::*;


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
async fn index(conn: LibraryDbConn) -> Result<Template> {
    use schema::users::dsl::*;
    let all_users = conn.run(|c| users.load::<UserEntity>(c)).await?;
    let mut context: HashMap<&str, Vec<UserEntity>> = HashMap::new();
    context.insert("users", all_users);
    Ok(Template::render("index", context))
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
        Err(_) => Err(Flash::error(Redirect::to(uri!(login)), "Email not found.")),
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
