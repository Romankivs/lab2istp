use super::*;
use calamine::{open_workbook, Reader, Xlsx};
use rocket::fs::TempFile;
use serde_json::json;

#[get("/rental_cases/<uid>")]
pub async fn rental_cases_show(
    conn: LibraryDbConn,
    uid: i32,
    user: StaffEntity,
) -> Result<Template> {
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
    let conv_date_rent: chrono::NaiveDate =
        chrono::NaiveDate::parse_from_str(&new.rent_date, "%Y-%m-%d").unwrap();
    let conv_date_return: chrono::NaiveDate =
        chrono::NaiveDate::parse_from_str(&new.return_date, "%Y-%m-%d").unwrap();
    let converted = RentalCase {
        staff_id: new.staff_id,
        plate_number: new.plate_number.clone(),
        customer_id: new.customer_id,
        rent_date: conv_date_rent,
        return_date: conv_date_return,
        returned: new.returned,
        comment: new.comment.clone(),
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
    let conv_date_rent: chrono::NaiveDate =
        chrono::NaiveDate::parse_from_str(&updated.rent_date, "%Y-%m-%d")
            .expect("Date conversion error");
    let conv_date_return: chrono::NaiveDate =
        chrono::NaiveDate::parse_from_str(&updated.return_date, "%Y-%m-%d")
            .expect("Date conversion error");
    let converted = RentalCase {
        staff_id: updated.staff_id,
        plate_number: updated.plate_number.clone(),
        customer_id: updated.customer_id,
        rent_date: conv_date_rent,
        return_date: conv_date_return,
        returned: updated.returned,
        comment: updated.comment.clone(),
    };
    conn.run(move |c| target.set(converted).execute(c)).await?;
    Ok(Redirect::to(uri!(rental_cases_list)))
}

#[delete("/rental_cases/<uid>")]
pub async fn rental_cases_delete(
    conn: LibraryDbConn,
    uid: i32,
    _user: StaffEntity,
) -> Result<Redirect> {
    use schema::rented_car::dsl::*;
    conn.run(move |c| delete(rented_car).filter(rented_car_id.eq(uid)).execute(c))
        .await?;

    Ok(Redirect::to(uri!(rental_cases_list)))
}

#[get("/rental_cases/add")]
pub async fn rental_cases_add_menu(conn: LibraryDbConn, user: StaffEntity) -> Result<Template> {
    use schema::car::dsl::*;
    use schema::customer::dsl::*;
    use schema::staff::dsl::*;
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
    use schema::car::dsl::*;
    use schema::customer::dsl::*;
    use schema::staff::dsl::*;
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

fn rental_cases_generate_excel(entities: Vec<RentalCaseEntity>) -> () {
    use xlsxwriter::*;
    let workbook = Workbook::new("public/download.xlsx");
    let mut sheet1 = workbook.add_worksheet(Option::Some("Sheet1")).unwrap();
    let bold = workbook.add_format().set_bold();
    sheet1
        .write_string(0, 0, "Rental Case Id", Option::Some(&bold))
        .unwrap();
    sheet1
        .write_string(0, 1, "Staff Id", Option::Some(&bold))
        .unwrap();
    sheet1
        .write_string(0, 2, "Car Plate Number", Option::Some(&bold))
        .unwrap();
    sheet1
        .write_string(0, 3, "Customer's Driver License Id", Option::Some(&bold))
        .unwrap();
    sheet1
        .write_string(0, 4, "Rent Date", Option::Some(&bold))
        .unwrap();
    sheet1
        .write_string(0, 5, "Return Date", Option::Some(&bold))
        .unwrap();
    sheet1
        .write_string(0, 6, "Returned", Option::Some(&bold))
        .unwrap();
    sheet1
        .write_string(0, 7, "Comment", Option::Some(&bold))
        .unwrap();
    for (i, elem) in entities.iter().enumerate() {
        sheet1
            .write_string(
                i as u32 + 1,
                0,
                &elem.rented_car_id.to_string(),
                Option::None,
            )
            .unwrap();
        sheet1
            .write_string(i as u32 + 1, 1, &elem.staff_id.to_string(), Option::None)
            .unwrap();
        sheet1
            .write_string(i as u32 + 1, 2, &elem.plate_number, Option::None)
            .unwrap();
        sheet1
            .write_string(i as u32 + 1, 3, &elem.customer_id.to_string(), Option::None)
            .unwrap();
        sheet1
            .write_string(
                i as u32 + 1,
                4,
                &elem.rent_date.format("%Y-%m-%d").to_string(),
                Option::None,
            )
            .unwrap();
        sheet1
            .write_string(
                i as u32 + 1,
                5,
                &elem.return_date.format("%Y-%m-%d").to_string(),
                Option::None,
            )
            .unwrap();
        sheet1
            .write_string(i as u32 + 1, 6, &elem.returned.to_string(), Option::None)
            .unwrap();
        sheet1
            .write_string(i as u32 + 1, 7, &elem.comment, Option::None)
            .unwrap();
    }
    workbook.close().unwrap();
}

#[get("/rental_cases/excel")]
pub async fn rental_cases_excel(conn: LibraryDbConn, _user: StaffEntity) -> Result<Redirect> {
    use schema::rented_car::dsl::*;
    let all = conn.run(|c| rented_car.load::<RentalCaseEntity>(c)).await?;
    rental_cases_generate_excel(all);
    Ok(Redirect::to(uri!(public_file("download.xlsx"))))
}

#[derive(FromForm)]
pub struct Upload<'f> {
    pub excel: TempFile<'f>,
}

#[post("/rental_cases/excel", data = "<new>")]
pub async fn rental_cases_upload_excel(
    conn: LibraryDbConn,
    _user: StaffEntity,
    mut new: Form<Upload<'_>>,
) -> Result<Redirect> {
    new.excel.copy_to("public/persist.xlsx").await.unwrap();
    use schema::rented_car::dsl::*;
    let all = conn.run(|c| rented_car.load::<RentalCaseEntity>(c)).await?;

    let mut workbook: Xlsx<_> = open_workbook("public/persist.xlsx").unwrap();
    let range = workbook.worksheet_range("Sheet1").unwrap().unwrap();

    let (height, width) = range.get_size();
    println!("Width: {}, Height: {}", width, height);
    conn.run(move |c| delete(rented_car).execute(c)).await?;
    for r in 1..height {
        let tmp = RentalCaseEntity {
            rented_car_id: range
                .get_value((r as u32, 0))
                .unwrap()
                .get_string()
                .unwrap()
                .parse::<i32>()
                .unwrap(),
            staff_id: range
                .get_value((r as u32, 1))
                .unwrap()
                .get_string()
                .unwrap()
                .parse::<i32>()
                .unwrap(),
            plate_number: range
                .get_value((r as u32, 2))
                .unwrap()
                .get_string()
                .unwrap()
                .to_string(),
            customer_id: range
                .get_value((r as u32, 3))
                .unwrap()
                .get_string()
                .unwrap()
                .parse::<i32>()
                .unwrap(),
            rent_date: chrono::NaiveDate::parse_from_str(
                &range
                    .get_value((r as u32, 4))
                    .unwrap()
                    .get_string()
                    .unwrap()
                    .to_string(),
                "%Y-%m-%d",
            )
            .unwrap(),
            return_date: chrono::NaiveDate::parse_from_str(
                &range
                    .get_value((r as u32, 5))
                    .unwrap()
                    .get_string()
                    .unwrap()
                    .to_string(),
                "%Y-%m-%d",
            )
            .unwrap(),
            returned: range
                .get_value((r as u32, 6))
                .unwrap()
                .get_string()
                .unwrap()
                .parse::<bool>()
                .unwrap(),
            comment: range
                .get_value((r as u32, 7))
                .unwrap()
                .get_string()
                .unwrap()
                .to_string(),
        };
        conn.run(move |c| insert_into(rented_car).values(tmp).execute(c))
            .await?;
    }

    rental_cases_generate_excel(all);
    Ok(Redirect::to(uri!("/rental_cases/list")))
}

#[get("/rental_cases/list")]
pub async fn rental_cases_list(conn: LibraryDbConn, user: StaffEntity) -> Result<Template> {
    use schema::rented_car::dsl::*;
    let all = conn.run(|c| rented_car.load::<RentalCaseEntity>(c)).await?;
    let context = json!({
        "entities" : all,
        "user" : user
    });
    Ok(Template::render(
        "rental_cases/list",
        json!({"data": context, "user": user}),
    ))
}
