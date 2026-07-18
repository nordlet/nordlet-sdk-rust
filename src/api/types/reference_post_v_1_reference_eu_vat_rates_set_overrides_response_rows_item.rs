pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceEuVatRatesSetOverridesResponseRowsItem {
    pub category: PostV1ReferenceEuVatRatesSetOverridesResponseRowsItemCategory,
    #[serde(rename = "ratePercent")]
    #[serde(default)]
    pub rate_percent: String,
}

impl PostV1ReferenceEuVatRatesSetOverridesResponseRowsItem {
    pub fn builder() -> PostV1ReferenceEuVatRatesSetOverridesResponseRowsItemBuilder {
        <PostV1ReferenceEuVatRatesSetOverridesResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceEuVatRatesSetOverridesResponseRowsItemBuilder {
    category: Option<PostV1ReferenceEuVatRatesSetOverridesResponseRowsItemCategory>,
    rate_percent: Option<String>,
}

impl PostV1ReferenceEuVatRatesSetOverridesResponseRowsItemBuilder {
    pub fn category(
        mut self,
        value: PostV1ReferenceEuVatRatesSetOverridesResponseRowsItemCategory,
    ) -> Self {
        self.category = Some(value);
        self
    }

    pub fn rate_percent(mut self, value: impl Into<String>) -> Self {
        self.rate_percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceEuVatRatesSetOverridesResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](PostV1ReferenceEuVatRatesSetOverridesResponseRowsItemBuilder::category)
    /// - [`rate_percent`](PostV1ReferenceEuVatRatesSetOverridesResponseRowsItemBuilder::rate_percent)
    pub fn build(
        self,
    ) -> Result<PostV1ReferenceEuVatRatesSetOverridesResponseRowsItem, BuildError> {
        Ok(PostV1ReferenceEuVatRatesSetOverridesResponseRowsItem {
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            rate_percent: self
                .rate_percent
                .ok_or_else(|| BuildError::missing_field("rate_percent"))?,
        })
    }
}
