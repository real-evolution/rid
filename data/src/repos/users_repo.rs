use super::repo::Repo;

use entity::user::Entity as User;

use uuid::Uuid;

pub trait UsersRepo: Repo<User, Uuid> {}
