use rocket::Route;
use rocket_dyn_templates::{context, Template};
use crate::interfaces::guards::admin_guard::AuthorizedUser;

#[get("/")]
pub fn index(_authorized_user: AuthorizedUser) -> Template {
    Template::render("admin/index", context! {})
}

pub fn routes() -> Vec<Route> {
    routes![index]
}