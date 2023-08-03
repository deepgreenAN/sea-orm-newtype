mod inner;

use syn::{parse_macro_input, DeriveInput};

/// NewTypeはジェネリクスを持つことができるが，~~元の型はジェネリクなパラメーターを持つことはできない．~~
/// transparentを指定した場合NewTypeはジェネリックなパラメーターを持つことはできない．
#[proc_macro_derive(DeriveNewType, attributes(sea_orm_newtype))]
pub fn derive_new_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_ast = parse_macro_input!(input as DeriveInput);
    inner::derive_newtype_inner(&input_ast)
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}
