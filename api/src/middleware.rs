use crate::responses::*;
use axum::{
    body,
    extract::Request,
    http::{header::SET_COOKIE, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};

use axum_extra::extract::cookie::{Cookie, CookieJar};

pub async fn cookie_middleware(request: Request, next: Next) -> Result<Response, StatusCode> {
    let response = next.run(request).await;

    let (parts, body) = response.into_parts();
    let bytes = body::to_bytes(body, usize::MAX)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("Body: {:#?}", bytes);

    if let Ok(set_cookie_response) = serde_json::from_slice::<SetCookieResponse>(&bytes) {
        let cookie = Cookie::parse(set_cookie_response.cookie.clone())
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let mut r = Json(set_cookie_response).into_response();

        r.headers_mut()
            .append(SET_COOKIE, cookie.to_string().parse().unwrap());

        println!("New Headers: {:#?}", r.headers());

        return Ok(r);
    } else {
        Ok(Response::from_parts(parts, bytes.into()))
    }
}
