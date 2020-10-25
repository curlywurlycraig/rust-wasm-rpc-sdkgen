use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{Stmt, Pat, parse_quote, FnArg, PatTuple, TypeTuple, PatType, Type};
use syn::punctuated::Punctuated;
use syn::token::{Paren, Comma};
use syn::FnArg::{Receiver, Typed};

#[proc_macro_attribute]
pub fn remote(_: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse(item).unwrap();
    impl_remote(ast)
}

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


fn impl_remote(mut ast: syn::ItemFn) -> TokenStream {
    // Get short variables for quote
    let attrs = &ast.attrs;
    let vis = &ast.vis;
    let sig = &mut ast.sig;
    let statements = &mut ast.block.stmts;

    // Determine what the args are as a tuple
    let inputs = &sig.inputs;
    let input_args_as_tuple = get_input_args_as_pat_tuple(&inputs);
    let input_args_as_type_tuple = get_input_args_as_type_tuple(&inputs);

    // Decode Vec<u8> into the tuple args
    let decode_input_statements: Vec<Stmt> = parse_quote! {
        let #input_args_as_tuple: #input_args_as_type_tuple = bincode::deserialize(bytes).unwrap();
    };

    statements.splice(0..0, decode_input_statements);

    let block = &ast.block;

    sig.inputs = Punctuated::new();

    let input_fn_arg: FnArg = parse_quote! {
        bytes: Vec<u8>
    };
    sig.inputs.push(input_fn_arg);

    let gen = quote! {
        #vis #sig #block
    };

    gen.into()
}