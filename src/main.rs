#[macro_use]
extern crate rocket;

mod domain;
mod interfaces;
mod catchers;
mod infrastructure;

use rocket_dyn_templates::Template;
use crate::interfaces::routes;
use infrastructure::database::connection::create_connection;

#[launch]
async fn rocket() -> _ {

    let con = create_connection()
        .await
        .expect("Failed to connect to database");

    rocket::build()
        .manage(con)
        .mount("/", routes::public_routes())
        .mount("/auth", routes::auth_routes())
        .mount("/admin", routes::admin_routes())
        .mount("/static", rocket::fs::FileServer::from("static"))
        .attach(Template::fairing())
}
