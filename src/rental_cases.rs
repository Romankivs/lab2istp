use super::*;
use serde_json::json;

#[get("/rental_cases/<uid>")]
pub async fn rental_cases_show(conn: LibraryDbConn, uid: i32, user: StaffEntity) -> Result<Template> {
    use schema::rented_car::dsl::*;
    let data: RentalCaseEntity = conn
        .run(move |c| rented_car.filter(rented_car_id.eq(uid)).first(c))
        .await?;
    Ok(Template::render(
        "rental_cases/show",
        json!({"data": data, "user": user}),
    ))
}

#[post("/rental_cases", data = "<new>")]
pub async fn rental_cases_new(
    conn: LibraryDbConn,
    new: Form<RentalCaseForm>,
    _user: StaffEntity,
) -> Result<Redirect> {
    use schema::rented_car::dsl::*;
    let conv_date_rent: chrono::NaiveDate = chrono::NaiveDate::parse_from_str(&new.rent_date, "%d-%m-%Y")
        .expect("Date conversion error");
    let conv_date_return: chrono::NaiveDate = chrono::NaiveDate::parse_from_str(&new.return_date, "%d-%m-%Y")
        .expect("Date conversion error");
    let converted = RentalCase {
        staff_id : new.staff_id,
        plate_number : new.plate_number.clone(),
        customer_id : new.customer_id,
        rent_date : conv_date_rent,
        return_date : conv_date_return,
        returned : new.returned,
        comment : new.comment.clone()
    };
    conn.run(move |c| insert_into(rented_car).values(converted).execute(c))
        .await?;
    Ok(Redirect::to(uri!(rental_cases_list)))
}

#[put("/rental_cases/<uid>", data = "<updated>")]
pub async fn rental_cases_update(
    conn: LibraryDbConn,
    uid: i32,
    updated: Form<RentalCaseForm>,
    _user: StaffEntity,
) -> Result<Redirect> {
    use schema::rented_car::dsl::*;
    let target = update(rented_car).filter(rented_car_id.eq(uid));
    let conv_date_rent: chrono::NaiveDate = chrono::NaiveDate::parse_from_str(&updated.rent_date, "%Y-%m-%d")
        .expect("Date conversion error");
    let conv_date_return: chrono::NaiveDate = chrono::NaiveDate::parse_from_str(&updated.return_date, "%Y-%m-%d")
        .expect("Date conversion error");
    let converted = RentalCase {
        staff_id : updated.staff_id,
        plate_number : updated.plate_number.clone(),
        customer_id : updated.customer_id,
        rent_date : conv_date_rent,
        return_date : conv_date_return,
        returned : updated.returned,
        comment : updated.comment.clone()
    };
    conn.run(move |c| target.set(converted).execute(c)).await?;
    Ok(Redirect::to(uri!(rental_cases_list)))
}

#[delete("/rental_cases/<uid>")]
pub async fn rental_cases_delete(conn: LibraryDbConn, uid: i32, _user: StaffEntity) -> Result<Redirect> {
    use schema::rented_car::dsl::*;
    conn.run(move |c| delete(rented_car).filter(rented_car_id.eq(uid)).execute(c))
        .await?;

    Ok(Redirect::to(uri!(rental_cases_list)))
}

#[get("/rental_cases/add")]
pub async fn rental_cases_add_menu(conn: LibraryDbConn, user: StaffEntity) -> Result<Template> {
    use schema::staff::dsl::*;
    use schema::car::dsl::*;
    use schema::customer::dsl::*;
    let staff_members = conn.run(|c| staff.load::<StaffEntity>(c)).await?;
    let cars = conn.run(|c| car.load::<CarEntity>(c)).await?;
    let customers = conn.run(|c| customer.load::<CustomerEntity>(c)).await?;
    Ok(Template::render(
        "rental_cases/add",
        json!({"staff": staff_members, "cars" : cars, "customers" : customers, "user": user}),
    ))
}

#[get("/rental_cases/update/<uid>")]
pub async fn rental_cases_update_menu(
    conn: LibraryDbConn,
    uid: i32,
    user: StaffEntity,
) -> Result<Template> {
    use schema::rented_car::dsl::*;
    let data: RentalCaseEntity = conn
        .run(move |c| rented_car.filter(rented_car_id.eq(uid)).first(c))
        .await?;
    use schema::staff::dsl::*;
    use schema::car::dsl::*;
    use schema::customer::dsl::*;
    let staff_members = conn.run(|c| staff.load::<StaffEntity>(c)).await?;
    let cars = conn.run(|c| car.load::<CarEntity>(c)).await?;
    let customers = conn.run(|c| customer.load::<CustomerEntity>(c)).await?;
    Ok(Template::render(
        "rental_cases/update",
        json!({"data": data,
            "staff": staff_members, "cars" : cars, "customers" : customers,
            "user": user
        }),
    ))
}

#[get("/rental_cases/list")]
pub async fn rental_cases_list(conn: LibraryDbConn, user: StaffEntity) -> Result<Template> {
    use schema::rented_car::dsl::*;
    let all = conn.run(|c| rented_car.load::<RentalCaseEntity>(c)).await?;
    let context = json!({
        "entities": all,
        "user" : user
    });
    Ok(Template::render(
        "rental_cases/list",
        json!({"data": context, "user": user}),
    ))
}
