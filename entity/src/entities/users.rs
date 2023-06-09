//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "custom(\"uuid\")")]
    pub user_id: String,
    pub username: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub phc_string: Option<String>,
    pub two_factor_enabled: i8,
    #[sea_orm(column_type = "Text", nullable)]
    pub two_factor_secret: Option<String>,
    pub two_factor_enable_date: Option<TimeDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::tokens::Entity")]
    Tokens,
}

impl Related<super::tokens::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tokens.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
