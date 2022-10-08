pub use sea_orm_migration::prelude::*;

mod m20221007_085739_create_users_table;
mod m20221007_093301_create_login_methods_table;
mod m20221007_093322_create_crypto_wallets_table;
mod m20221007_093334_create_transactions_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221007_085739_create_users_table::Migration),
            Box::new(m20221007_093301_create_login_methods_table::Migration),
            Box::new(m20221007_093322_create_crypto_wallets_table::Migration),
            Box::new(m20221007_093334_create_transactions_table::Migration),
        ]
    }
}
