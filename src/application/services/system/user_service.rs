use crate::domain::entities::system::user::User;
use crate::domain::repositories::system::user_repository::UserRepository;

pub struct UserService<'a> {
    pub user_repository: &'a mut (dyn UserRepository + Send),
}

impl<'a> UserService<'a> {

    pub async fn get_user_by_email(&mut self, email: &str) -> Option<User> {
        self.user_repository.get_user_by_email(email).await
    }
}