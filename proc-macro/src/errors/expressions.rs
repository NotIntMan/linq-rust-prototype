use crate::errors::ErrorDescription;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClosureExpressionExpected;

impl ClosureExpressionExpected {
    pub fn description(&self) -> ErrorDescription {
        ErrorDescription::new("closure expression expected")
            .with_help("Expressions needs to be inside a closure to be unbounded from local context")
            .with_note("Example of usage: expr!(|a| a * 2)")
    }
}
