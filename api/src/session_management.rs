use crate::models::prelude::*;
use crate::schema::user_sessions::dsl as schema;
use chrono::Utc;
use deadpool_diesel::postgres::Pool;
use derive_new::new;
use diesel::{delete, insert_into, prelude::*};
use dioxus::prelude::extract;
use uuid::Uuid;

#[derive(new)]
pub(crate) struct SessionService {
    pool: Pool,
}

type Error = dioxus::fullstack::prelude::ServerFnError;

impl SessionService {
    pub async fn create_session(
        &mut self,
        user_id: Uuid,
        ip_address: String,
        device_info: String,
        user_agent: String,
    ) -> Result<UserSession, Error> {
        let now = Utc::now();
        let expires_at = now + chrono::Duration::days(90);

        let new_session = NewUserSession {
            user_id,
            ip_address,
            device_info,
            user_agent,
            expires_at,
        };

        let manager = self.pool.get().await?;

        let session: UserSession = manager
            .interact(move |c| {
                insert_into(schema::user_sessions)
                    .values(&new_session)
                    .get_result(c)
            })
            .await??;

        Ok(session)
    }

    pub async fn validate_session(
        &mut self,
        login_session: &LoginSession,
    ) -> Result<UserSession, Error> {
        let id = login_session.id;
        let header: HeaderMap = extract().await?;
        let user_agent = user_agent(header);
        let ConnectInfo(socket_addr): ConnectInfo<SocketAddr> = extract().await?;
        let ip_address: String = socket_addr.ip().to_string();
        let now = Utc::now().naive_utc();

        let manager = self.pool.get().await?;

        let session: UserSession = manager
            .interact(move |c| {
                schema::user_sessions
                    .filter(schema::id.eq(id))
                    .filter(schema::expires_at.gt(now))
                    .filter(schema::ip_address.eq(ip_address))
                    .filter(schema::user_agent.eq(user_agent))
                    .first(c)
            })
            .await??;
        Ok(session)
    }

    pub async fn invalidate_session(&mut self, id: Uuid) -> Result<(), Error> {
        let manager = self.pool.get().await?;
        manager
            .interact(move |c| delete(schema::user_sessions.filter(schema::id.eq(id))).execute(c))
            .await??;
        Ok(())
    }

    pub async fn invalidate_other_sessions(&mut self, id: Uuid) -> Result<(), Error> {
        let manager = self.pool.get().await?;
        manager
            .interact(move |c| delete(schema::user_sessions.filter(schema::id.ne(id))).execute(c))
            .await??;
        Ok(())
    }
}

use axum::{
    body::Body,
    extract::*,
    http::{header, HeaderMap, Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    Extension,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use std::net::SocketAddr;

pub fn user_agent(headers: HeaderMap) -> String {
    headers
        .get(header::USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown")
        .to_string()
}

pub fn device_info(user_agent: String) -> String {
    if user_agent.contains("Mobile") {
        "Mobile".into()
    } else if user_agent.contains("Tablet") {
        "Tablet".into()
    } else {
        "Desktop".into()
    }
}

use axum::{
    async_trait,
    extract::{ConnectInfo, FromRequestParts},
    http::request::Parts,
};

pub struct AuthSession(pub UserSession);

#[async_trait]
impl<S> FromRequestParts<S> for AuthSession
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let extensions = &parts.extensions.clone();
        // Get the pool from extensions
        let pool = extensions.get::<Pool>().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database connection not found",
        ))?;

        // Get headers
        let headers = parts.headers.clone();

        println!("Headers: {:#?}", headers);

        let session: UserSession = parts
            .extensions
            .get::<CookieJar>()
            .and_then(|jar| jar.get("session"))
            .map(|c| serde_json::from_str(&c.value().to_string()).unwrap())
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid session token"))?;

        println!("Session: {:?}", session);

        let id = session.id;
        let user_agent = user_agent(headers.clone());

        let ConnectInfo(socket_addr): ConnectInfo<SocketAddr> = extract()
            .await
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Could not get the ip address"))?;
        let ip_address: String = socket_addr.ip().to_string();

        // let mut session_service = SessionService::new(pool.clone());
        // let session = session_service
        //     .validate_session(id, ip_address, user_agent)
        //     .await
        //     .map_err(|_| {
        //         (
        //             StatusCode::INTERNAL_SERVER_ERROR,
        //             "Failed to validate session",
        //         )
        //     })?
        //     .ok_or((StatusCode::UNAUTHORIZED, "Invalid session"))?;

        Ok(AuthSession(session))
    }
}
