pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesIssueResponseVatEvidenceLocation {
    #[serde(rename = "billingCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl PostV1SalesInvoicesIssueResponseVatEvidenceLocation {
    pub fn builder() -> PostV1SalesInvoicesIssueResponseVatEvidenceLocationBuilder {
        <PostV1SalesInvoicesIssueResponseVatEvidenceLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesIssueResponseVatEvidenceLocationBuilder {
    billing_country_code: Option<String>,
    source: Option<String>,
}

impl PostV1SalesInvoicesIssueResponseVatEvidenceLocationBuilder {
    pub fn billing_country_code(mut self, value: impl Into<String>) -> Self {
        self.billing_country_code = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesIssueResponseVatEvidenceLocation`].
    pub fn build(self) -> Result<PostV1SalesInvoicesIssueResponseVatEvidenceLocation, BuildError> {
        Ok(PostV1SalesInvoicesIssueResponseVatEvidenceLocation {
            billing_country_code: self.billing_country_code,
            source: self.source,
        })
    }
}
