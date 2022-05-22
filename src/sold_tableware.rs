use super::*;
use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/sold_tableware/<uid>")]
pub async fn sold_tableware_get(
    conn: LibraryDbConn,
    uid: i32,
) -> Result<Json<SoldTablewareEntity>> {
    use schema::sold_tableware::dsl::*;
    let data: SoldTablewareEntity = conn
        .run(move |c| sold_tableware.filter(staff_id.eq(uid)).first(c))
        .await?;
    Ok(Json(data))
}

#[post("/sold_tableware", data = "<new>")]
pub async fn sold_tableware_new(
    conn: LibraryDbConn,
    new: Json<SoldTableware>,
) -> Result<Json<SoldTablewareEntity>> {
    use schema::sold_tableware::dsl::*;
    let res: SoldTablewareEntity = conn
        .run(move |c| {
            insert_into(sold_tableware)
                .values(new.into_inner())
                .get_result(c)
        })
        .await?;
    Ok(Json(res))
}

#[put("/sold_tableware/<uid>", data = "<updated>")]
pub async fn sold_tableware_update(
    conn: LibraryDbConn,
    uid: i32,
    updated: Json<SoldTableware>,
) -> Result<Json<SoldTablewareEntity>> {
    use schema::sold_tableware::dsl::*;
    let target = update(sold_tableware).filter(staff_id.eq(uid));
    let res: SoldTablewareEntity = conn
        .run(move |c| target.set(updated.into_inner()).get_result(c))
        .await?;
    Ok(Json(res))
}

#[delete("/sold_tableware/<uid>")]
pub async fn sold_tableware_delete(conn: LibraryDbConn, uid: i32) -> Result<Status> {
    use schema::sold_tableware::dsl::*;
    conn.run(move |c| delete(sold_tableware).filter(staff_id.eq(uid)).execute(c))
        .await?;
    Ok(Status::Accepted)
}

#[get("/sold_tableware/list")]
pub async fn sold_tableware_list(conn: LibraryDbConn) -> Result<Json<Vec<SoldTablewareEntity>>> {
    use schema::sold_tableware::dsl::*;
    let all = conn
        .run(|c| sold_tableware.load::<SoldTablewareEntity>(c))
        .await?;
    Ok(Json(all))
}
