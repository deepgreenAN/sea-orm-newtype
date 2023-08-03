use darling::FromDeriveInput;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_quote, DeriveInput, Generics, Ident};

// -------------------------------------------------------------------------------------------------
// ConvertType

/// 変換方法を示す列挙体
pub enum ConvertType {
    /// from_into = "ident"
    FromInto(Ident),
    /// try_from_into = "ident"
    TryFromInto(Ident),
    /// transparent
    Transparent(Ident),
}

use ConvertType::*;

impl ConvertType {
    fn ident(&self) -> &Ident {
        match self {
            FromInto(ident) => ident,
            TryFromInto(ident) => ident,
            Transparent(ident) => ident,
        }
    }
}

// -------------------------------------------------------------------------------------------------
// InputReceiver

/// コンテナアトリビュート
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(sea_orm_newtype))]
struct InputReceiver {
    /// 読み取り時にFrom<A> for NewType，書き出し時にFrom<NewType> for Aで経由する
    from_into: Option<String>,
    /// 読み取り時にTryFrom<A> for NewType，書き出し時にFrom<NewType> for Aで経由する
    try_from_into: Option<String>,
    /// self.0の型を経由する
    #[darling(default)]
    transparent: bool,
    /// TryFromU64を実装する
    #[darling(default)]
    primary_key: bool,
    /// ValueType::type_nameを自身の名前に変更する
    #[darling(default)]
    type_name: bool,
}

// -------------------------------------------------------------------------------------------------
// derive_newtype_inner

/// derive_newtypeの内部関数
pub fn derive_newtype_inner(input: &DeriveInput) -> syn::Result<TokenStream> {
    let InputReceiver {
        from_into,
        try_from_into,
        transparent,
        primary_key,
        type_name,
    } = InputReceiver::from_derive_input(input)?;

    let new_type_name = &input.ident;

    let convert_type = match (from_into, try_from_into, transparent) {
        (Some(from_into), None, false) => FromInto(Ident::new(&from_into, Span::call_site())),
        (None, Some(try_from_into), false) => {
            TryFromInto(Ident::new(&try_from_into, Span::call_site()))
        }
        (None, None, true) | (None, None, false) => {
            Transparent(get_and_check_transparent_type(input)?)
        }
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                r#"sea_orm_newtype must be set from_into = "BaseType" or try_from_into = "BaseType" or transparent at most one."#,
            ))
        }
    };

    let generics = &input.generics;

    let mod_name = format_ident!("__sea_orm_newtype_{}", new_type_name);

    let impl_from_newtype_for_value =
        from_newtype_for_value(new_type_name, &convert_type, generics);
    let impl_try_getable_for_newtype =
        try_getable_for_newtype(new_type_name, &convert_type, generics);
    let impl_value_type_for_newtype =
        value_type_for_newtype(new_type_name, &convert_type, generics, type_name);

    let impl_nullable_for_newtype = nullable_for_newtype(new_type_name, &convert_type, generics);
    let impl_try_from_u64_for_newtype =
        primary_key.then(|| try_from_u64_for_newtype(new_type_name, &convert_type, generics));

    Ok(quote! {
        #[allow(non_snake_case)]
        mod #mod_name {
            use super::*;

            #impl_from_newtype_for_value

            #impl_try_getable_for_newtype

            #impl_value_type_for_newtype

            #impl_nullable_for_newtype

            #impl_try_from_u64_for_newtype
        }
    })
}

/// transparentの場合に型を取得．フィールドが無名かつ一つだけであるかどうかチェック
fn get_and_check_transparent_type(input: &DeriveInput) -> syn::Result<Ident> {
    if let syn::Data::Struct(data_struct) = &input.data {
        if let syn::Fields::Unnamed(unnamed_fields) = &data_struct.fields {
            if unnamed_fields.unnamed.len() == 1 {
                let base_type_ty = &(unnamed_fields.unnamed.first().unwrap().ty);

                return Ok(parse_quote!(#base_type_ty));
            }
        }
    }

    Err(syn::Error::new(
        Span::call_site(),
        r#"`sea_orm_newtype(transparent)` can only use for tuple struct thats have only one field."#,
    ))
}

/// impl From<NewType> for Value
fn from_newtype_for_value(
    new_type_name: &Ident,
    convert_type: &ConvertType,
    generics: &Generics,
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let from_block = match convert_type {
        FromInto(base_type_name) | TryFromInto(base_type_name) => {
            quote! { Into::<#base_type_name>::into(value).into() }
        }
        Transparent(_) => {
            quote! { value.0.into() }
        }
    };

    quote! {
        impl #impl_generics From<#new_type_name #ty_generics> for ::sea_orm_newtype::Value #where_clause {
            fn from(value: #new_type_name #ty_generics) -> Self {
                #from_block
            }
        }
    }
}

/// impl TryGetable for NewType
fn try_getable_for_newtype(
    new_type_name: &Ident,
    convert_type: &ConvertType,
    generics: &Generics,
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut assertion_error_bound = Option::<TokenStream>::None;

    let try_get_block = match convert_type {
        FromInto(base_type_name) => {
            quote! { Ok(Into::<Self>::into(res.try_get_by::<#base_type_name, I>(index)?)) }
        }
        TryFromInto(base_type_name) => {
            // アサーションを追加しておく(エラーはstd::error::Errorを実装する)
            assertion_error_bound = {
                let assertion_temp_type = format_ident!("__Assertion{}TryFrom", new_type_name);

                Some(quote! {
                    #[allow(non_camel_case_types)]
                    struct #assertion_temp_type where
                    <#new_type_name as TryFrom<#base_type_name>>::Error: ::std::error::Error;
                })
            };

            quote! {
                Ok(
                    TryInto::<Self>::try_into(res.try_get_by::<#base_type_name, I>(index)?)
                        .map_err(|e| ::sea_orm_newtype::sea_orm::DbErr::Custom(e.to_string()))?,
                )
            }
        }
        Transparent(base_type_name) => {
            quote! { Ok(#new_type_name (res.try_get_by::<#base_type_name, I>(index)?)) }
        }
    };

    quote! {
        // アサーション
        #assertion_error_bound

        impl #impl_generics ::sea_orm_newtype::TryGetable for #new_type_name #ty_generics #where_clause
        {
            fn try_get_by<I: ::sea_orm_newtype::sea_orm::ColIdx>(
                res: &::sea_orm_newtype::sea_orm::QueryResult,
                index: I,
            ) -> Result<Self, ::sea_orm_newtype::sea_orm::TryGetError> {
                #try_get_block
            }
        }
    }
}

/// impl ValueType for NewType
fn value_type_for_newtype(
    new_type_name: &Ident,
    convert_type: &ConvertType,
    generics: &Generics,
    use_type_name: bool,
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let try_from_block = match convert_type {
        FromInto(base_type_name) => {
            quote! {
                Ok(
                    Into::<Self>::into(<#base_type_name as ::sea_orm_newtype::ValueType>::try_from(v)?)
                )
            }
        }
        TryFromInto(base_type_name) => {
            quote! {
                TryInto::<Self>::try_into(<#base_type_name as ::sea_orm_newtype::ValueType>::try_from(v)?)
                .map_err(|_| ::sea_orm_newtype::sea_query::ValueTypeErr)
            }
        }
        Transparent(base_type_name) => {
            quote! {
                Ok(#new_type_name(<#base_type_name as ::sea_orm_newtype::ValueType>::try_from(v)?))
            }
        }
    };

    let type_name_block = if use_type_name {
        quote! {stringify!(#new_type_name #ty_generics).to_owned()}
    } else {
        let base_type_name = convert_type.ident();
        quote! {<#base_type_name as ::sea_orm_newtype::ValueType>::type_name()}
    };

    {
        let base_type_name = convert_type.ident();
        quote! {
            impl #impl_generics ::sea_orm_newtype::ValueType for #new_type_name #ty_generics #where_clause {
                fn try_from(
                    v: ::sea_orm_newtype::Value,
                ) -> Result<Self, ::sea_orm_newtype::sea_query::ValueTypeErr> {
                    #try_from_block
                }
                fn type_name() -> String {
                    #type_name_block
                }
                fn array_type() -> ::sea_orm_newtype::sea_query::ArrayType {
                    <#base_type_name as ::sea_orm_newtype::ValueType>::array_type()
                }
                fn column_type() -> ::sea_orm_newtype::sea_query::ColumnType {
                    <#base_type_name as ::sea_orm_newtype::ValueType>::column_type()
                }
            }
        }
    }
}

/// impl Nullable for NewType
fn nullable_for_newtype(
    new_type_name: &Ident,
    convert_type: &ConvertType,
    generics: &Generics,
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let base_type_name = convert_type.ident();

    quote! {
        impl #impl_generics ::sea_orm_newtype::Nullable for #new_type_name #ty_generics #where_clause
        {
            fn null() -> ::sea_orm_newtype::Value {
                <#base_type_name as ::sea_orm_newtype::Nullable>::null()
            }
        }
    }
}

/// impl TryFromU64 for NewType
fn try_from_u64_for_newtype(
    new_type_name: &Ident,
    convert_type: &ConvertType,
    generics: &Generics,
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let base_type_name = convert_type.ident();

    quote! {
        impl #impl_generics ::sea_orm_newtype::TryFromU64 for #new_type_name #ty_generics #where_clause
        {
            fn try_from_u64(n: u64) -> Result<Self, ::sea_orm_newtype::sea_orm::DbErr> {
                Ok(
                    Into::<Self>::into(<#base_type_name as ::sea_orm_newtype::TryFromU64>::try_from_u64(n)?)
                )
            }
        }
    }
}
