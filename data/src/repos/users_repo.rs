use super::Repo;
use crate::models;

use uuid::Uuid;

pub trait UsersRepo: Repo<models::User, Uuid> {}
