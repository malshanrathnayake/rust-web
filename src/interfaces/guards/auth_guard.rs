use rocket::request::{FromRequest, Outcome};
use rocket::{Request, http::Status};

pub struct AuthenticatedUser {
    pub user_id: i32,
    pub role: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        let token = req.cookies().get("auth_token");

        if token.is_none() {
            return Outcome::Error((Status::Unauthorized, ()));
        }

        let user = AuthenticatedUser {
            user_id: 1,
            role: String::from("admin"),
        };

        Outcome::Success(user)
    }
}