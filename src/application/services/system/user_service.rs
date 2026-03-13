use crate::domain::repositories::system::user_repository::UserRepository;

pub struct UserService<R: UserRepository> {
    pub user_repository: R,
}

impl<R: UserRepository> UserService<R> {

    pub fn get_user_by_email(&self, email: &str) -> Option<User> {
        let user = self.user_repository.get_user_by_email(email);
        return user
    }
}