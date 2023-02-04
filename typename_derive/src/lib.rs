// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(TypeName)]
pub fn derive_typename(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    #[cfg(feature = "under_proctrack")]
    let gen = quote! {
        impl #impl_generics ::proctrack::typename::TypeName for #name #ty_generics #where_clause {
            fn type_name(&self) -> &str{
                stringify!(#name)
            }
            fn type_name_static() -> &'static str{
                stringify!(#name)
            }
        }
    };

    #[cfg(not(feature = "under_proctrack"))]
    let gen = quote! {
        impl #impl_generics ::typename::TypeName for #name #ty_generics #where_clause {
            fn type_name(&self) -> &str{
                stringify!(#name)
            }
            fn type_name_static() -> &'static str{
                stringify!(#name)
            }
        }
    };

    gen.into()
}
