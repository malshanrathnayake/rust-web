use rocket::form::Form;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};
use std::collections::HashMap;

use crate::domain::entities::system::login;

pub fn login_page() -> Template {
    Template::render("auth/login", context! {errors: Option::<HashMap<&str, &str>>::None})
}

pub fn login_post(form: Form<login::Login>) -> Result<Redirect, Template> {
    
    let data = form.into_inner();
    let mut errors = data.validate();

    if !errors.is_empty() {
        return Err(Template::render("auth/login", context! { errors: &errors, data: &data }));
        
    }

    if data.username == "admin" && data.password == "1234" {
        Ok(Redirect::to("/admin/"))
    } else {

        errors.entry("username").and_modify(|e| *e = "Invalid username or password").or_insert("Invalid username or password");
        errors.entry("password").and_modify(|e| *e = "Invalid username or password").or_insert("Invalid username or password");

        return Err(Template::render("auth/login", context! {errors: errors}));
    }

}