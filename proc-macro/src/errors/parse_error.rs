use crate::errors::ErrorDescription;

#[derive(Debug, Clone)]
pub struct ParseError(pub syn::Error);

impl ParseError {
    pub fn description(&self) -> ErrorDescription {
        ErrorDescription::new(&self.0)
    }
}

impl PartialEq for ParseError {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_string() == other.0.to_string()
    }
}

impl Eq for ParseError {}
