pub mod auth;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    let mut routes = Vec::new();

    routes.extend(auth::routes());

    routes
}