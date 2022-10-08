use super::Repo;
use crate::models;

use uuid::Uuid;

pub trait CryptoWalletsRepo: Repo<models::CryptoWallet, Uuid> {}
