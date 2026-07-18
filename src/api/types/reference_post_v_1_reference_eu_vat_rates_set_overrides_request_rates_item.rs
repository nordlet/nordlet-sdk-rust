pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceEuVatRatesSetOverridesRequestRatesItem {
    pub category: PostV1ReferenceEuVatRatesSetOverridesRequestRatesItemCategory,
    #[serde(rename = "ratePercent")]
    #[serde(default)]
    pub rate_percent: String,
}

impl PostV1ReferenceEuVatRatesSetOverridesRequestRatesItem {
    pub fn builder() -> PostV1ReferenceEuVatRatesSetOverridesRequestRatesItemBuilder {
        <PostV1ReferenceEuVatRatesSetOverridesRequestRatesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceEuVatRatesSetOverridesRequestRatesItemBuilder {
    category: Option<PostV1ReferenceEuVatRatesSetOverridesRequestRatesItemCategory>,
    rate_percent: Option<String>,
}

impl PostV1ReferenceEuVatRatesSetOverridesRequestRatesItemBuilder {
    pub fn category(
        mut self,
        value: PostV1ReferenceEuVatRatesSetOverridesRequestRatesItemCategory,
    ) -> Self {
        self.category = Some(value);
        self
    }

    pub fn rate_percent(mut self, value: impl Into<String>) -> Self {
        self.rate_percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceEuVatRatesSetOverridesRequestRatesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](PostV1ReferenceEuVatRatesSetOverridesRequestRatesItemBuilder::category)
    /// - [`rate_percent`](PostV1ReferenceEuVatRatesSetOverridesRequestRatesItemBuilder::rate_percent)
    pub fn build(
        self,
    ) -> Result<PostV1ReferenceEuVatRatesSetOverridesRequestRatesItem, BuildError> {
        Ok(PostV1ReferenceEuVatRatesSetOverridesRequestRatesItem {
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            rate_percent: self
                .rate_percent
                .ok_or_else(|| BuildError::missing_field("rate_percent"))?,
        })
    }
}
