pub mod admin;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    let mut routes = Vec::new();

    routes.extend(admin::routes());

    routes
}