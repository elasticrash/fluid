//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "schedule")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub expression: String,
    pub plan: i32,
    pub start: DateTime,
    pub finish: Option<DateTime>,
    pub process: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::call_back::Entity")]
    CallBack,
    #[sea_orm(has_many = "super::execution::Entity")]
    Execution,
}

impl Related<super::call_back::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CallBack.def()
    }
}

impl Related<super::execution::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Execution.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
