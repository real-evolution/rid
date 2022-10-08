use crate::m20221007_085739_create_users_table::User;

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        assert_eq!(OAuth2LoginMethod::Table.to_string(), "oauth2_login_method");
        assert_eq!(OAuth2LoginMethod::Id.to_string(), "id");
        assert_eq!(OAuth2LoginMethod::Type.to_string(), "type");
        assert_eq!(OAuth2LoginMethod::AccessToken.to_string(), "access_token");
        assert_eq!(OAuth2LoginMethod::RefreshToken.to_string(), "refresh_token");
        assert_eq!(
            OAuth2LoginMethod::ProviderUserId.to_string(),
            "provider_user_id"
        );
        assert_eq!(OAuth2LoginMethod::IsActive.to_string(), "is_active");
        assert_eq!(OAuth2LoginMethod::UserId.to_string(), "user_id");
        assert_eq!(OAuth2LoginMethod::CreatedAt.to_string(), "created_at");
        assert_eq!(OAuth2LoginMethod::UpdatedAt.to_string(), "updated_at");

        manager
            .create_table(
                Table::create()
                    .table(OAuth2LoginMethod::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OAuth2LoginMethod::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OAuth2LoginMethod::Type).string().not_null())
                    .col(
                        ColumnDef::new(OAuth2LoginMethod::AccessToken)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OAuth2LoginMethod::RefreshToken)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OAuth2LoginMethod::ProviderUserId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OAuth2LoginMethod::IsActive)
                            .boolean()
                            .default(false),
                    )
                    .col(ColumnDef::new(OAuth2LoginMethod::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(OAuth2LoginMethod::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OAuth2LoginMethod::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(OAuth2LoginMethod::Table, OAuth2LoginMethod::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OAuth2LoginMethod::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum OAuth2LoginMethod {
    #[iden = "oauth2_login_method"]
    Table,
    Id,
    Type,
    AccessToken,
    RefreshToken,
    ProviderUserId,
    IsActive,
    UserId,
    CreatedAt,
    UpdatedAt,
}
