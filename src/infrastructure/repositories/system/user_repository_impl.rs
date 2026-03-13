use crate::domain::entities::system::user::User;
use crate::domain::repositories::system::user_repository::UserRepository;
use crate::infrastructure::unit_of_work::unit_of_work::UniOfWork;

pub struct UserRepositoryImpl {
    pub unit_of_work: UniOfWork,
}

#[async_trait::async_trait]
impl UserRepository for UserRepositoryImpl {

    async fn get_user_by_email(&mut self, email: &str) -> Option<User> {

        let procedure = format!("GetUser @email='{}'", email);

        let users: Vec<User> = self.unit_of_work.retrieve_data(&procedure).await.unwrap_or(vec![]);

        users.into_iter().next()
    }
}