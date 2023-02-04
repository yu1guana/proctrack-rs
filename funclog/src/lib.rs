// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use proc_macro::TokenStream;
#[cfg(all(debug_assertions, not(feature = "disable")))]
use quote::ToTokens;
#[cfg(all(debug_assertions, not(feature = "disable")))]
use syn::parse::Parser;
#[cfg(all(debug_assertions, not(feature = "disable")))]
use syn::punctuated::Punctuated;
#[cfg(all(debug_assertions, not(feature = "disable")))]
use syn::{parse_macro_input, parse_quote, Block, ItemFn};
#[cfg(all(debug_assertions, not(feature = "disable")))]
use syn::{Expr, Token};

#[proc_macro_attribute]
pub fn funclog(args: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(all(debug_assertions, not(feature = "disable")))]
    {
        let mut ast = parse_macro_input!(item as ItemFn);
        let func_name = &ast.sig.ident;
        let func_body = ast.block.as_ref();
        let value_check_block = value_check_block(args);

        let block = parse_quote! {{
            eprintln!("[DEBUG:func_enter({}:{})] {}", file!(), line!(), stringify!(#func_name));
            #value_check_block;

            let func_body_closure = || #func_body;
            let ret = func_body_closure();

            #value_check_block;
            eprintln!("[DEBUG:func_exit({}:{})] {}", file!(), line!(), stringify!(#func_name));

            ret
        }};

        *ast.block = block;
        ast.into_token_stream().into()
    }
    #[cfg(not(all(debug_assertions, not(feature = "disable"))))]
    {
        let _ = args;
        item
    }
}

#[proc_macro_attribute]
pub fn methodlog(args: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(all(debug_assertions, not(feature = "disable")))]
    {
        let mut ast = parse_macro_input!(item as ItemFn);
        let func_name = &ast.sig.ident;
        let func_body = ast.block.as_ref();
        let value_check_block = value_check_block(args);

        let block;

        #[cfg(feature = "under_proctrack")]
        {
            block = parse_quote! {{
                use ::proctrack::typename::TypeName;

                eprintln!("[DEBUG:func_enter({}:{})] {}::{}", file!(), line!(), <Self as TypeName>::type_name(self), stringify!(#func_name));
                #value_check_block;

                let mut func_body_closure = || #func_body;
                let ret = func_body_closure();

                #value_check_block;
                eprintln!("[DEBUG:func_exit({}:{})] {}::{}", file!(), line!(), <Self as TypeName>::type_name(self), stringify!(#func_name));

                ret
            }};
        }
        #[cfg(not(feature = "under_proctrack"))]
        {
            block = parse_quote! {{
                use ::typename::TypeName;

                eprintln!("[DEBUG:func_enter({}:{})] {}::{}", file!(), line!(), <Self as TypeName>::type_name(self), stringify!(#func_name));
                #value_check_block;

                let mut func_body_closure = || #func_body;
                let ret = func_body_closure();

                #value_check_block;
                eprintln!("[DEBUG:func_exit({}:{})] {}::{}", file!(), line!(), <Self as TypeName>::type_name(self), stringify!(#func_name));

                ret
            }};
        }

        *ast.block = block;
        ast.into_token_stream().into()
    }
    #[cfg(not(all(debug_assertions, not(feature = "disable"))))]
    {
        let _ = args;
        item
    }
}

#[proc_macro_attribute]
pub fn methodlog_move(args: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(all(debug_assertions, not(feature = "disable")))]
    {
        let mut ast = parse_macro_input!(item as ItemFn);
        let func_name = &ast.sig.ident;
        let func_body = ast.block.as_ref();
        let value_check_block = value_check_block(args);

        let block;

        #[cfg(feature = "under_proctrack")]
        {
            block = parse_quote! {{
                use ::proctrack::typename::TypeName;

                let typename = <Self as TypeName>::type_name(&self).to_owned();

                eprintln!("[DEBUG:func_enter({}:{})] {}::{}", file!(), line!(), typename, stringify!(#func_name));
                #value_check_block;

                let func_body_closure = || #func_body;
                let ret = func_body_closure();

                #value_check_block;
                eprintln!("[DEBUG:func_exit({}:{})] {}::{}", file!(), line!(), typename, stringify!(#func_name));

                ret
            }};
        }
        #[cfg(not(feature = "under_proctrack"))]
        {
            block = parse_quote! {{
                use ::typename::TypeName;

                let typename = <Self as TypeName>::type_name(&self).to_owned();

                eprintln!("[DEBUG:func_enter({}:{})] {}::{}", file!(), line!(), typename, stringify!(#func_name));
                #value_check_block;

                let func_body_closure = || #func_body;
                let ret = func_body_closure();

                #value_check_block;
                eprintln!("[DEBUG:func_exit({}:{})] {}::{}", file!(), line!(), typename, stringify!(#func_name));

                ret
            }};
        }

        *ast.block = block;
        ast.into_token_stream().into()
    }
    #[cfg(not(all(debug_assertions, not(feature = "disable"))))]
    {
        let _ = args;
        item
    }
}

#[proc_macro_attribute]
pub fn methodlog_static(args: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(all(debug_assertions, not(feature = "disable")))]
    {
        let mut ast = parse_macro_input!(item as ItemFn);
        let func_name = &ast.sig.ident;
        let func_body = ast.block.as_ref();
        let value_check_block = value_check_block(args);

        let block;

        #[cfg(feature = "under_proctrack")]
        {
            block = parse_quote! {{
                use ::proctrack::typename::TypeName;

                eprintln!("[DEBUG:func_enter({}:{})] {}::{}", file!(), line!(), <Self as TypeName>::type_name_static(), stringify!(#func_name));
                #value_check_block;

                let func_body_closure = || #func_body;
                let ret = func_body_closure();

                #value_check_block;
                eprintln!("[DEBUG:func_exit({}:{})] {}::{}", file!(), line!(), <Self as TypeName>::type_name_static(), stringify!(#func_name));

                ret
            }};
        }
        #[cfg(not(feature = "under_proctrack"))]
        {
            block = parse_quote! {{
                use ::typename::TypeName;

                eprintln!("[DEBUG:func_enter({}:{})] {}::{}", file!(), line!(), <Self as TypeName>::type_name_static(), stringify!(#func_name));
                #value_check_block;

                let func_body_closure = || #func_body;
                let ret = func_body_closure();

                #value_check_block;
                eprintln!("[DEBUG:func_exit({}:{})] {}::{}", file!(), line!(), <Self as TypeName>::type_name_static(), stringify!(#func_name));

                ret
            }};
        }

        *ast.block = block;
        ast.into_token_stream().into()
    }
    #[cfg(not(all(debug_assertions, not(feature = "disable"))))]
    {
        let _ = args;
        item
    }
}

#[cfg(all(debug_assertions, not(feature = "disable")))]
fn value_check_block(args: TokenStream) -> Block {
    let args = Punctuated::<Expr, Token![,]>::parse_terminated
        .parse(args)
        .expect("Variables are expected");

    let mut value_check_block = parse_quote! {{}};

    for arg in args {
        value_check_block = parse_quote! {{
            #value_check_block
            eprintln!("[DEBUG:value({}:{})] {} = {:?}",file!(), line!(), stringify!(#arg), #arg);
        }};
    }

    value_check_block
}
