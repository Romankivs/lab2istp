table! {
    country (country_id) {
        country_id -> Varchar,
        name -> Text,
    }
}

table! {
    customer (customer_id) {
        customer_id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        phone_number -> Text,
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
    sold_tableware (sold_tableware_id) {
        sold_tableware_id -> Int4,
        customer_id -> Int4,
        tableware_id -> Int4,
        staff_id -> Int4,
        date -> Date,
        amount -> Int4,
    }
}

table! {
    staff (staff_id) {
        staff_id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        phone_number -> Text,
    }
}

table! {
    tableware (tableware_id) {
        tableware_id -> Int4,
        manufacturer_id -> Int4,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        main_material -> Text,
        main_colour -> Text,
    }
}

joinable!(manufacturer -> country (country_id));
joinable!(sold_tableware -> customer (customer_id));
joinable!(sold_tableware -> staff (staff_id));
joinable!(sold_tableware -> tableware (tableware_id));
joinable!(tableware -> manufacturer (manufacturer_id));

allow_tables_to_appear_in_same_query!(
    country,
    customer,
    manufacturer,
    sold_tableware,
    staff,
    tableware,
);
