use bigdecimal::BigDecimal;
use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::database;

#[database("pg_library")]
pub struct LibraryDbConn(diesel::PgConnection);

use super::schema::{car, car_model, manufacturer, staff};

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

#[derive(Queryable, Serialize)]
pub struct ManufacturerEntity {
    pub manufacturer_id: i32,
    pub name: String,
    pub country: String,
    pub website: String,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset, FromForm)]
#[table_name = "manufacturer"]
pub struct Manufacturer {
    pub name: String,
    pub country: String,
    pub website: String,
}

#[derive(Queryable, Serialize)]
pub struct CarModelEntity {
    pub car_model_id: i32,
    pub model_name: String,
    pub manufacturer_id: i32,
    pub release_year: i32,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset, FromForm)]
#[table_name = "car_model"]
pub struct CarModel {
    pub model_name: String,
    pub manufacturer_id: i32,
    pub release_year: i32,
}

#[derive(Queryable, Serialize, Insertable)]
#[table_name = "car"]
pub struct CarEntity {
    pub plate_number: String,
    pub car_model_id: i32,
    pub available: bool,
    pub condition: String,
    pub price_per_day: BigDecimal,
}

#[derive(FromForm)]
pub struct CarEntityForm {
    pub plate_number: String,
    pub car_model_id: i32,
    pub available: bool,
    pub condition: String,
    pub price_per_day: f32,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "car"]
pub struct Car {
    pub car_model_id: i32,
    pub available: bool,
    pub condition: String,
    pub price_per_day: BigDecimal,
}

#[derive(FromForm)]
pub struct CarForm {
    pub car_model_id: i32,
    pub available: bool,
    pub condition: String,
    pub price_per_day: f32,
}
