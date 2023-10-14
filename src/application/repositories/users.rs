use crate::application::repositories::Repository;
use crate::domain::entities::users::Users;

pub trait UsersRepository: Repository<Users> {
    fn find_by_email(&self, email: String) -> Option<Users>;
}