use proc_macro::TokenStream;
use syn::{Expr, parse_macro_input, ExprClosure, Attribute, spanned::Spanned};
use quote::quote;
use proc_macro_error::{
    abort,
    proc_macro_error,
};
use syn::export::Span;

struct StringColumn(Vec<String>);

impl core::fmt::Display for StringColumn {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for string in &self.0 {
            writeln!(f, "{}", string)?;
        }
        Ok(())
    }
}

struct Error {
    pub span: Span,
    pub message: String,
    pub help: StringColumn,
    pub note: StringColumn,
}

impl Error {
    pub fn new(span: Span, message: impl ToString) -> Self {
        Self {
            span,
            message: message.to_string(),
            help: StringColumn(vec![]),
            note: StringColumn(vec![]),
        }
    }

    #[allow(dead_code)]
    pub fn with_help(mut self, help: impl ToString) -> Self {
        self.help.0.push(help.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn with_note(mut self, note: impl ToString) -> Self {
        self.note.0.push(note.to_string());
        self
    }
}

fn span_to_attrs(attrs: &[Attribute]) -> Span {
    match attrs {
        [first] => first.span(),
        [first, .., last] => first.span().join(last.span()).unwrap(),
        _ => panic!("Set should not be empty"),
    }
}

fn transform_closure(closure: ExprClosure) -> Result<TokenStream, Error> {
    if !closure.attrs.is_empty() {
        return Err(Error::new(span_to_attrs(&closure.attrs), "closure attributes is not supported in expressions"));
    }
    Ok(quote!(#closure).into())
}

#[proc_macro_error]
#[proc_macro]
pub fn expr(input: TokenStream) -> TokenStream {
    let expr: Expr = parse_macro_input!(input as Expr);
    let closure = match expr {
        Expr::Closure(c) => c,
        x => abort!(x, "closure expression expected";
                    help = "Expressions needs to be inside a closure to be unbounded from local context";
                    note = "Example of usage: expr!(|a| a * 2)"
        ),
    };
    match transform_closure(closure) {
        Ok(x) => x,
        Err(e) => match (e.help.0.len(), e.note.0.len()) {
            (0, 0) => abort!(e.span, e.message),
            (0, _) => abort!(e.span, e.message; note = e.note),
            (_, 0) => abort!(e.span, e.message; help = e.help),
            (_, _) => abort!(e.span, e.message; help = e.help; note = e.note),
        },
    }
}
