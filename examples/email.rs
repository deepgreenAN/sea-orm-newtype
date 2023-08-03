use std::fmt::Debug;
use std::str::FromStr;

use sea_orm_newtype::DeriveNewType;

#[derive(Clone, Debug, PartialEq, DeriveNewType)]
#[sea_orm_newtype(try_from_into = "String")]
pub struct EmailAddress(email_address::EmailAddress);

#[derive(Debug, thiserror::Error)]
#[error("ParseError")]
pub struct ParseError;

impl TryFrom<String> for EmailAddress {
    type Error = ParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(EmailAddress(
            email_address::EmailAddress::from_str(&value).map_err(|_| ParseError)?,
        ))
    }
}

impl From<EmailAddress> for String {
    fn from(value: EmailAddress) -> Self {
        value.0.to_string()
    }
}

// -------------------------------------------------------------------------------------------------

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
