use super::*;
use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/customer/<uid>")]
pub async fn customer_get(conn: LibraryDbConn, uid: i32) -> Result<Json<CustomerEntity>> {
    use schema::customer::dsl::*;
    let data: CustomerEntity = conn
        .run(move |c| customer.filter(customer_id.eq(uid)).first(c))
        .await?;
    Ok(Json(data))
}

#[post("/customer", data = "<new>")]
pub async fn customer_new(
    conn: LibraryDbConn,
    new: Json<Customer>,
) -> Result<Json<CustomerEntity>> {
    use schema::customer::dsl::*;
    let res: CustomerEntity = conn
        .run(move |c| insert_into(customer).values(new.into_inner()).get_result(c))
        .await?;
    Ok(Json(res))
}

#[put("/customer/<uid>", data = "<updated>")]
pub async fn customer_update(
    conn: LibraryDbConn,
    uid: i32,
    updated: Json<Customer>,
) -> Result<Json<CustomerEntity>> {
    use schema::customer::dsl::*;
    let target = update(customer).filter(customer_id.eq(uid));
    let res: CustomerEntity = conn
        .run(move |c| target.set(updated.into_inner()).get_result(c))
        .await?;
    Ok(Json(res))
}

#[delete("/customer/<uid>")]
pub async fn customer_delete(conn: LibraryDbConn, uid: i32) -> Result<Status> {
    use schema::customer::dsl::*;
    conn.run(move |c| delete(customer).filter(customer_id.eq(uid)).execute(c))
        .await?;
    Ok(Status::Accepted)
}

#[get("/customer/list")]
pub async fn customer_list(conn: LibraryDbConn) -> Result<Json<Vec<CustomerEntity>>> {
    use schema::customer::dsl::*;
    let all = conn.run(|c| customer.load::<CustomerEntity>(c)).await?;
    Ok(Json(all))
}
