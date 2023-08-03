use std::marker::PhantomData;
use uuid::Uuid;

use sea_orm_newtype::DeriveNewType;

#[derive(Debug, Clone, PartialEq, DeriveNewType)]
#[sea_orm_newtype(from_into = "Uuid", primary_key)]
pub struct Id<T>(Uuid, PhantomData<T>);

impl<T> From<Uuid> for Id<T> {
    fn from(id: Uuid) -> Id<T> {
        Id(id, PhantomData)
    }
}

impl<T> From<Id<T>> for Uuid {
    fn from(value: Id<T>) -> Self {
        value.0
    }
}

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ModelId;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "foo")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: Id<ModelId>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

fn main() {}
