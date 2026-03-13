pub mod landing;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    let mut routes = Vec::new();

    routes.extend(landing::routes());

    routes
}