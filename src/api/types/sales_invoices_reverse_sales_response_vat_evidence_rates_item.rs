pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesReverseSalesResponseVatEvidenceRatesItem {
    #[serde(rename = "ratePercent")]
    #[serde(default)]
    pub rate_percent: String,
    #[serde(default)]
    pub country: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

impl InvoicesReverseSalesResponseVatEvidenceRatesItem {
    pub fn builder() -> InvoicesReverseSalesResponseVatEvidenceRatesItemBuilder {
        <InvoicesReverseSalesResponseVatEvidenceRatesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesReverseSalesResponseVatEvidenceRatesItemBuilder {
    rate_percent: Option<String>,
    country: Option<String>,
    category: Option<String>,
}

impl InvoicesReverseSalesResponseVatEvidenceRatesItemBuilder {
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

    /// Consumes the builder and constructs a [`InvoicesReverseSalesResponseVatEvidenceRatesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rate_percent`](InvoicesReverseSalesResponseVatEvidenceRatesItemBuilder::rate_percent)
    /// - [`country`](InvoicesReverseSalesResponseVatEvidenceRatesItemBuilder::country)
    pub fn build(self) -> Result<InvoicesReverseSalesResponseVatEvidenceRatesItem, BuildError> {
        Ok(InvoicesReverseSalesResponseVatEvidenceRatesItem {
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
