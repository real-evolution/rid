use crate::m20221007_085739_create_users_table::User;

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        assert_eq!(CryptoWallet::Table.to_string(), "crypto_wallet");
        assert_eq!(CryptoWallet::Id.to_string(), "id");
        assert_eq!(CryptoWallet::Address.to_string(), "address");
        assert_eq!(CryptoWallet::Network.to_string(), "network");
        assert_eq!(CryptoWallet::Token.to_string(), "token");
        assert_eq!(CryptoWallet::UserId.to_string(), "user_id");
        assert_eq!(CryptoWallet::CreatedAt.to_string(), "created_at");
        assert_eq!(CryptoWallet::UpdatedAt.to_string(), "updated_at");

        manager
            .create_table(
                Table::create()
                    .table(CryptoWallet::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CryptoWallet::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CryptoWallet::Address).string().not_null())
                    .col(ColumnDef::new(CryptoWallet::Network).string().not_null())
                    .col(ColumnDef::new(CryptoWallet::Token).string().not_null())
                    .col(ColumnDef::new(CryptoWallet::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(CryptoWallet::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CryptoWallet::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(CryptoWallet::Table, CryptoWallet::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CryptoWallet::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum CryptoWallet {
    #[iden = "crypto_wallet"]
    Table,
    Id,
    Address,
    Network,
    Token,
    UserId,
    CreatedAt,
    UpdatedAt,
}
