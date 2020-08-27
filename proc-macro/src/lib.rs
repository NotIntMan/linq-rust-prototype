mod errors;

use proc_macro::TokenStream;
use syn::{Expr, parse, ExprClosure, Attribute, spanned::Spanned};
use quote::quote;
use proc_macro_error::proc_macro_error;
use syn::export::Span;
use crate::errors::Error;
use crate::errors::attributes::AttributesNotSupported;
use crate::errors::expressions::ClosureExpressionExpected;
use crate::errors::parse_error::ParseError;

fn span_to_attrs(attrs: &[Attribute]) -> Span {
    match attrs {
        [first] => first.span(),
        [first, .., last] => first.span().join(last.span()).unwrap(),
        _ => panic!("Set should not be empty"),
    }
}

pub fn transform_closure(closure: ExprClosure) -> Result<TokenStream, Error> {
    if !closure.attrs.is_empty() {
        return Err(Error::new(&span_to_attrs(&closure.attrs), AttributesNotSupported));
    }
    Ok(quote!(#closure).into())
}

pub fn transform_expr(input: TokenStream) -> Result<TokenStream, Error> {
    let expr = match parse::<Expr>(input) {
        Ok(x) => x,
        Err(e) => return Err(Error::new(&e.span(), ParseError(e))),
    };
    let closure = match expr {
        Expr::Closure(c) => c,
        x => return Err(Error::new(&x, ClosureExpressionExpected)),
    };
    let result = transform_closure(closure)?;
    Ok(result)
}

#[proc_macro_error]
#[proc_macro]
pub fn expr(input: TokenStream) -> TokenStream {
    match transform_expr(input) {
        Ok(x) => x,
        Err(e) => e.abort(),
    }
}
