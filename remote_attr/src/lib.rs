use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{Stmt, parse_quote};


#[proc_macro_attribute]
pub fn remote(_: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse(item).unwrap();
    impl_remote(ast)
}

fn impl_remote(mut ast: syn::ItemFn) -> TokenStream {
    let new_ident = format!("_{}", &ast.sig.ident);
    ast.sig.ident = syn::Ident::new(&new_ident, quote::__private::Span::call_site());

    // Get short variables for quote
    let attrs = &ast.attrs;
    let vis = &ast.vis;
    let sig = &ast.sig;
    let statements = &mut ast.block.stmts;

    let decode_input_statements: Vec<Stmt> = parse_quote! {
        // Determine what the args are as a tuple

        // Update the signature to take Vec<u8> (if it has args)

        // Decode Vec<u8> into the tuple args

        // Spread out args using pattern, which should now be available to rest of statements
    };

    statements.splice(0..0, decode_input_statements);

    let block = &ast.block;

    let gen = quote! {
        #vis #sig #block
    };

    gen.into()
}