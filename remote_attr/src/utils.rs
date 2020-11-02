use quote::__private::{Span};
use syn;
use syn::{Ident, Pat, parse_quote, FnArg, PatTuple, TypeTuple, Type, ItemFn, Expr};
use syn::punctuated::Punctuated;
use syn::token::{Paren, Comma};
use syn::FnArg::{Receiver, Typed};

pub fn get_input_args_as_pat_tuple(inputs: &Punctuated<FnArg, Comma>) -> PatTuple {
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

pub fn get_input_args_as_type_tuple(inputs: &Punctuated<FnArg, Comma>) -> TypeTuple {
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

pub fn get_pat_tuple_as_exprs(tuple: &PatTuple) -> Punctuated<Expr, Comma> {
    tuple.elems.iter().map(|pat: &Pat| {
        let ident: Ident = match pat {
            Pat::Ident(i) => i.ident.clone(),
            _ => panic!("Argument looks like something other than a simple identifier.")
        };

        let result: Expr = parse_quote! { #ident };
        result
    }).collect()
}

pub fn get_fn_with_prefixed_underscore(ast: &ItemFn) -> ItemFn {
    let mut inner_fn_definition = ast.clone();
    let sig = &inner_fn_definition.sig;
    let ident = &sig.ident;
    inner_fn_definition.sig.ident = make_ident_from_str(&format!("_{}", ident));
    inner_fn_definition
}

pub fn make_ident_from_str(ident_str: &str) -> Ident {
    Ident::new(&ident_str, Span::call_site())
}