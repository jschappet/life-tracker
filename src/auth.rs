use actix_web::dev::ServiceRequest;

use actix_web::{HttpRequest, Error};


use actix_web_httpauth::extractors::bearer::BearerAuth;

// Used for integration with `actix-web-httpauth`
use actix_web_grants::authorities::AttachAuthorities;

use crate::claims;
use crate::models::User;
use diesel::r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;

pub fn get_user_from_request(
    req: &HttpRequest,
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Option<User> {
    let auth_header = req.headers().get("Authorization")?;
    let token = auth_header.to_str().ok()?;
    let token = token.strip_prefix("Bearer ")?;
    log::debug!("Token: {token}");
    match claims::decode_jwt(token) {
        Ok(claims) => {
            let username = claims.username;
            log::debug!("Authorized Username: {username}");
            // Check for errors before returning user
            match crate::crud::get_user_by_username(conn, username) {
                Ok(user) => Some(user),
                Err(err) => {
                    log::error!("Failed to get user: {}", err);
                    None
                }
            }
        }
        Err(err) => {
            log::error!("Failed to decode JWT: {}", err);
            None
        }
    }
}


pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // We just get permissions from JWT
    log::debug!("validator {:?}", req);
    let result = claims::decode_jwt(credentials.token());
    match result {
        Ok(claims) => {
            log::debug!("validator: {:?}", claims);
            req.attach(claims.permissions);
            Ok(req)
        }
        // required by `actix-web-httpauth` validator signature
        Err(e) => Err((e, req)),
    }
}
