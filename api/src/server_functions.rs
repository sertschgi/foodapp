mod deps {
    pub use crate::models::prelude::*;
    pub use crate::requests::*;
    pub use diesel::prelude::*;
    pub use dioxus::prelude::*;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::from_str as deserialize;
    pub use serde_json::to_string as serialize;
    pub use uuid::Uuid;
}
use deps::*;

#[server(GetServerPing)]
pub async fn get_server_ping() -> Result<String, ServerFnError> {
    Ok("Ping from Server".to_string())
}

#[derive(Serialize, Deserialize)]
pub struct RestaurantItem {
    pub restaurant: Restaurant,
    pub ratings: Vec<Rating>,
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
                .expect("Could not query ratings!")
        })
        .await?;
    Ok(result)
}

#[server(GetRestaurants)]
pub async fn get_restaurants(query: String) -> Result<String, ServerFnError> {
    use crate::schema::restaurants::dsl::*;
    pub use axum::extract::*;
    pub use deadpool_diesel::postgres::Pool;
    let Extension(pool): Extension<Pool> = extract().await?;
    let manager = pool.get().await?;
    let result = manager
        .interact(move |c| {
            restaurants
                .filter(name.ilike(format!("%{}%", query)))
                .load::<Restaurant>(c)
                .expect("Could not query restaurants!")
        })
        .await?;
    let mut items = Vec::<RestaurantItem>::new();
    for restaurant in result {
        let ratings = get_ratings_of_restaurant(restaurant.id, &manager).await?;
        items.push(RestaurantItem {
            restaurant,
            ratings,
        })
    }
    Ok(serialize(&items)?)
}

#[server(Register)]
pub async fn register(credentials: String) -> Result<String, ServerFnError> {
    use crate::schema::users::dsl as schema;
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };
    pub use axum::extract::*;
    pub use deadpool_diesel::postgres::Pool;
    pub use diesel::insert_into;
    let new_user: RegisterRequest = deserialize(&credentials)?;
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
        .interact(move |c| {
            insert_into(schema::users)
                .values(user)
                .execute(c)
                .expect("Could not insert into restaurants!")
        })
        .await?;
    Ok(serialize(&result)?)
}

#[server(Login)]
pub async fn login(credentials: String) -> Result<String, ServerFnError> {
    use crate::schema::users::dsl as schema;
    use crate::session_management::*;
    use argon2::{Argon2, PasswordHash, PasswordVerifier};
    use axum::extract::*;
    use axum::http::{header, HeaderMap};
    use deadpool_diesel::postgres::Pool;
    use ipnet::*;
    use std::net::SocketAddr;

    let user_login: LoginRequest = deserialize(&credentials)?;

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
                .expect("Could not query ratings!")
        })
        .await?;

    let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
    let valid = Argon2::default().verify_password(user_login.password.as_bytes(), &parsed_hash);

    if valid.is_err() {
        return Err(ServerFnError::new("Invalid Credentials"));
    }

    let ConnectInfo(socker_addr): ConnectInfo<SocketAddr> = extract().await?;
    let ip_address: IpNet = socker_addr.to_string().parse()?;

    let headers: HeaderMap = extract().await?;

    let user_agent = user_agent(headers);
    let device_info = device_info(user_agent);

    let mut session_service = SessionService::new(pool);
    let session = session_service
        .create_session(user.id, ip_address, device_info, user_agent)
        .await?;

    Ok(serialize(&session)?)
}

#[server(Logout)]
pub async fn logout(id: Uuid) -> Result<String, ServerFnError> {
    use crate::session_management::*;
    use axum::extract::*;
    use deadpool_diesel::postgres::Pool;

    let Extension(pool): Extension<Pool> = extract().await?;
    let mut session_service = SessionService::new(pool);
    let result = session_service.invalidate_session(id).await?;

    Ok(serialize(&result)?)
}

#[server(LogoutAllOther)]
pub async fn logout_all_other(id: Uuid) -> Result<String, ServerFnError> {
    use crate::session_management::*;
    use axum::extract::*;
    use deadpool_diesel::postgres::Pool;

    let Extension(pool): Extension<Pool> = extract().await?;
    let mut session_service = SessionService::new(pool);
    let result = session_service.invalidate_other_sessions(id).await?;

    Ok(serialize(&result)?)
}

#[server(AddFavourite)]
pub async fn add_favourite() -> Result<String, ServerFnError> {
    todo!()
}

#[server(GetFavourites)]
pub async fn get_favourites() -> Result<String, ServerFnError> {
    use crate::schema::favourites::dsl as favourites_schema;
    use crate::schema::restaurants::dsl as resaturants_schema;
    use crate::schema::users::dsl as user_schema;
    use crate::session_management::*;
    use axum::extract::*;
    use axum::http::StatusCode;
    use deadpool_diesel::postgres::Pool;

    let AuthSession(session) = extract()
        .await
        .map_err(|e: (StatusCode, &'static str)| ServerFnError::new(e.1))?;

    let Extension(pool): Extension<Pool> = extract().await?;
    let manager = pool.get().await?;
    let user = manager
        .interact(move |c| {
            user_schema::users
                .filter(user_schema::id.eq(session.user_id))
                .first::<User>(c)
                .expect("Could not query ratings!")
        })
        .await?;

    todo!()
}
