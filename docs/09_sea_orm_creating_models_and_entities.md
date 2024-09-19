# Sea-ORM: Creating Models and Entities

The first model we are going to create is `User`. To create the migration for the `User` model, run this command,

```bash
sea-orm-cli migrate generate create_user_table
```

The general command is basically,

```bash
sea-orm-cli migrate generate <name_of_your_migration>
```

Here, we chose `create_user_table` but you can choose whatever is meaningful to you.

After running the above command, we should have a new migration file inside our `migration` crate. In my case it looked
like this,

```bash
migration
├── Cargo.toml
├── README.md
└── src
    ├── lib.rs
    ├── m20220101_000001_create_table.rs
    ├── m20240822_150235_create_user_table.rs
    └── main.rs
```

**Note:** The `m20240822_150235` part in `m20240822_150235_create_user_table.rs` is auto-generated. The `m20240822` part
is the current date in `yyyymmdd` format starting with an `m` which is probably short for migration, and the `150235`
part is probably the time in UTC when the migration is created. This is just a guess though, I haven't checked the
documentation on this yet. So if you are curious, you might want to check that out.

Let's open up `m20240822_150235_create_user_table.rs` in our editor and let's see what it looks like,

```rust
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(pk_auto(Post::Id))
                    .col(string(Post::Title))
                    .col(string(Post::Text))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}
```

We will remove the `todo!()`s, change the `Post` `enum` to `User`, update the fields for our `User` and then change the
migration script.

First, let's update `User`,

```rust
#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    Firstname,
    Lastname,
    Email,
    IsActive,
    LastLogin,
    DateJoined,
    CreatedAt,
    UpdatedAt,
}
```

Now, let's update the `up()` function in our `MigrationTrait`. The `down()` function is fine as it is. Here's how the
migration file looks like after all the changes,

```rust
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
    IsActive,
    LastLogin,
    DateJoined,
    CreatedAt,
    UpdatedAt,
}
```

To make sure our newly created migration runs with the default migration command, we need to add it to the list of
migrations that needs to be applied. They are located in the `migration/src/lib.rs` file. Here's what it looked like
before,

```rust
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
        ]
    }
}
```

At this moment, this file contains the example migration `m20220101_000001_create_table.rs`, that was created
automatically by Sea-ORM when we ran the migration creation command. We can remove that from this list and also delete
the corresponding migration file as we won't need that. After we add our own migration to this `vec![]`, the `lib.rs`
file looks like this,

```rust
pub use sea_orm_migration::prelude::*;

mod m20240913_193712_create_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240913_193712_create_user_table::Migration),
        ]
    }
}
```

Our migration setup is complete. Now let's run the migration. Run the following command,

```bash
sea-orm-cli migrate up
```

If the migration runs without any issues, you should see something like this at the end,

```bash
Applying all pending migrations
Applying migration 'm20240913_193712_create_user_table'
Migration 'm20240913_193712_create_user_table' has been applied
```

This will result in the creation of a `User` table in our `actix-fullstack` database in our PostgreSQL database under
the `public` schema. You can verify that with the [command line](https://www.postgresql.org/docs/current/app-psql.html)
if you are comfortable or use a tool like **DataGrip** (paid) or **BeeKeeper Studio** (free).

Now we should also create the entity crate,

```bash
sea-orm-cli generate entity -o entity/src
```

If these commands ran successfully, you should see two new directories in your project workspace and they would look
like the following with their contents,

```bash
entity
└── src
    ├── mod.rs
    ├── prelude.rs
    └── user.rs
```

Notice that our `entity` crate doesn't have a `Cargo.toml` file. We need to create that manually.

Create a `Cargo.toml` file inside `entity`,

```bash
entity
├── Cargo.toml  # we just created this one
└── src
    ├── mod.rs
    ├── prelude.rs
    └── user.rs
```

Inside `entity/Cargo.toml` paste the following,

```toml
[package]
name = "entity"
version = "0.1.0"
edition = "2021"
publish = false

[lib]
name = "entity"
path = "src/mod.rs"

[dependencies]
serde = { version = "1.0.125", features = ["derive"] }

[dependencies.sea-orm]
version = "0.12.15"
```

The `entity/src/user.rs` file will be generated automatically based on our `migration`. Let's see what's inside this
file,

```rust
//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub username: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    #[sea_orm(unique)]
    pub email: String,
    pub is_active: Option<bool>,
    pub last_login: Option<DateTime>,
    pub date_joined: Option<DateTime>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
```

We need to add the `entity` and `migration` crates to our project as workspace `members` so that we can work with them.
We also need to add them as `dependencies` to be able to reference them in our project. In the `Cargo.toml` of the main
project, add in the following,

```toml
[workspace]
members = [".", "entity", "migration"]

[dependencies]
// other dependencies
entity = { path = "entity" }
migration = { path = "migration" }
```

We can now start using the `User` model in our application. Except for a rather important field I didn't remember to
include and that is a `password` field. Let's add that and test it out.
