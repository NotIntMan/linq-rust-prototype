use crate::errors::ErrorDescription;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttributesNotSupported;

impl AttributesNotSupported {
    pub fn description(&self) -> ErrorDescription {
        ErrorDescription::new("closure attributes is not supported in expressions")
    }
}

#[cfg(test)]
mod tests {
    use crate::transform_expr;
    use crate::errors::attributes::AttributesNotSupported;
    use quote::quote;

    #[test]
    fn attributes_not_supported_test() {
        let result = transform_expr(quote!(#[azaz] |x| x * 2).into());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, AttributesNotSupported.into());
    }
}