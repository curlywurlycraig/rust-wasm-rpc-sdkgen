use proc_macro::TokenStream;

mod backend;
mod frontend;
mod utils;

use backend::impl_backend_remote;
use frontend::impl_frontend_remote;

#[proc_macro_attribute]
pub fn remote(_: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse(item).unwrap();

    if cfg!(feature = "frontend") {
        impl_frontend_remote(ast)
    } else {
        impl_backend_remote(ast)
    }
}
