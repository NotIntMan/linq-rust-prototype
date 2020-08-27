pub mod attributes;
pub mod expressions;
pub mod parse_error;

use syn::export::Span;
use proc_macro_error::abort;
use crate::errors::attributes::AttributesNotSupported;
use syn::spanned::Spanned;
use crate::errors::expressions::ClosureExpressionExpected;
use crate::errors::parse_error::ParseError;

pub struct Error {
    span: Span,
    kind: ErrorKind,
}

impl Error {
    pub fn new(span: &impl Spanned, kind: impl Into<ErrorKind>) -> Self {
        Self {
            span: span.span(),
            kind: kind.into(),
        }
    }

    pub fn abort(&self) -> ! {
        pub struct StringColumn<'a>(&'a [String]);

        impl<'a> core::fmt::Display for StringColumn<'a> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                for string in self.0 {
                    writeln!(f, "{}", string)?;
                }
                Ok(())
            }
        }

        let Error { span, kind } = self;
        let ErrorDescription { message, help, note } = kind.description();

        match (help.len(), note.len()) {
            (0, 0) => abort!(span, message),
            (0, _) => abort!(span, message; note = StringColumn(&note)),
            (_, 0) => abort!(span, message; help = StringColumn(&help)),
            (_, _) => abort!(span, message; help = StringColumn(&help); note = StringColumn(&note)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    AttributesNotSupported(AttributesNotSupported),
    ClosureExpressionExpected(ClosureExpressionExpected),
    ParseError(ParseError),
}

impl From<AttributesNotSupported> for ErrorKind {
    fn from(x: AttributesNotSupported) -> Self {
        ErrorKind::AttributesNotSupported(x)
    }
}

impl From<ClosureExpressionExpected> for ErrorKind {
    fn from(x: ClosureExpressionExpected) -> Self {
        ErrorKind::ClosureExpressionExpected(x)
    }
}

impl From<ParseError> for ErrorKind {
    fn from(x: ParseError) -> Self {
        ErrorKind::ParseError(x)
    }
}

impl ErrorKind {
    fn description(&self) -> ErrorDescription {
        match self {
            ErrorKind::AttributesNotSupported(x) => x.description(),
            ErrorKind::ClosureExpressionExpected(x) => x.description(),
            ErrorKind::ParseError(x) => x.description(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ErrorDescription {
    pub message: String,
    pub help: Vec<String>,
    pub note: Vec<String>,
}

impl ErrorDescription {
    pub fn new(message: impl ToString) -> Self {
        Self {
            message: message.to_string(),
            help: vec![],
            note: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn with_help(mut self, help: impl ToString) -> Self {
        self.help.push(help.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn with_note(mut self, note: impl ToString) -> Self {
        self.note.push(note.to_string());
        self
    }
}
