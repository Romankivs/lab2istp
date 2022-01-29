use rocket_sync_db_pools::database;
use rocket::serde::{Serialize, Deserialize};

#[database("pg_library")]
pub struct LibraryDbConn(diesel::PgConnection);

use super::schema::users;

#[derive(Queryable, Serialize)]
pub struct UserEntity {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: i32,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub name: String,
    pub email: String,
    pub age: i32,
}

  
