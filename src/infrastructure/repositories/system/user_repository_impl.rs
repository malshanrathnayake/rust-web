use crate::domain::entities::system::user::User;
use crate::domain::repositories::system::user_repository::UserRepository;
use crate::infrastructure::unit_of_work::UniOfWork;

pub struct UserRepositoryImpl<'a> {
    pub unit_of_work: &'a mut UniOfWork,
}

#[async_trait::async_trait]
impl<'a> UserRepository for UserRepositoryImpl<'a> {
    async fn get_user_by_email(&mut self, email: &str) -> Option<User> {
        let query = "EXEC GetUser @email = @P1";

        let users: Vec<User> = match self.unit_of_work.retrieve_data(query, &[&email]).await {
            Ok(users) => users,
            Err(e) => {
                println!("retrieve_data error: {}", e);
                return None;
            }
        };

        users.into_iter().next()
    }
}
