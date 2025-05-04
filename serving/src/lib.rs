use api::middleware::*;
use api::prelude::*;

use axum::extract::*;
use axum::middleware;
use axum::response::IntoResponse;
use axum::routing::*;
use axum::Router;
use deadpool_diesel::{
    postgres::{Manager, Pool},
    Runtime::Tokio1,
};
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dioxus::fullstack::prelude::*;
use dioxus::fullstack::server::DioxusRouterExt;
use dioxus::logger::tracing::*;
use dotenv::dotenv;
use std::env;
use std::net::*;
use tokio::net::TcpSocket;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[tokio::main]
pub async fn serve() {
    dotenv().ok();

    dioxus::logger::initialize_default();

    // ############################## DATABASE ############################## //

    let database_url = env::var("DATABASE_URL").expect("Please provide a correct DATABASE_URL.");

    let manager = Manager::new(database_url, Tokio1);
    let pool = Pool::builder(manager).build().unwrap();

    {
        let manager = pool.get().await.unwrap();
        manager
            .interact(|c| c.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    // ############################## DIOXUS APPLICATION SERVER ############################## //

    // let serve_config = ServeConfigBuilder::new()
    //     .build()
    //     .expect("Index html not found: Please provide an index.html");

    // ############################## AXUM APP ############################## //

    let app = Router::new()
        .register_server_functions()
        .layer(Extension(pool))
        .layer(middleware::from_fn(cookie_middleware))
        .with_state(());

    let host = env::var("INTERN_SV_HOST")
        .expect("An intern server host must be specified")
        .parse::<Ipv4Addr>()
        .expect("Please provide an intern ipv4 adress as host");
    let port = env::var("INTERN_SV_PORT")
        .expect("An intern server port must be specified")
        .parse::<u16>()
        .expect("Please provide an integer as intern port");
    let addr = SocketAddr::new(IpAddr::V4(host), port);
    info!("Adress: {addr}");

    let socket = TcpSocket::new_v4().expect("Could not crate tcp socket");
    socket
        .set_reuseaddr(true)
        .expect("Could not set feature to reuse adress of tcp socket");
    socket
        .bind(addr)
        .expect("Could not add adress to tcp socket");

    let listener = socket.listen(10).expect("Could not establish tcp listener");

    info!("Serving...");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Could not serve");
}
