use rocket::request::{FromRequest, Outcome};
use rocket::{Request, http::Status};

use crate::guards::auth_guard::AuthenticatedUser;

pub struct AuthorizedUser(pub AuthenticatedUser);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthorizedUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        let auth_result = req.guard::<AuthenticatedUser>().await;

        let user = match auth_result {
            Outcome::Success(user) => user,
            _ => return Outcome::Error((Status::Unauthorized, ())),
        };

        if user.role != "admin" {
            return Outcome::Error((Status::Forbidden, ()));
        }

        Outcome::Success(AuthorizedUser(user))
    }
}