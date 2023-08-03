use std::marker::PhantomData;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
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

mod __sea_orm_newtype_id_mod {
    use super::*;

    impl<T> From<Id<T>> for sea_orm_newtype::Value {
        fn from(value: Id<T>) -> Self {
            Into::<Uuid>::into(value).into()
        }
    }

    impl<T> sea_orm_newtype::TryGetable for Id<T> {
        fn try_get_by<I: sea_orm_newtype::sea_orm::ColIdx>(
            res: &sea_orm_newtype::sea_orm::QueryResult,
            index: I,
        ) -> Result<Self, sea_orm_newtype::sea_orm::TryGetError> {
            Ok(Into::<Self>::into(res.try_get_by::<Uuid, I>(index)?))
        }
    }

    impl<T> sea_orm_newtype::ValueType for Id<T> {
        fn try_from(
            v: sea_orm_newtype::Value,
        ) -> Result<Self, sea_orm_newtype::sea_query::ValueTypeErr> {
            Ok(Into::<Self>::into(
                <Uuid as sea_orm_newtype::ValueType>::try_from(v)?,
            ))
        }
        fn type_name() -> String {
            <Uuid as sea_orm_newtype::ValueType>::type_name()
        }
        fn array_type() -> sea_orm_newtype::sea_query::ArrayType {
            <Uuid as sea_orm_newtype::ValueType>::array_type()
        }
        fn column_type() -> sea_orm_newtype::sea_query::ColumnType {
            <Uuid as sea_orm_newtype::ValueType>::column_type()
        }
    }

    impl<T> sea_orm_newtype::Nullable for Id<T> {
        fn null() -> sea_orm_newtype::Value {
            <Uuid as sea_orm_newtype::Nullable>::null()
        }
    }

    impl<T> sea_orm_newtype::TryFromU64 for Id<T> {
        fn try_from_u64(n: u64) -> Result<Self, sea_orm_newtype::sea_orm::DbErr> {
            Ok(Into::<Self>::into(
                <Uuid as sea_orm_newtype::TryFromU64>::try_from_u64(n)?,
            ))
        }
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
