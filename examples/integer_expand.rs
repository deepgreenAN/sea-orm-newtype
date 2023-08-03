#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Integer(i32);

mod __sea_orm_newtype_integer_mod {
    use super::*;

    impl From<Integer> for sea_orm_newtype::Value {
        fn from(value: Integer) -> Self {
            value.0.into()
        }
    }

    impl sea_orm_newtype::TryGetable for Integer {
        fn try_get_by<I: sea_orm_newtype::sea_orm::ColIdx>(
            res: &sea_orm_newtype::sea_orm::QueryResult,
            index: I,
        ) -> Result<Self, sea_orm_newtype::sea_orm::TryGetError> {
            Ok(Integer(res.try_get_by::<i32, I>(index)?))
        }
    }

    impl sea_orm_newtype::ValueType for Integer {
        fn try_from(
            v: sea_orm_newtype::Value,
        ) -> Result<Self, sea_orm_newtype::sea_query::ValueTypeErr> {
            Ok(Integer(<i32 as sea_orm_newtype::ValueType>::try_from(v)?))
        }
        fn type_name() -> String {
            // <i32 as sea_orm_newtype::ValueType>::type_name()
            stringify!(Integer).to_owned()
        }
        fn array_type() -> sea_orm::sea_query::ArrayType {
            <i32 as sea_orm_newtype::ValueType>::array_type()
        }
        fn column_type() -> sea_orm_newtype::sea_query::ColumnType {
            <i32 as sea_orm_newtype::ValueType>::column_type()
        }
    }

    impl sea_orm_newtype::Nullable for Integer {
        fn null() -> sea_orm_newtype::sea_query::Value {
            <i32 as sea_orm_newtype::Nullable>::null()
        }
    }
}

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
