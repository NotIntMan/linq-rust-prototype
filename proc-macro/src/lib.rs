use proc_macro::TokenStream;
use syn::{
    parse_macro_input,
    Expr,
};
use quote::quote;
use proc_macro_error::{proc_macro_error, abort};

#[proc_macro_error]
#[proc_macro]
pub fn expr(input: TokenStream) -> TokenStream {
    let expr: Expr = parse_macro_input!(input as Expr);
    let closure = match expr {
        Expr::Closure(c) => c,
        x => abort!(x, "closure expression expected"),
    };
    quote!(#closure).into()
}
