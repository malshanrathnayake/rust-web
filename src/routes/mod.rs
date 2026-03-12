pub mod auth;
pub mod admin;
pub mod public;

use rocket::Route;

pub fn auth_routes() -> Vec<Route> {
    let mut routes = Vec::new();

    routes.extend(auth::routes());

    routes
}

pub fn admin_routes() -> Vec<Route> {
    let mut routes = Vec::new();

    routes.extend(admin::routes());

    routes
}

pub fn public_routes() -> Vec<Route> {
    let mut routes = Vec::new();

    routes.extend(public::routes());

    routes
}
