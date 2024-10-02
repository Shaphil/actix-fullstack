# Adding missing field to a Model

There are two ways to add a missing field to a model,

1. The general best practice - \*Create a new migration to **Alter the Table\***
2. The completely crazy method - **\*Destroy** everything, Edit the existing migration and start over\*

I am going to show both of the methods here. But the rule of the universe is that, **if you have data in your database
table, you should never alter an existing migration but create a new migration to alter the table**. This is because,
most ORMs track which migrations have been applied in a separate table, which is called `seaql_migrations` in the case
of Sea-ORM, atleast in the version ("1.0.1") I used. Since this table lists the migrations that have already been
applied, no matter how many changes you make to your existing migration, Sea-ORM wouldn't bother looking at that. When
you run `sea-orm-cli migrate up` even after changing you existing migration, Sea-ORM will look at the `seaql_migrations`
table, see that your migration has been applied once and **ignore** your command.

## Alter Table

We will first look at how we can alter our existing `User` table which is what you should do if you have data in your
table. Also, this gives you a chance to learn how to write migration for altering a table.

First, create a new migration for this purpose,

```bash
sea-orm-cli migrate generate alter_user_table_add_password_field
```

This will create a file like `m20240822_195452_alter_user_table_add_password_field.rs` in the `migration/src` directory.

The migration to alter the `User` table and add the missing `password` field looks like this,

```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(Table::alter()
                .table(User::Table)
                .add_column_if_not_exists(
                    ColumnDef::new(User::Password)
                        .string()
                        .not_null()
                        .default(String::new())
                ).to_owned()
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
    Password,
}
```

Notice the `.default(String::new())` part in this migration. This sets an empty string as the default value. You should
choose a suitable default that serves your application requirements.

After that, we should add it to the list of migrations in `migration/src/lib.rs`,

```rust
pub use sea_orm_migration::prelude::*;

mod m20240818_160752_create_user_table;
mod m20240822_195452_alter_user_table_add_password_field;   // just added


pub struct Migrator;

@@ - 9, 6 + 11, 7 @ @ impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240818_160752_create_user_table::Migration),
            Box::new(m20240822_195452_alter_user_table_add_password_field::Migration)   // just added
        ]
    }
}
```

For this change to take effect, simply run the `sea-orm-cli migrate up` command again. This is one approach.

## Editing existing migration

Though this is **not recommended**, I did it because my app is in development state and I don't care about the dummy
data. This process involves **dropping you existing database** (which is irreversible) so do it at your own risk.

This is what I did. I simply dropped the entire `actix-fullstack` database, modified my existing migration and ran the
`sea-orm-cli migrate up` command again. Here's the modification,

```rust
// existing code ommited
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    // existing migration code
                    .col(ColumnDef::new(User::Password).string().not_null())
                    // ...
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    // other fields
    Password,
}
```

Whichever approach you take, at this point you should now have a `password` field in your `User` model. If we want our
`User` to be able to login to our app, we would need a `password`.
