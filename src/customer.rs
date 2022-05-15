use super::*;
use serde_json::json;

#[get("/customer/<uid>")]
pub async fn customer_show(conn: LibraryDbConn, uid: i32, user: StaffEntity) -> Result<Template> {
    use schema::customer::dsl::*;
    let data: CustomerEntity = conn
        .run(move |c| customer.filter(driver_license_id.eq(uid)).first(c))
        .await?;
    Ok(Template::render(
        "customer/show",
        json!({"data": data, "user": user}),
    ))
}

#[post("/customer", data = "<new>")]
pub async fn customer_new(
    conn: LibraryDbConn,
    new: Form<CustomerEntityForm>,
    _user: StaffEntity,
) -> Result<Redirect> {
    use schema::customer::dsl::*;
    let conv_date: chrono::NaiveDate =
        chrono::NaiveDate::parse_from_str(&new.birth_date, "%Y-%m-%d")
            .expect("Date conversion error");
    let converted = CustomerEntity {
        driver_license_id: new.driver_license_id,
        first_name: new.first_name.clone(),
        last_name: new.last_name.clone(),
        birth_date: conv_date,
        email: new.email.clone(),
        phone_number: new.phone_number.clone(),
    };
    conn.run(move |c| insert_into(customer).values(converted).execute(c))
        .await?;
    Ok(Redirect::to(uri!(customer_list)))
}

#[put("/customer/<uid>", data = "<updated>")]
pub async fn customer_update(
    conn: LibraryDbConn,
    uid: i32,
    updated: Form<CustomerEntityForm>,
    _user: StaffEntity,
) -> Result<Redirect> {
    use schema::customer::dsl::*;
    let target = update(customer).filter(driver_license_id.eq(uid));
    let conv_date: chrono::NaiveDate =
        chrono::NaiveDate::parse_from_str(&updated.birth_date, "%Y-%m-%d")
            .expect("Date conversion error");
    let converted = Customer {
        first_name: updated.first_name.clone(),
        last_name: updated.last_name.clone(),
        birth_date: conv_date,
        email: updated.email.clone(),
        phone_number: updated.phone_number.clone(),
    };
    conn.run(move |c| target.set(converted).execute(c)).await?;
    Ok(Redirect::to(uri!(customer_list)))
}

#[delete("/customer/<uid>")]
pub async fn customer_delete(
    conn: LibraryDbConn,
    uid: i32,
    _user: StaffEntity,
) -> Result<Redirect> {
    use schema::customer::dsl::*;
    conn.run(move |c| {
        delete(customer)
            .filter(driver_license_id.eq(uid))
            .execute(c)
    })
    .await?;

    Ok(Redirect::to(uri!(customer_list)))
}

#[get("/customer/add")]
pub async fn customer_add_menu(_conn: LibraryDbConn, user: StaffEntity) -> Result<Template> {
    Ok(Template::render("customer/add", json!({ "user": user })))
}

#[get("/customer/update/<uid>")]
pub async fn customer_update_menu(
    conn: LibraryDbConn,
    uid: i32,
    user: StaffEntity,
) -> Result<Template> {
    use schema::customer::dsl::*;
    let data: CustomerEntity = conn
        .run(move |c| customer.filter(driver_license_id.eq(uid)).first(c))
        .await?;
    Ok(Template::render(
        "customer/update",
        json!({"data": data,
            "user": user
        }),
    ))
}

#[get("/customer/list")]
pub async fn customer_list(conn: LibraryDbConn, user: StaffEntity) -> Result<Template> {
    use schema::customer::dsl::*;
    let all = conn.run(|c| customer.load::<CustomerEntity>(c)).await?;
    let context = json!({
        "entities": all,
        "user" : user
    });
    Ok(Template::render(
        "customer/list",
        json!({"data": context, "user": user}),
    ))
}
