use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        assert_eq!(User::Table.to_string(), "user");
        assert_eq!(User::Id.to_string(), "id");
        assert_eq!(User::IsActive.to_string(), "is_active");
        assert_eq!(User::DisplayName.to_string(), "display_name");
        assert_eq!(User::ProfilePicUrl.to_string(), "profile_pic_url");
        assert_eq!(User::CachedTotalRid.to_string(), "cached_total_rid");
        assert_eq!(User::CreatedAt.to_string(), "created_at");
        assert_eq!(User::UpdatedAt.to_string(), "updated_at");

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(User::IsActive).boolean().default(false))
                    .col(ColumnDef::new(User::DisplayName).string().null())
                    .col(ColumnDef::new(User::ProfilePicUrl).string().null())
                    .col(ColumnDef::new(User::CachedTotalRid).decimal().default(0.0))
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    IsActive,
    DisplayName,
    ProfilePicUrl,
    CachedTotalRid,
    CreatedAt,
    UpdatedAt,
}
