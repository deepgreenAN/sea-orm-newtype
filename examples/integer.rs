use sea_orm_newtype::DeriveNewType;

#[derive(Clone, Debug, PartialEq, Eq, DeriveNewType)]
#[sea_orm_newtype(transparent)]
pub struct Integer(i32);

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "foo")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: uuid::Uuid,
    integer: Integer,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

fn main() {}
