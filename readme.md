# sea-orm-newtype

This crate provides helper derive macro to implement [new-type pattern](https://www.sea-ql.org/SeaORM/docs/generate-entity/newtype/) for sea-orm. From sea-orm @ 0.12.x, you can use [DeriveValueType](https://docs.rs/sea-orm/0.12.1/sea_orm/derive.DeriveValueType.html) macro. Please check it too.

## Example

### convert by From and Into trait

```rust
use std::marker::PhantomData;
use uuid::Uuid;

use sea_orm_newtype::DeriveNewType;

/// New type for id that has specific type.
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
```

### convert by TryFrom and Into trait

```rust
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
```

### handle it as self.0 type

```rust
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
```
