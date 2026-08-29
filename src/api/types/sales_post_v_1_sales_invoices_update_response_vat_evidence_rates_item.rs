pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesUpdateResponseVatEvidenceRatesItem {
    #[serde(rename = "ratePercent")]
    #[serde(default)]
    pub rate_percent: String,
    #[serde(default)]
    pub country: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

impl PostV1SalesInvoicesUpdateResponseVatEvidenceRatesItem {
    pub fn builder() -> PostV1SalesInvoicesUpdateResponseVatEvidenceRatesItemBuilder {
        <PostV1SalesInvoicesUpdateResponseVatEvidenceRatesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesUpdateResponseVatEvidenceRatesItemBuilder {
    rate_percent: Option<String>,
    country: Option<String>,
    category: Option<String>,
}

impl PostV1SalesInvoicesUpdateResponseVatEvidenceRatesItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesUpdateResponseVatEvidenceRatesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rate_percent`](PostV1SalesInvoicesUpdateResponseVatEvidenceRatesItemBuilder::rate_percent)
    /// - [`country`](PostV1SalesInvoicesUpdateResponseVatEvidenceRatesItemBuilder::country)
    pub fn build(
        self,
    ) -> Result<PostV1SalesInvoicesUpdateResponseVatEvidenceRatesItem, BuildError> {
        Ok(PostV1SalesInvoicesUpdateResponseVatEvidenceRatesItem {
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
