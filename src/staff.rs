use super::*;
use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/staff/<uid>")]
pub async fn staff_get(conn: LibraryDbConn, uid: i32) -> Result<Json<StaffEntity>> {
    use schema::staff::dsl::*;
    let data: StaffEntity = conn
        .run(move |c| staff.filter(staff_id.eq(uid)).first(c))
        .await?;
    Ok(Json(data))
}

#[post("/staff", data = "<new>")]
pub async fn staff_new(
    conn: LibraryDbConn,
    new: Json<Staff>,
) -> Result<Json<StaffEntity>> {
    use schema::staff::dsl::*;
    let res: StaffEntity = conn
        .run(move |c| insert_into(staff).values(new.into_inner()).get_result(c))
        .await?;
    Ok(Json(res))
}

#[put("/staff/<uid>", data = "<updated>")]
pub async fn staff_update(
    conn: LibraryDbConn,
    uid: i32,
    updated: Json<Staff>,
) -> Result<Json<StaffEntity>> {
    use schema::staff::dsl::*;
    let target = update(staff).filter(staff_id.eq(uid));
    let res: StaffEntity = conn
        .run(move |c| target.set(updated.into_inner()).get_result(c))
        .await?;
    Ok(Json(res))
}

#[delete("/staff/<uid>")]
pub async fn staff_delete(conn: LibraryDbConn, uid: i32) -> Result<Status> {
    use schema::staff::dsl::*;
    conn.run(move |c| delete(staff).filter(staff_id.eq(uid)).execute(c))
        .await?;
    Ok(Status::Accepted)
}

#[get("/staff/list")]
pub async fn staff_list(conn: LibraryDbConn) -> Result<Json<Vec<StaffEntity>>> {
    use schema::staff::dsl::*;
    let all = conn.run(|c| staff.load::<StaffEntity>(c)).await?;
    Ok(Json(all))
}