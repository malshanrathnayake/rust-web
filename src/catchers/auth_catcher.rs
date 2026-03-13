use rocket::{Request};
use rocket_dyn_templates::{Template, context};

#[catch(401)]
pub fn unauthorized(_req: &Request) -> Template {
    Template::render("errors/401", context! {})
}

#[catch(403)]
pub fn forbidden(_req: &Request) -> Template {
    Template::render("errors/403", context! {})
}

#[catch(404)]
pub fn not_found(_req: &Request) -> Template {
    Template::render("errors/404", context! {})
}