use rocket::request::{FromRequest, Outcome};
use rocket::{Request, http::Status};

pub struct AuthUser {
    pub user_id: i32,
    pub role: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        // Example: read cookie or header
        let token = req.cookies().get("auth_token");

        if token.is_none() {
            return Outcome::Error((Status::Unauthorized, ()));
        }

        // Normally you would verify JWT or session
        // Example mock user
        let user = AuthUser {
            user_id: 1,
            role: "admin".to_string(),
        };

        Outcome::Success(user)
    }
}