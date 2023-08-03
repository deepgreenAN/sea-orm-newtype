//! # sea-orm-newtype
//! This crate provides helper derive macro to implement [new-type pattern](https://www.sea-ql.org/SeaORM/docs/generate-entity/newtype/) for sea-orm.
//!
//! ## Example
//! ```
//! use std::marker::PhantomData;
//! use uuid::Uuid;
//!
//! use sea_orm_newtype::DeriveNewType;
//!
//! /// New type for id that has specific type.
//! #[derive(Debug, Clone, PartialEq, DeriveNewType)]
//! #[sea_orm_newtype(from_into = "Uuid", primary_key)]
//! pub struct Id<T>(Uuid, PhantomData<T>);
//!
//! impl<T> From<Uuid> for Id<T> {
//!     fn from(id: Uuid) -> Id<T> {
//!         Id(id, PhantomData)
//!     }
//! }
//!
//! impl<T> From<Id<T>> for Uuid {
//!     fn from(value: Id<T>) -> Self {
//!         value.0
//!     }
//! }
//! ```

pub use sea_orm;
pub use sea_orm::sea_query;

pub use sea_orm::{
    sea_query::{value::Nullable, ValueType},
    TryFromU64, TryGetable, Value,
};

/// derive macro to implement new-type pattern for sea-orm.
pub use sea_orm_newtype_derive::DeriveNewType;
