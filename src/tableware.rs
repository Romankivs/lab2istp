use super::*;
use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/tableware/<uid>")]
pub async fn tableware_get(conn: LibraryDbConn, uid: i32) -> Result<Json<TablewareEntity>> {
    use schema::tableware::dsl::*;
    let data: TablewareEntity = conn
        .run(move |c| tableware.filter(tableware_id.eq(uid)).first(c))
        .await?;
    Ok(Json(data))
}

#[post("/tableware", data = "<new>")]
pub async fn tableware_new(
    conn: LibraryDbConn,
    new: Json<Tableware>,
) -> Result<Json<TablewareEntity>> {
    use schema::tableware::dsl::*;
    let res: TablewareEntity = conn
        .run(move |c| {
            insert_into(tableware)
                .values(new.into_inner())
                .get_result(c)
        })
        .await?;
    Ok(Json(res))
}

#[put("/tableware/<uid>", data = "<updated>")]
pub async fn tableware_update(
    conn: LibraryDbConn,
    uid: i32,
    updated: Json<Tableware>,
) -> Result<Json<TablewareEntity>> {
    use schema::tableware::dsl::*;
    let target = update(tableware).filter(tableware_id.eq(uid));
    let res: TablewareEntity = conn
        .run(move |c| target.set(updated.into_inner()).get_result(c))
        .await?;
    Ok(Json(res))
}

#[delete("/tableware/<uid>")]
pub async fn tableware_delete(conn: LibraryDbConn, uid: i32) -> Result<Status> {
    use schema::tableware::dsl::*;
    conn.run(move |c| delete(tableware).filter(tableware_id.eq(uid)).execute(c))
        .await?;
    Ok(Status::Accepted)
}

#[get("/tableware/list")]
pub async fn tableware_list(conn: LibraryDbConn) -> Result<Json<Vec<TablewareEntity>>> {
    use schema::tableware::dsl::*;
    let all = conn.run(|c| tableware.load::<TablewareEntity>(c)).await?;
    Ok(Json(all))
}
