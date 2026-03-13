use rocket::Route;
use rocket_dyn_templates::{context, Template};
use crate::guards::admin_guard::AdminUser;

#[get("/")]
pub fn index(_admin: AdminUser) -> Template {
    Template::render("demo/body", context! {})
}

pub fn routes() -> Vec<Route> {
    routes![index]
}