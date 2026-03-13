use rocket::{Route, form::Form};
use rocket::response::Redirect;
use rocket_dyn_templates::{Template};

use crate::interfaces::controllers::auth_controllers::auth_controller;
use crate::domain::entities::system::login;

#[get("/login")]
pub fn login_page() -> Template {
    auth_controller::login_page()
}

#[post("/login", data = "<form>")]
pub fn login_post(form: Form<login::Login>) -> Result<Redirect, Template> {
    auth_controller::login_post(form)
}

pub fn routes() -> Vec<Route> {
    routes![login_page, login_post]
}