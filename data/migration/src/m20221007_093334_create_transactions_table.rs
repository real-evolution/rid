use crate::m20221007_085739_create_users_table::User;

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        assert_eq!(Transaction::Table.to_string(), "transaction");
        assert_eq!(Transaction::Id.to_string(), "id");
        assert_eq!(Transaction::Amount.to_string(), "amount");
        assert_eq!(Transaction::Source.to_string(), "source");
        assert_eq!(Transaction::Currency.to_string(), "currency");
        assert_eq!(Transaction::Extra.to_string(), "extra");
        assert_eq!(Transaction::IsCompleted.to_string(), "is_completed");
        assert_eq!(Transaction::UserId.to_string(), "user_id");
        assert_eq!(Transaction::CreatedAt.to_string(), "created_at");
        assert_eq!(Transaction::UpdatedAt.to_string(), "updated_at");

        manager
            .create_table(
                Table::create()
                    .table(Transaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Transaction::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Transaction::Amount).decimal().not_null())
                    .col(ColumnDef::new(Transaction::Source).string().not_null())
                    .col(ColumnDef::new(Transaction::Currency).string().not_null())
                    .col(ColumnDef::new(Transaction::Extra).string().null())
                    .col(ColumnDef::new(Transaction::IsCompleted).boolean().null())
                    .col(ColumnDef::new(Transaction::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(Transaction::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Transaction::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Transaction::Table, Transaction::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Transaction::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Transaction {
    Table,
    Id,
    Amount,
    Source,
    Currency,
    IsCompleted,
    Extra,
    UserId,
    CreatedAt,
    UpdatedAt,
}
