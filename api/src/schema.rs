// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    favourites (id) {
        id -> Uuid,
        user_id -> Uuid,
        restaurant_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    ratings (id) {
        id -> Uuid,
        restaurant_id -> Uuid,
        stars -> Int2,
        price -> Int2,
        rating -> Nullable<Text>,
        #[max_length = 50]
        author -> Varchar,
        created_at -> Timestamptz,
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
        picture -> Nullable<Bytea>,
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
        device_info -> Text,
        ip_address -> Inet,
        user_agent -> Text,
        created_at -> Timestamptz,
        expires_at -> Timestamptz,
        last_accessed_at -> Timestamptz,
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
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(favourites -> restaurants (restaurant_id));
diesel::joinable!(favourites -> users (user_id));
diesel::joinable!(ratings -> restaurants (restaurant_id));
diesel::joinable!(user_sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    favourites,
    ratings,
    restaurants,
    spatial_ref_sys,
    user_sessions,
    users,
);
