use super::Repo;
use crate::models;

use uuid::Uuid;

pub trait TransactionsRepo: Repo<models::Transaction, Uuid> {}
