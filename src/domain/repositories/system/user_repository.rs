use crate::domain::entities::system::user::User;

#[async_trait::async_trait]
pub trait UserRepository: Send {
    async fn get_user_by_email(&mut self, email: &str) -> Option<User>;
}