table! {
    points (id) {
        id -> Int4,
        user_id -> Int4,
        vendor_id -> Int4,
        amount -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        ucid -> Varchar,
    }
}

table! {
    vendors (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        phone -> Varchar,
    }
}

joinable!(points -> users (user_id));
joinable!(points -> vendors (vendor_id));

allow_tables_to_appear_in_same_query!(points, users, vendors,);
