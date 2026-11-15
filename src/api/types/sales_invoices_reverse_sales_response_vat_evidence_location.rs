pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesReverseSalesResponseVatEvidenceLocation {
    #[serde(rename = "billingCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl InvoicesReverseSalesResponseVatEvidenceLocation {
    pub fn builder() -> InvoicesReverseSalesResponseVatEvidenceLocationBuilder {
        <InvoicesReverseSalesResponseVatEvidenceLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesReverseSalesResponseVatEvidenceLocationBuilder {
    billing_country_code: Option<String>,
    source: Option<String>,
}

impl InvoicesReverseSalesResponseVatEvidenceLocationBuilder {
    pub fn billing_country_code(mut self, value: impl Into<String>) -> Self {
        self.billing_country_code = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesReverseSalesResponseVatEvidenceLocation`].
    pub fn build(self) -> Result<InvoicesReverseSalesResponseVatEvidenceLocation, BuildError> {
        Ok(InvoicesReverseSalesResponseVatEvidenceLocation {
            billing_country_code: self.billing_country_code,
            source: self.source,
        })
    }
}
