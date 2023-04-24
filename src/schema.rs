// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        state -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        deleted -> Bool,
        is_staff -> Bool,
        is_superuser -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
