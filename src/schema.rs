table! {
    car (plate_number) {
        plate_number -> Varchar,
        car_model_id -> Int4,
        available -> Bool,
        condition -> Text,
        price_per_day -> Numeric,
    }
}

table! {
    car_model (car_model_id) {
        car_model_id -> Int4,
        model_name -> Text,
        manufacturer_id -> Int4,
        release_year -> Int4,
    }
}

table! {
    country (country_id) {
        country_id -> Varchar,
        name -> Text,
    }
}

table! {
    customer (driver_license_id) {
        driver_license_id -> Int4,
        first_name -> Text,
        last_name -> Text,
        birth_date -> Date,
        email -> Text,
        phone_number -> Varchar,
    }
}

table! {
    manufacturer (manufacturer_id) {
        manufacturer_id -> Int4,
        name -> Text,
        country_id -> Varchar,
        website -> Text,
    }
}

table! {
    rented_car (rented_car_id) {
        rented_car_id -> Int4,
        staff_id -> Int4,
        plate_number -> Varchar,
        customer_id -> Int4,
        rent_date -> Date,
        return_date -> Date,
        returned -> Bool,
        comment -> Text,
    }
}

table! {
    staff (staff_id) {
        staff_id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        password -> Text,
    }
}

joinable!(car -> car_model (car_model_id));
joinable!(car_model -> manufacturer (manufacturer_id));
joinable!(manufacturer -> country (country_id));
joinable!(rented_car -> car (plate_number));
joinable!(rented_car -> customer (customer_id));
joinable!(rented_car -> staff (staff_id));

allow_tables_to_appear_in_same_query!(
    car,
    car_model,
    country,
    customer,
    manufacturer,
    rented_car,
    staff,
);
