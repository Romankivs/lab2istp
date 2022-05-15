use super::*;
use serde_json::json;

#[get("/manufacturer/<uid>")]
pub async fn man_show(conn: LibraryDbConn, uid: i32, user: StaffEntity) -> Result<Template> {
    use schema::manufacturer::dsl::*;
    let data: ManufacturerEntity = conn
        .run(move |c| manufacturer.filter(manufacturer_id.eq(uid)).first(c))
        .await?;
    Ok(Template::render(
        "manufacturer/show",
        json!({"data":data, "user": user}),
    ))
}

#[post("/manufacturer", data = "<new>")]
pub async fn man_new(
    conn: LibraryDbConn,
    new: Form<Manufacturer>,
    _user: StaffEntity,
) -> Result<Redirect> {
    use schema::manufacturer::dsl::*;
    conn.run(move |c| insert_into(manufacturer).values(&*new).execute(c))
        .await?;
    Ok(Redirect::to(uri!(man_list)))
}

#[put("/manufacturer/<uid>", data = "<updated>")]
pub async fn man_update(
    conn: LibraryDbConn,
    uid: i32,
    updated: Form<Manufacturer>,
    _user: StaffEntity,
) -> Result<Redirect> {
    use schema::manufacturer::dsl::*;
    let target = update(manufacturer).filter(manufacturer_id.eq(uid));
    conn.run(move |c| target.set(&*updated).execute(c)).await?;
    Ok(Redirect::to(uri!(man_list)))
}

#[delete("/manufacturer/<uid>")]
pub async fn man_delete(conn: LibraryDbConn, uid: i32, _user: StaffEntity) -> Result<Redirect> {
    use schema::manufacturer::dsl::*;
    conn.run(move |c| {
        delete(manufacturer)
            .filter(manufacturer_id.eq(uid))
            .execute(c)
    })
    .await?;

    Ok(Redirect::to(uri!(man_list)))
}

#[get("/manufacturer/add")]
pub async fn man_add_menu(conn: LibraryDbConn, user: StaffEntity) -> Result<Template> {
    use schema::country::dsl::*;
    let countries = conn.run(|c| country.load::<CountryEntity>(c)).await?;
    Ok(Template::render(
        "manufacturer/add",
        json!({"countries": countries, "user": user}),
    ))
}

#[get("/manufacturer/update/<uid>")]
pub async fn man_update_menu(conn: LibraryDbConn, uid: i32, user: StaffEntity) -> Result<Template> {
    use schema::manufacturer::dsl::*;
    let data: ManufacturerEntity = conn
        .run(move |c| manufacturer.filter(manufacturer_id.eq(uid)).first(c))
        .await?;
    use schema::country::dsl::*;
    let countries = conn.run(|c| country.load::<CountryEntity>(c)).await?;
    Ok(Template::render(
        "manufacturer/update",
        json!({"data": data,
            "countries": countries,
            "user": user
        }),
    ))
}

#[get("/manufacturer/list")]
pub async fn man_list(conn: LibraryDbConn, user: StaffEntity) -> Result<Template> {
    use schema::manufacturer::dsl::*;
    let all = conn
        .run(|c| manufacturer.load::<ManufacturerEntity>(c))
        .await?;
    let context = json!({
        "entities": all,
        "user" : user
    });
    Ok(Template::render(
        "manufacturer/list",
        json!({"data": context, "user": user}),
    ))
}
