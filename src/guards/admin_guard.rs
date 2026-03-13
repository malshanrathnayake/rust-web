use rocket::request::{FromRequest, Outcome};
use rocket::{Request, http::Status};

use crate::guards::auth_guard::AuthUser;

pub struct AdminUser(pub AuthUser);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        let user = match req.guard::<AuthUser>().await {
            Outcome::Success(user) => user,
            _ => return Outcome::Error((Status::Unauthorized, ())),
        };

        if user.role != "admin" {
            return Outcome::Error((Status::Forbidden, ()));
        }

        Outcome::Success(AdminUser(user))
    }
}