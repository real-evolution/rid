use super::Repo;
use crate::models;

use uuid::Uuid;

pub trait OAuth2LoginMethodsRepo: Repo<models::OAuth2LoginMethod, Uuid> {}
