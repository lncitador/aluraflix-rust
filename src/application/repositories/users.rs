use crate::application::repositories::Repository;
use crate::domain::entities::users::Users;
use crate::domain::value_objects::email::EmailEntity;

pub trait UsersRepository: Repository<Users> {
    fn find_by_email(&self, email: EmailEntity) -> Option<Users>;
}