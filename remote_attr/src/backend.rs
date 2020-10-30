use proc_macro::TokenStream;
use quote::quote;
use quote::__private::{Span};
use syn;
use syn::{Ident, Stmt, Pat, parse_quote, FnArg, PatTuple, TypeTuple, Type, ItemFn, Expr, Item};
use syn::punctuated::Punctuated;
use syn::token::{Paren, Comma};
use syn::FnArg::{Receiver, Typed};

fn get_input_args_as_pat_tuple(inputs: &Punctuated<FnArg, Comma>) -> PatTuple {
    let elems: Punctuated<Pat, Comma> = inputs.iter().map(|arg: &FnArg| {
        match arg {
            Receiver(_) => panic!("methods are not yet supported"),
            Typed(x) => *x.pat.clone()
        }
    }).collect();

    PatTuple {
        attrs: vec![],
        paren_token: Paren {
            span: quote::__private::Span::call_site(),
        },
        elems
    }
}

fn get_input_args_as_type_tuple(inputs: &Punctuated<FnArg, Comma>) -> TypeTuple {
    let elems: Punctuated<Type, Comma> = inputs.iter().map(|arg: &FnArg| {
        match arg {
            Receiver(_) => panic!("methods are not yet supported"),
            Typed(x) => *x.ty.clone()
        }
    }).collect();

    TypeTuple {
        paren_token: Paren {
            span: quote::__private::Span::call_site(),
        },
        elems
    }
}

fn get_pat_tuple_as_exprs(tuple: &PatTuple) -> Punctuated<Expr, Comma> {
    tuple.elems.iter().map(|pat: &Pat| {
        let ident: Ident = match pat {
            Pat::Ident(i) => i.ident.clone(),
            _ => panic!("Argument looks like something other than a simple identifier.")
        };

        let result: Expr = parse_quote! { #ident };
        result
    }).collect()
}

fn get_fn_with_prefixed_underscore(ast: &ItemFn) -> ItemFn {
    let mut inner_fn_definition = ast.clone();
    let sig = &inner_fn_definition.sig;
    let ident = &sig.ident;
    let prefixed_ident: String = format!("_{}", ident);
    inner_fn_definition.sig.ident = Ident::new(&prefixed_ident, Span::call_site());
    inner_fn_definition
}

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
        let #input_args_as_tuple: #input_args_as_type_tuple = bincode::deserialize(bytes).unwrap();

        #inner_fn_stmt

        let call_output = #inner_fn_ident(#inner_fn_args);
        let encoded: Vec<u8> = bincode::serialize(&call_output).unwrap();
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