mod inner;

use syn::{parse_macro_input, DeriveInput};

/// Derive some traits to use in sea-orm. By default, the following traits are implemented.
/// - From<T> for sea_query::Value
/// - sea_orm::TryGetable for T
/// - sea_query::ValueType for T
/// - sea_query::Nullable for T
///
/// # Attributes
/// - `from_into = "OrmType"`: NewType is converted into a type that can be used in sea-orm by From<OrmType> and Into<OrmType> trait.
/// - `try_from_into = "OrmType"`: NewType is converted into a type that can be used in sea-orm by TryFrom<OrmType> and Into<OrmType> trait.
/// - `transparent`: NewType is interpreted as a type of self.0
/// - `primary_key`: In addition to the defaults, sea_orm::TryFromU64 is implemented.
/// - `type_name`: Change the ValueType::type_name implementation for using its own name.  
#[proc_macro_derive(DeriveNewType, attributes(sea_orm_newtype))]
pub fn derive_new_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_ast = parse_macro_input!(input as DeriveInput);
    inner::derive_newtype_inner(&input_ast)
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}
