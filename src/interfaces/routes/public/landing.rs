use rocket::Route;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index() -> Template {
    Template::render("public/index", context! {})
}

pub fn routes() -> Vec<Route> {
    routes![index]
}