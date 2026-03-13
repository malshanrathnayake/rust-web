use rocket::{Route, form::Form};
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};

use crate::controllers::auth_controllers::auth_controller;
use crate::models::auth_models::login_form;

#[get("/login")]
pub fn login_page() -> Template {
    auth_controller::login_page()
}

#[post("/login", data = "<form>")]
pub fn login(form: Form<login_form::LoginForm>) -> Result<Redirect, Template> {
    auth_controller::login(form)
}

pub fn routes() -> Vec<Route> {
    routes![login_page, login]
}