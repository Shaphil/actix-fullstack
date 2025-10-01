use crate::sea_orm::DbBackend;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();

        let alter_table_query = Table::alter()
            .table(User::Table)
            .add_column_if_not_exists(ColumnDef::new(User::IsAdmin).boolean().default(false))
            .add_column_if_not_exists(ColumnDef::new(User::IsSuperadmin).boolean().default(false))
            .to_owned();

        match backend {
            DbBackend::MySql | DbBackend::Postgres => manager.alter_table(alter_table_query).await,

            DbBackend::Sqlite => {
                // To support SQLite, we must execute each column addition separately.

                // Add IsAdmin column
                manager
                    .alter_table(
                        Table::alter()
                            .table(User::Table)
                            .add_column_if_not_exists(
                                ColumnDef::new(User::IsAdmin).boolean().default(false),
                            )
                            .to_owned(),
                    )
                    .await?;

                // Add IsSuperadmin column
                manager
                    .alter_table(
                        Table::alter()
                            .table(User::Table)
                            .add_column_if_not_exists(
                                ColumnDef::new(User::IsSuperadmin).boolean().default(false),
                            )
                            .to_owned(),
                    )
                    .await
            }
        }
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
    IsAdmin,
    IsSuperadmin,
}
