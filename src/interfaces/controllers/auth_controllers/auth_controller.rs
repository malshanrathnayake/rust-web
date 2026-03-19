use rocket::form::Form;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};
use std::collections::HashMap;

use crate::domain::entities::system::login;
use crate::infrastructure::unit_of_work::UniOfWork;
use crate::infrastructure::repositories::system::user_repository_impl::UserRepositoryImpl;
use crate::application::services::system::user_service::UserService;

pub async fn login_page() -> Template {
    Template::render("auth/login", context! {errors: Option::<HashMap<&str, &str>>::None})
}

pub async fn login_post(form: Form<login::Login>) -> Result<Redirect, Template> {
    
    let data = form.into_inner();
    let mut errors = data.validate();

    if !errors.is_empty() {
        return Err(Template::render("auth/login", context! { errors: &errors, data: &data }));
    }

    let mut uow = UniOfWork { connection: None };

    let mut repo = UserRepositoryImpl {
        unit_of_work: &mut uow,
    };

    let mut service = UserService {
        user_repository: &mut repo,
    };

    let user = service
        .get_user_by_email("malshan.edu@gmail.com")
        .await;

    match user {
        Some(u) => {
            Ok(Redirect::to("/admin/"))
        }
        None => {
            errors.insert("username", "Invalid username or password");
            errors.insert("password", "Invalid username or password");

            Err(Template::render("auth/login", context! { errors: errors }))
        }
    }

}