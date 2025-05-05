mod deps {
    pub use crate::models::prelude::*;
    pub use crate::requests::*;
    pub use crate::responses::*;
    pub use diesel::prelude::*;
    pub use dioxus::prelude::*;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::to_string as serialize;
    pub use uuid::Uuid;
}
use deps::*;

#[server]
pub async fn get_server_ping() -> Result<String, ServerFnError> {
    Ok("Ping from Server".to_string())
}

#[cfg(feature = "server")]
async fn get_ratings_of_restaurant(
    restaurant_uuid: Uuid,
    manager: &deadpool_diesel::postgres::Object,
) -> Result<Vec<Rating>, ServerFnError> {
    use crate::schema::ratings::dsl::*;
    let result = manager
        .interact(move |c| {
            ratings
                .filter(restaurant_id.eq(restaurant_uuid))
                .load::<Rating>(c)
        })
        .await??;
    Ok(result)
}

#[cfg(feature = "server")]
async fn is_favourite_restaurant(
    restaurant_uuid: Uuid,
    user_uuid: Uuid,
    manager: &deadpool_diesel::postgres::Object,
) -> Result<bool, ServerFnError> {
    use crate::schema::favourites::dsl::*;
    use diesel::{dsl::count_star, sql_types::BigInt};
    let result: i64 = manager
        .interact(move |c| {
            favourites
                .filter(user_id.eq(user_uuid))
                .filter(restaurant_id.eq(restaurant_uuid))
                .select(count_star())
                .first(c)
        })
        .await??;
    Ok(result > 0)
}

#[cfg(feature = "server")]
async fn to_restaurant_items(
    restaurants: Vec<Restaurant>,
    pool: deadpool_diesel::postgres::Pool,
    session: Option<LoginSession>,
) -> Result<Vec<RestaurantItem>, ServerFnError> {
    let mut items = Vec::<RestaurantItem>::new();
    use crate::session_management::*;
    let mut session_service = SessionService::new(pool.clone());
    let manager = pool.get().await?;
    for restaurant in restaurants {
        let ratings = get_ratings_of_restaurant(restaurant.id, &manager).await?;
        let mut favourite = false;
        if let Some(s) = &session {
            let _ = session_service.validate_session(s).await?;
            favourite = is_favourite_restaurant(restaurant.id, s.user_id, &manager).await?;
        }
        items.push(RestaurantItem {
            restaurant,
            ratings,
            favourite,
        })
    }
    Ok(items)
}

#[server]
pub async fn get_user_info(session: LoginSession) -> Result<UserInfo, ServerFnError> {
    use crate::schema::users::dsl as users_schema;
    use crate::session_management::*;
    use axum::extract::*;
    use deadpool_diesel::postgres::Pool;
    let Extension(pool): Extension<Pool> = extract().await?;
    let manager = pool.get().await?;
    let mut session_service = SessionService::new(pool.clone());
    let session = session_service.validate_session(&session).await?;
    let result = manager
        .interact(move |c| {
            users_schema::users
                .filter(users_schema::id.eq(session.user_id))
                .select(UserInfo::as_select())
                .first(c)
        })
        .await??;
    Ok(result)
}

#[server]
pub async fn add_rating(
    request: RatingRequest,
    session: LoginSession,
) -> Result<(), ServerFnError> {
    use crate::schema::ratings::dsl as ratings_schema;
    use crate::session_management::*;
    use axum::extract::*;
    use deadpool_diesel::postgres::Pool;
    use diesel::insert_into;

    let Extension(pool): Extension<Pool> = extract().await?;
    let manager = pool.get().await?;
    let mut session_service = SessionService::new(pool.clone());
    let _ = session_service.validate_session(&session).await?;
    let user = get_user_info(session).await?;
    let rating = NewRating {
        restaurant_id: request.restaurant_id,
        stars: request.stars,
        price: request.price,
        rating: request.rating,
        author: user.username,
    };

    let _ = manager
        .interact(move |c| {
            insert_into(ratings_schema::ratings)
                .values(&rating)
                .execute(c)
        })
        .await??;

    Ok(())
}

#[server]
pub async fn remove_rating(id: Uuid, session: LoginSession) -> Result<(), ServerFnError> {
    use crate::schema::ratings::dsl as ratings_schema;
    use crate::session_management::*;
    use axum::extract::*;
    use deadpool_diesel::postgres::Pool;
    use diesel::delete;

    let Extension(pool): Extension<Pool> = extract().await?;
    let manager = pool.get().await?;
    let mut session_service = SessionService::new(pool.clone());
    let _ = session_service.validate_session(&session).await?;
    let user = get_user_info(session).await?;

    let _ = manager
        .interact(move |c| {
            delete(ratings_schema::ratings)
                .filter(ratings_schema::id::eq(ratings_schema::id, id))
                .filter(ratings_schema::author.eq(user.username))
                .execute(c)
        })
        .await??;

    Ok(())
}

#[server]
pub async fn get_restaurants(
    query: String,
    session: Option<LoginSession>,
) -> Result<Vec<RestaurantItem>, ServerFnError> {
    use crate::schema::restaurants::dsl::*;
    pub use axum::extract::*;
    pub use deadpool_diesel::postgres::Pool;
    let Extension(pool): Extension<Pool> = extract().await?;
    let manager = pool.get().await?;
    let results = manager
        .interact(move |c| {
            restaurants
                .filter(name.ilike(format!("%{}%", query)))
                .load::<Restaurant>(c)
        })
        .await??;
    let items = to_restaurant_items(results, pool, session).await?;
    Ok(items)
}

#[server]
pub async fn register(new_user: RegisterRequest) -> Result<String, ServerFnError> {
    use crate::schema::users::dsl as schema;
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };
    pub use axum::extract::*;
    pub use deadpool_diesel::postgres::Pool;
    pub use diesel::insert_into;
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(new_user.password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    let user = NewUser {
        username: new_user.username,
        email: new_user.email,
        password_hash,
        salt: salt.to_string(),
    };
    let Extension(pool): Extension<Pool> = extract().await?;
    let manager = pool.get().await?;
    let result = manager
        .interact(move |c| insert_into(schema::users).values(user).execute(c))
        .await??;
    Ok(serialize(&result)?)
}

#[server]
pub async fn login(user_login: LoginRequest) -> Result<LoginSession, ServerFnError> {
    use crate::schema::users::dsl as schema;
    use crate::session_management::*;
    use argon2::{Argon2, PasswordHash, PasswordVerifier};
    use axum::body::Body;
    use axum::extract::*;
    use axum::http::{header, HeaderMap};
    use axum::response::Response;
    use axum::{
        extract::Request,
        http::{header::SET_COOKIE, StatusCode},
        middleware::Next,
    };
    use axum_extra::extract::cookie::Cookie;
    use deadpool_diesel::postgres::Pool;
    use std::net::SocketAddr;

    let Extension(pool): Extension<Pool> = extract().await?;
    let manager = pool.get().await?;
    let user = manager
        .interact(move |c| {
            schema::users
                .filter(
                    schema::username
                        .eq(user_login.ident.clone())
                        .or(schema::email.eq(user_login.ident)),
                )
                .first::<User>(c)
        })
        .await??;

    let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
    let valid = Argon2::default().verify_password(user_login.password.as_bytes(), &parsed_hash);

    if valid.is_err() {
        return Err(ServerFnError::new("Invalid Credentials"));
    }

    let ConnectInfo(socket_addr): ConnectInfo<SocketAddr> = extract().await?;
    let ip_address: String = socket_addr.ip().to_string();

    let headers: HeaderMap = extract().await?;

    let user_agent = user_agent(headers);
    let device_info = device_info(user_agent.clone());

    let mut session_service = SessionService::new(pool);
    let session = session_service
        .create_session(user.id, ip_address, device_info, user_agent)
        .await?;

    // let serialized: String = serialize(&session)?;
    //
    // let cookie = Cookie::build(("session", serialized))
    //     .path("/")
    //     .secure(true)
    //     .http_only(true)
    //     .same_site(axum_extra::extract::cookie::SameSite::Strict)
    //     .build();

    Ok(session.into())
}

#[server]
pub async fn logout(session: LoginSession) -> Result<String, ServerFnError> {
    use crate::session_management::*;
    use axum::extract::*;
    use axum::http::StatusCode;
    use deadpool_diesel::postgres::Pool;

    let Extension(pool): Extension<Pool> = extract().await?;
    let mut session_service = SessionService::new(pool);
    let _ = session_service.validate_session(&session).await?;
    let result = session_service.invalidate_session(session.id).await?;

    Ok(serialize(&result)?)
}

#[server]
pub async fn logout_all_other(session: LoginSession) -> Result<String, ServerFnError> {
    use crate::session_management::*;
    use axum::extract::*;
    use axum::http::StatusCode;
    use deadpool_diesel::postgres::Pool;
    let Extension(pool): Extension<Pool> = extract().await?;
    let mut session_service = SessionService::new(pool);
    let _ = session_service.validate_session(&session).await?;
    let result = session_service
        .invalidate_other_sessions(session.id)
        .await?;

    Ok(serialize(&result)?)
}

#[server]
pub async fn add_favourite(id: Uuid, session: LoginSession) -> Result<String, ServerFnError> {
    use crate::schema::favourites::dsl as schema;
    use crate::session_management::*;
    use axum::extract::*;
    use axum::http::StatusCode;
    use deadpool_diesel::postgres::Pool;
    pub use diesel::insert_into;

    let favourite = NewFavourite {
        user_id: session.user_id,
        restaurant_id: id,
    };
    let Extension(pool): Extension<Pool> = extract().await?;
    let mut session_service = SessionService::new(pool.clone());
    let _ = session_service.validate_session(&session).await?;
    let manager = pool.get().await?;
    let _ = manager
        .interact(move |c| {
            insert_into(schema::favourites)
                .values(&favourite)
                .execute(c)
        })
        .await?;
    Ok(String::new())
}

#[server]
pub async fn remove_favourite(id: Uuid, session: LoginSession) -> Result<String, ServerFnError> {
    use crate::schema::favourites::dsl as schema;
    use crate::session_management::*;
    use axum::extract::*;
    use axum::http::StatusCode;
    use deadpool_diesel::postgres::Pool;
    pub use diesel::delete;

    let Extension(pool): Extension<Pool> = extract().await?;
    let mut session_service = SessionService::new(pool.clone());
    let _ = session_service.validate_session(&session).await?;
    let manager = pool.get().await?;
    let _ = manager
        .interact(move |c| {
            delete(
                schema::favourites
                    .filter(schema::restaurant_id.eq(id))
                    .filter(schema::user_id.eq(session.user_id)),
            )
            .execute(c)
        })
        .await??;
    Ok(String::new())
}

#[server]
pub async fn get_favourites(session: LoginSession) -> Result<Vec<RestaurantItem>, ServerFnError> {
    use crate::schema::favourites::dsl as favourites_schema;
    use crate::schema::restaurants::dsl as restaurants_schema;
    use crate::schema::users::dsl as user_schema;
    use crate::session_management::*;
    use axum::extract::*;
    use axum::http::StatusCode;
    use deadpool_diesel::postgres::Pool;

    let Extension(pool): Extension<Pool> = extract().await?;
    let manager = pool.get().await?;
    let mut session_service = SessionService::new(pool.clone());
    let _ = session_service.validate_session(&session).await?;
    let restaurants: Vec<Restaurant> = manager
        .interact(move |c| {
            restaurants_schema::restaurants
                .left_outer_join(favourites_schema::favourites)
                .filter(favourites_schema::user_id.eq(session.user_id))
                .select(Restaurant::as_select())
                .load::<Restaurant>(c)
                .unwrap()
        })
        .await?;
    let items = to_restaurant_items(restaurants, pool, Some(session)).await?;
    Ok(items)
}
