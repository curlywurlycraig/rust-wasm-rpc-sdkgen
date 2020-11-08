use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{Stmt, parse_quote, FnArg, ItemFn, Item};
use syn::punctuated::Punctuated;

use crate::utils::*;

pub fn impl_backend_remote(ast: ItemFn) -> TokenStream {
    let inner_fn = get_fn_with_prefixed_underscore(&ast);
    let inner_fn_stmt = Stmt::Item(Item::from(inner_fn.clone()));
    let inner_fn_ident = &inner_fn.sig.ident;

    let sig = &ast.sig;
    let vis = &ast.vis;

    // Determine what the args are as a tuple
    let inputs = sig.inputs.clone();
    let input_args_as_tuple = get_input_args_as_pat_tuple(&inputs);
    let input_args_as_type_tuple = get_input_args_as_type_tuple(&inputs);

    let inner_fn_args = get_pat_tuple_as_exprs(&input_args_as_tuple);

    let statements: Vec<Stmt> = parse_quote! {
        let #input_args_as_tuple: #input_args_as_type_tuple = rust_wasm_rpc_sdkgen::bincode::deserialize(bytes).unwrap();

        #inner_fn_stmt

        let call_output = #inner_fn_ident(#inner_fn_args);
        let encoded: Vec<u8> = rust_wasm_rpc_sdkgen::bincode::serialize(&call_output).unwrap();
        let result = String::from_utf8(encoded).unwrap();
        format!("{}", result)
    };

    let mut mutated_block = ast.block.clone();
    mutated_block.stmts = statements;

    let mut outer_sig = sig.clone();
    outer_sig.inputs = Punctuated::new();

    let input_fn_arg: FnArg = parse_quote! {
        bytes: &[u8]
    };
    outer_sig.inputs.push(input_fn_arg);
    outer_sig.output = parse_quote! { -> String };

    let gen = quote! {
        #vis #outer_sig #mutated_block
    };

    gen.into()
}