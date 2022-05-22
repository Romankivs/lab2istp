use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::database;

use super::schema::{country, customer, manufacturer, sold_tableware, staff, tableware};

#[database("pg_library")]
pub struct LibraryDbConn(diesel::PgConnection);

#[derive(Queryable, Serialize)]
pub struct StaffEntity {
    pub staff_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset, FromForm)]
#[table_name = "staff"]
pub struct Staff {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
}

#[derive(Queryable, Serialize)]
pub struct CountryEntity {
    pub country_id: String,
    pub name: String,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset, FromForm)]
#[table_name = "country"]
pub struct Country {
    pub name: String,
}

#[derive(Queryable, Serialize)]
pub struct ManufacturerEntity {
    pub manufacturer_id: i32,
    pub name: String,
    pub country_id: String,
    pub website: String,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset, FromForm)]
#[table_name = "manufacturer"]
pub struct Manufacturer {
    pub name: String,
    pub country_id: String,
    pub website: String,
}

#[derive(Queryable, Serialize)]
pub struct TablewareEntity {
    pub tableware_id: i32,
    pub manufacturer_id: i32,
    pub name: String,
    pub type_: String,
    pub main_material: String,
    pub main_colour: String,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset, FromForm)]
#[table_name = "tableware"]
pub struct Tableware {
    pub manufacturer_id: i32,
    pub name: String,
    pub type_: String,
    pub main_material: String,
    pub main_colour: String,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset, FromForm)]
#[table_name = "customer"]
pub struct Customer {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
}

#[derive(Queryable, Serialize, Deserialize, Insertable, FromForm)]
#[table_name = "customer"]
pub struct CustomerEntity {
    pub customer_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "sold_tableware"]
pub struct SoldTablewareEntity {
    pub sold_tableware_id: i32,
    pub customer_id: i32,
    pub tableware_id: i32,
    pub staff_id: i32,
    pub date: chrono::NaiveDate,
    pub amount: i32,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "sold_tableware"]
pub struct SoldTableware {
    pub customer_id: i32,
    pub tableware_id: i32,
    pub staff_id: i32,
    pub date: chrono::NaiveDate,
    pub amount: i32,
}
