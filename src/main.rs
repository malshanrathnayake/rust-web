#[macro_use]
extern crate rocket;

mod routes;
mod guards;
mod catchers;

use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes::public_routes())
        .mount("/auth", routes::auth_routes())
        .mount("/admin", routes::admin_routes())
        .register("/", catchers![catchers::auth_catcher::unauthorized, catchers::auth_catcher::forbidden, catchers::auth_catcher::not_found])
        .mount("/static", rocket::fs::FileServer::from("static"))
        .attach(Template::fairing())
}
