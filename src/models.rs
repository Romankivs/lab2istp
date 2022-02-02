use rocket_sync_db_pools::database;
use rocket::serde::{Serialize, Deserialize};

#[database("pg_library")]
pub struct LibraryDbConn(diesel::PgConnection);

use super::schema::staff;

#[derive(Queryable, Serialize)]
pub struct StaffEntity {
    pub staff_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset, FromForm)]
#[table_name = "staff"]
pub struct Staff {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

  
