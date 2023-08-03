use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmailAddress(email_address::EmailAddress);

#[derive(Debug)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, stringify!(ParseError))
    }
}

impl std::error::Error for ParseError {}

impl TryFrom<String> for EmailAddress {
    type Error = ParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(EmailAddress(
            email_address::EmailAddress::from_str(&value).map_err(|_| ParseError)?,
        ))
    }
}

/// ParseErrorがトレイト境界を満たしているかチェック
#[allow(non_camel_case_types)]
struct __Assersion_EmailAddress_TryFrom
where
    <EmailAddress as TryFrom<String>>::Error: std::error::Error;

impl From<EmailAddress> for String {
    fn from(value: EmailAddress) -> Self {
        value.0.to_string()
    }
}

mod __sea_orm_newtype_email_address_mod {
    use super::*;

    impl From<EmailAddress> for sea_orm_newtype::Value {
        fn from(value: EmailAddress) -> Self {
            Into::<String>::into(value).into()
        }
    }

    impl sea_orm_newtype::TryGetable for EmailAddress {
        fn try_get_by<I: sea_orm_newtype::sea_orm::ColIdx>(
            res: &sea_orm_newtype::sea_orm::QueryResult,
            index: I,
        ) -> Result<Self, sea_orm_newtype::sea_orm::TryGetError> {
            Ok(
                TryInto::<Self>::try_into(res.try_get_by::<String, I>(index)?)
                    .map_err(|e| sea_orm_newtype::sea_orm::DbErr::Custom(e.to_string()))?,
            )
        }
    }

    impl sea_orm_newtype::ValueType for EmailAddress {
        fn try_from(
            v: sea_orm_newtype::Value,
        ) -> Result<Self, sea_orm_newtype::sea_query::ValueTypeErr> {
            TryInto::<Self>::try_into(<String as sea_orm_newtype::ValueType>::try_from(v)?)
                .map_err(|_| sea_orm_newtype::sea_query::ValueTypeErr)
        }
        fn type_name() -> String {
            // <String as sea_orm_newtype::ValueType>::type_name()
            stringify!(EmailAddress).to_owned()
        }
        fn array_type() -> sea_orm::sea_query::ArrayType {
            <String as sea_orm_newtype::ValueType>::array_type()
        }
        fn column_type() -> sea_orm_newtype::sea_query::ColumnType {
            <String as sea_orm_newtype::ValueType>::column_type()
        }
    }

    impl sea_orm_newtype::Nullable for EmailAddress {
        fn null() -> sea_orm_newtype::sea_query::Value {
            <String as sea_orm_newtype::Nullable>::null()
        }
    }
}

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "foo")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: uuid::Uuid,
    email_address: EmailAddress,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

fn main() {}
