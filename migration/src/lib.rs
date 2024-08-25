pub use sea_orm_migration::prelude::*;

mod m20240818_160752_create_user_table;
mod m20240822_195452_alter_user_table_add_password_field;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240818_160752_create_user_table::Migration),
            Box::new(m20240822_195452_alter_user_table_add_password_field::Migration)
        ]
    }
}
