#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
use diesel::{delete, insert_into, prelude::*, update};
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::http::{Cookie, CookieJar};
use rocket::request::FlashMessage;
use rocket::response::{Debug, Flash, Redirect};
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use std::path::PathBuf;

mod models;
mod schema;
use models::*;
mod customer;
mod manufacturer;
mod sold_tableware;
mod staff;
mod tableware;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/data/<uid>")]
async fn data(conn: LibraryDbConn, uid: i32) -> Result<Template> {
    use schema::staff::dsl::*;
    let data: StaffEntity = conn
        .run(move |c| staff.filter(staff_id.eq(uid)).first(c))
        .await?;
    Ok(Template::render("show", data))
}

#[post("/data", data = "<new_staff>")]
async fn new_data(conn: LibraryDbConn, new_staff: Form<Staff>) -> Result<Redirect> {
    use schema::staff::dsl::*;
    conn.run(move |c| insert_into(staff).values(&*new_staff).execute(c))
        .await?;
    Ok(Redirect::to(uri!(index)))
}

#[put("/data/<uid>", data = "<updated_user>")]
async fn update_data(conn: LibraryDbConn, uid: i32, updated_user: Form<Staff>) -> Result<Redirect> {
    use schema::staff::dsl::*;
    let target = update(staff).filter(staff_id.eq(uid));
    conn.run(move |c| target.set(&*updated_user).execute(c))
        .await?;
    Ok(Redirect::to(uri!(index)))
}

#[delete("/data/<uid>")]
async fn delete_data(conn: LibraryDbConn, uid: i32) -> Result<Redirect> {
    use schema::staff::dsl::*;
    conn.run(move |c| delete(staff).filter(staff_id.eq(uid)).execute(c))
        .await?;

    Ok(Redirect::to(uri!(index)))
}

#[get("/register")]
fn add_staff() -> Template {
    Template::render("add", HashMap::<i32, i32>::new())
}

#[get("/data/update/<uid>")]
async fn update_staff(conn: LibraryDbConn, uid: i32) -> Result<Template> {
    use schema::staff::dsl::*;
    let data: StaffEntity = conn
        .run(move |c| staff.filter(staff_id.eq(uid)).first(c))
        .await?;
    Ok(Template::render("update", data))
}

#[get("/")]
async fn index(_conn: LibraryDbConn) -> Result<Template> {
    Ok(Template::render("index", {}))
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
                public_file,
                data,
                new_data,
                update_data,
                delete_data,
                add_staff,
                update_staff,
                tableware::tableware_list,
                tableware::tableware_get,
                tableware::tableware_new,
                tableware::tableware_update,
                tableware::tableware_delete,
                sold_tableware::sold_tableware_list,
                sold_tableware::sold_tableware_get,
                sold_tableware::sold_tableware_new,
                sold_tableware::sold_tableware_update,
                sold_tableware::sold_tableware_delete,
                manufacturer::manufacturer_list,
                manufacturer::manufacturer_get,
                manufacturer::manufacturer_new,
                manufacturer::manufacturer_update,
                manufacturer::manufacturer_delete,
                customer::customer_list,
                customer::customer_get,
                customer::customer_new,
                customer::customer_update,
                customer::customer_delete,
                staff::staff_list,
                staff::staff_get,
                staff::staff_new,
                staff::staff_update,
                staff::staff_delete,
            ],
        )
        .attach(Template::fairing())
        .attach(LibraryDbConn::fairing())
}
