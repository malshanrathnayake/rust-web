use rocket::{Route, form::Form};
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};

#[derive(FromForm)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[get("/login")]
pub fn login_page() -> Template {
    Template::render("auth/login", context! {})
}

#[post("/login", data = "<form>")]
pub fn login(form: Form<LoginForm>) -> Redirect {
    let data = form.into_inner();

    if data.username == "admin" && data.password == "1234" {
        Redirect::to("/admin/")
    } else {
        Redirect::to("/auth/login")
    }
}

pub fn routes() -> Vec<Route> {
    routes![login_page, login]
}