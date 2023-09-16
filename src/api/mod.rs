use std::net::SocketAddr;

use anyhow::Context;
use axum::extract::State;
use axum::headers::authorization::Basic;
use axum::headers::Authorization;
use axum::http::{Method, Request};
use axum::middleware::{from_fn_with_state, Next};
use axum::response::Response;
use axum::routing::get;
use axum::{http, Router, TypedHeader};
use diesel::prelude::*;
use diesel::{Identifiable, QueryDsl, Queryable, Selectable, SqliteConnection};
use http::StatusCode;
use log::info;
use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::api::routes::*;
use crate::db::DbPool;
use crate::{Error, Result};

pub mod dto;
pub mod routes;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
}

pub async fn run(pool: DbPool) -> Result<()> {
    let config = super::settings::get_config()?;
    let _dev = config.get_bool("dev")?;
    let state = AppState { pool };

    let cors = tower_http::cors::CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(tower_http::cors::Any);

    let app = Router::new()
        .route("/api/version", get(get_version))
        .route("/api/artists", get(get_artists))
        .route("/api/artists/:id", get(get_artist))
        .route("/api/albums", get(get_albums))
        .route("/api/albums/:id", get(get_album))
        .route("/api/tracks", get(get_tracks))
        .route("/api/tracks/:id", get(get_track))
        .route("/api/play/:id", get(play_track))
        .route("/api/art/:id", get(get_art))
        .route("/api/search/:query", get(unimplemented))
        .route_layer(ServiceBuilder::new().layer(from_fn_with_state(state.clone(), auth)))
        .route("/api/users/create", get(unimplemented))
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let (mut host, port) = (config.get_string("host")?, config.get_int("port")? as u16);
    if host == *"localhost" {
        host = "127.0.0.1".to_string()
    }
    info!("Starting API webserver on {}:{}", host, port);

    axum::Server::bind(&SocketAddr::new(host.parse()?, port))
        .serve(app.into_make_service())
        .await
        .context("Failed to bind webserver")?;

    Ok(())
}

async fn auth<B>(
    state: State<AppState>,
    auth: TypedHeader<Authorization<Basic>>,
    request: Request<B>,
    next: Next<B>,
) -> std::result::Result<Response, StatusCode>
where
    B: Send + 'static,
{
    let mut conn = state
        .pool
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if User::check(auth.username(), auth.password(), &mut conn).is_none() {
        Err(StatusCode::UNAUTHORIZED)
    } else {
        let response = next.run(request).await;

        Ok(response)
    }
}

#[derive(
    Debug, PartialEq, Eq, Selectable, Queryable, QueryableByName, Insertable, Identifiable,
)]
#[diesel(table_name = crate::db::schema::users)]
struct User {
    id: i32,
    username: String,
    password: String,
    salt: String,
    email: Option<String>,
    role: String,
}

impl User {
    fn new(name: &str, pw: &str, conn: &mut SqliteConnection) -> Result<Self> {
        use crate::db::schema::users::*;

        if table
            .select(User::as_select())
            .filter(username.eq(name))
            .first(conn)
            .is_ok()
        {
            return Err(Error::msg("Account already exists"));
        };

        let argon2 = argon2::Argon2::default();
        let generated_salt = argon2::password_hash::SaltString::generate(
            &mut argon2::password_hash::rand_core::OsRng,
        );
        let salt_string = generated_salt.as_str();

        let password_hash = argon2
            .hash_password(pw.as_bytes(), &generated_salt)
            .unwrap()
            .to_string();

        Ok(diesel::insert_into(table)
            .values(&(
                username.eq(name),
                password.eq(password_hash),
                salt.eq(salt_string),
            ))
            .on_conflict_do_nothing()
            .get_result::<User>(conn)?)
    }
    fn check(name: &str, pw: &str, conn: &mut SqliteConnection) -> Option<Self> {
        use crate::db::schema::users::*;

        let user = table
            .select(User::as_select())
            .filter(username.like(format!("%{name}%")))
            .first(conn)
            .ok()?;

        let argon2 = argon2::Argon2::default();
        let password_hash = PasswordHash::new(user.password.as_str()).ok()?;

        if argon2
            .verify_password(pw.as_bytes(), &password_hash)
            .is_ok()
        {
            Some(user)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_tests() {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();
        crate::db::run_migrations(&mut conn).unwrap();

        let user = User::new("kitty", "verysecurepassword", &mut conn);
        // try creating the same user again
        let user2 = User::new("kitty", "anotherpawssword", &mut conn);

        assert!(user.is_ok());
        assert!(user2.is_err());

        assert!(User::check("kitty", "verysecurepassword", &mut conn).is_some());
        assert!(User::check("kitty", "bla", &mut conn).is_none());
    }
}
