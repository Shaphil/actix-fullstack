use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(User::Username).string().not_null().unique_key())
                    .col(ColumnDef::new(User::Firstname).string())
                    .col(ColumnDef::new(User::Lastname).string())
                    .col(ColumnDef::new(User::Email).string().unique_key().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::IsActive).boolean().default(false))
                    .col(ColumnDef::new(User::LastLogin).date_time())
                    .col(ColumnDef::new(User::DateJoined).date_time())
                    .col(ColumnDef::new(User::CreatedAt).date_time())
                    .col(ColumnDef::new(User::UpdatedAt).date_time())
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

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    Firstname,
    Lastname,
    Email,
    Password,
    IsActive,
    LastLogin,
    DateJoined,
    CreatedAt,
    UpdatedAt,
}
