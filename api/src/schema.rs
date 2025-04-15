// @generated automatically by Diesel CLI.
//

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    ratings (id) {
        id -> Uuid,
        restaurant_id -> Uuid,
        stars -> Int2,
        price -> Int2,
        rating -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    restaurants (id) {
        id -> Uuid,
        #[max_length = 40]
        name -> Varchar,
        location -> Geography,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    spatial_ref_sys (srid) {
        srid -> Int4,
        #[max_length = 256]
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        #[max_length = 2048]
        srtext -> Nullable<Varchar>,
        #[max_length = 2048]
        proj4text -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    user_sessions (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        session_token -> Varchar,
        #[max_length = 255]
        refresh_token -> Varchar,
        device_info -> Nullable<Text>,
        ip_address -> Nullable<Inet>,
        user_agent -> Nullable<Text>,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
        expires_at -> Timestamptz,
        last_accessed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    users (id) {
        id -> Uuid,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 100]
        salt -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(ratings -> restaurants (restaurant_id));
diesel::joinable!(user_sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    ratings,
    restaurants,
    spatial_ref_sys,
    user_sessions,
    users,
);
