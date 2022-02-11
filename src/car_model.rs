use super::*;
use serde_json::json;

#[get("/car_model/<uid>")]
pub async fn car_mod_show(conn: LibraryDbConn, uid: i32, user: StaffEntity) -> Result<Template> {
    use schema::car_model::dsl::*;
    let data: CarModelEntity = conn
        .run(move |c| car_model.filter(car_model_id.eq(uid)).first(c))
        .await?;
    Ok(Template::render(
        "car_model/show",
        json!({"data":data, "user": user}),
    ))
}

#[post("/car_model", data = "<new>")]
pub async fn car_mod_new(
    conn: LibraryDbConn,
    new: Form<CarModel>,
    _user: StaffEntity,
) -> Result<Redirect> {
    use schema::car_model::dsl::*;
    conn.run(move |c| insert_into(car_model).values(&*new).execute(c))
        .await?;
    Ok(Redirect::to(uri!(car_mod_list)))
}

#[put("/car_model/<uid>", data = "<updated>")]
pub async fn car_mod_update(
    conn: LibraryDbConn,
    uid: i32,
    updated: Form<CarModel>,
    _user: StaffEntity,
) -> Result<Redirect> {
    use schema::car_model::dsl::*;
    let target = update(car_model).filter(car_model_id.eq(uid));
    conn.run(move |c| target.set(&*updated).execute(c)).await?;
    Ok(Redirect::to(uri!(car_mod_list)))
}

#[delete("/car_model/<uid>")]
pub async fn car_mod_delete(conn: LibraryDbConn, uid: i32, _user: StaffEntity) -> Result<Redirect> {
    use schema::car_model::dsl::*;
    conn.run(move |c| delete(car_model).filter(car_model_id.eq(uid)).execute(c))
        .await?;

    Ok(Redirect::to(uri!(car_mod_list)))
}

#[get("/car_model/add")]
pub async fn car_mod_add_menu(conn: LibraryDbConn, user: StaffEntity) -> Result<Template> {
    use schema::manufacturer::dsl::*;
    let manufacturers = conn
        .run(|c| manufacturer.load::<ManufacturerEntity>(c))
        .await?;
    Ok(Template::render(
        "car_model/add",
        json!({"manufacturers": manufacturers, "user": user}),
    ))
}

#[get("/car_model/update/<uid>")]
pub async fn car_mod_update_menu(
    conn: LibraryDbConn,
    uid: i32,
    user: StaffEntity,
) -> Result<Template> {
    use schema::car_model::dsl::*;
    let data: CarModelEntity = conn
        .run(move |c| car_model.filter(car_model_id.eq(uid)).first(c))
        .await?;
    use schema::manufacturer::dsl::*;
    let manufacturers = conn
        .run(|c| manufacturer.load::<ManufacturerEntity>(c))
        .await?;
    Ok(Template::render(
        "car_model/update",
        json!({"data": data,
            "manufacturers": manufacturers,
            "user": user
        }),
    ))
}

#[get("/car_model/list")]
pub async fn car_mod_list(conn: LibraryDbConn, user: StaffEntity) -> Result<Template> {
    use schema::car_model::dsl::*;
    let all = conn.run(|c| car_model.load::<CarModelEntity>(c)).await?;
    let context = json!({
        "entities": all,
        "user" : user
    });
    Ok(Template::render(
        "car_model/list",
        json!({"data": context, "user": user}),
    ))
}
