pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VatResolveReferenceResponseRatesItem {
    pub category: VatResolveReferenceResponseRatesItemCategory,
    #[serde(rename = "ratePercent")]
    #[serde(default)]
    pub rate_percent: String,
}

impl VatResolveReferenceResponseRatesItem {
    pub fn builder() -> VatResolveReferenceResponseRatesItemBuilder {
        <VatResolveReferenceResponseRatesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatResolveReferenceResponseRatesItemBuilder {
    category: Option<VatResolveReferenceResponseRatesItemCategory>,
    rate_percent: Option<String>,
}

impl VatResolveReferenceResponseRatesItemBuilder {
    pub fn category(mut self, value: VatResolveReferenceResponseRatesItemCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn rate_percent(mut self, value: impl Into<String>) -> Self {
        self.rate_percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VatResolveReferenceResponseRatesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](VatResolveReferenceResponseRatesItemBuilder::category)
    /// - [`rate_percent`](VatResolveReferenceResponseRatesItemBuilder::rate_percent)
    pub fn build(self) -> Result<VatResolveReferenceResponseRatesItem, BuildError> {
        Ok(VatResolveReferenceResponseRatesItem {
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            rate_percent: self
                .rate_percent
                .ok_or_else(|| BuildError::missing_field("rate_percent"))?,
        })
    }
}
