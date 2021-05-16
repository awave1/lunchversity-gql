table! {
    points (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        vendor_id -> Nullable<Int4>,
        amount -> Nullable<Int4>,
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

allow_tables_to_appear_in_same_query!(
    points,
    users,
    vendors,
);
