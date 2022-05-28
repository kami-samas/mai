//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "_prisma_migrations")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub checksum: String,
    pub finished_at: Option<DateTimeWithTimeZone>,
    pub migration_name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub logs: Option<String>,
    pub rolled_back_at: Option<DateTimeWithTimeZone>,
    pub started_at: DateTimeWithTimeZone,
    pub applied_steps_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
