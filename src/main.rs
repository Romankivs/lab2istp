#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::{delete, update};
use rocket::fs::NamedFile;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, json::serde_json::json, Deserialize, Serialize};
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

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/data/<uid>")]
async fn data(conn: LibraryDbConn, uid: i32) -> Option<Json<UserEntity>> {
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
   // let context: HashMap<u32, u32> = HashMap::new();
    let test_1 : UserEntity = UserEntity {
        id : 1,
        name : "Ivan".to_string(),
        email : "funny@gmail.com".to_string(),
        age : 42
    };
    let test_2 : UserEntity = UserEntity {
        id : 2,
        name : "Vanya".to_string(),
        email : "sad@gmail.com".to_string(),
        age : 24
    };
    
    let mut context : HashMap<String, Vec<UserEntity>> = HashMap::new();
    context.insert("users".to_string(), vec![test_1, test_2]); 
    Template::render("index", context)
}

#[get("/login")]
fn login() -> Template {
    let context: HashMap<u32, u32> = HashMap::new();
    Template::render("login", &context)
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
