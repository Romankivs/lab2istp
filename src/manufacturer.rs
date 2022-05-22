use super::*;
use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/manufacturer/<uid>")]
pub async fn manufacturer_get(conn: LibraryDbConn, uid: i32) -> Result<Json<ManufacturerEntity>> {
    use schema::manufacturer::dsl::*;
    let data: ManufacturerEntity = conn
        .run(move |c| manufacturer.filter(manufacturer_id.eq(uid)).first(c))
        .await?;
    Ok(Json(data))
}

#[post("/manufacturer", data = "<new>")]
pub async fn manufacturer_new(
    conn: LibraryDbConn,
    new: Json<Manufacturer>,
) -> Result<Json<ManufacturerEntity>> {
    use schema::manufacturer::dsl::*;
    let res: ManufacturerEntity = conn
        .run(move |c| insert_into(manufacturer).values(new.into_inner()).get_result(c))
        .await?;
    Ok(Json(res))
}

#[put("/manufacturer/<uid>", data = "<updated>")]
pub async fn manufacturer_update(
    conn: LibraryDbConn,
    uid: i32,
    updated: Json<Manufacturer>,
) -> Result<Json<ManufacturerEntity>> {
    use schema::manufacturer::dsl::*;
    let target = update(manufacturer).filter(manufacturer_id.eq(uid));
    let res: ManufacturerEntity = conn
        .run(move |c| target.set(updated.into_inner()).get_result(c))
        .await?;
    Ok(Json(res))
}

#[delete("/manufacturer/<uid>")]
pub async fn manufacturer_delete(conn: LibraryDbConn, uid: i32) -> Result<Status> {
    use schema::manufacturer::dsl::*;
    conn.run(move |c| delete(manufacturer).filter(manufacturer_id.eq(uid)).execute(c))
        .await?;
    Ok(Status::Accepted)
}

#[get("/manufacturer/list")]
pub async fn manufacturer_list(conn: LibraryDbConn) -> Result<Json<Vec<ManufacturerEntity>>> {
    use schema::manufacturer::dsl::*;
    let all = conn.run(|c| manufacturer.load::<ManufacturerEntity>(c)).await?;
    Ok(Json(all))
}