#[macro_use]
extern crate rocket;

mod routes;

use rocket_dyn_templates::Template;
use routes::home::index;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", rocket::fs::FileServer::from("static"))
        .attach(Template::fairing())
}
