pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesIssueResponseVatEvidenceRatesItem {
    #[serde(rename = "ratePercent")]
    #[serde(default)]
    pub rate_percent: String,
    #[serde(default)]
    pub country: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

impl PostV1SalesInvoicesIssueResponseVatEvidenceRatesItem {
    pub fn builder() -> PostV1SalesInvoicesIssueResponseVatEvidenceRatesItemBuilder {
        <PostV1SalesInvoicesIssueResponseVatEvidenceRatesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesIssueResponseVatEvidenceRatesItemBuilder {
    rate_percent: Option<String>,
    country: Option<String>,
    category: Option<String>,
}

impl PostV1SalesInvoicesIssueResponseVatEvidenceRatesItemBuilder {
    pub fn rate_percent(mut self, value: impl Into<String>) -> Self {
        self.rate_percent = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesIssueResponseVatEvidenceRatesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rate_percent`](PostV1SalesInvoicesIssueResponseVatEvidenceRatesItemBuilder::rate_percent)
    /// - [`country`](PostV1SalesInvoicesIssueResponseVatEvidenceRatesItemBuilder::country)
    pub fn build(self) -> Result<PostV1SalesInvoicesIssueResponseVatEvidenceRatesItem, BuildError> {
        Ok(PostV1SalesInvoicesIssueResponseVatEvidenceRatesItem {
            rate_percent: self
                .rate_percent
                .ok_or_else(|| BuildError::missing_field("rate_percent"))?,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            category: self.category,
        })
    }
}
