pub use sea_orm_migration::prelude::*;

mod m20240913_193712_create_user_table;
mod m20240916_144220_alter_user_table_add_admin_fields;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240913_193712_create_user_table::Migration),
            Box::new(m20240916_144220_alter_user_table_add_admin_fields::Migration),
        ]
    }
}
