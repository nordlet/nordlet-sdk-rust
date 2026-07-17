pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceVatResolveResponseRatesItem {
    pub category: PostV1ReferenceVatResolveResponseRatesItemCategory,
    #[serde(rename = "ratePercent")]
    #[serde(default)]
    pub rate_percent: String,
}

impl PostV1ReferenceVatResolveResponseRatesItem {
    pub fn builder() -> PostV1ReferenceVatResolveResponseRatesItemBuilder {
        <PostV1ReferenceVatResolveResponseRatesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceVatResolveResponseRatesItemBuilder {
    category: Option<PostV1ReferenceVatResolveResponseRatesItemCategory>,
    rate_percent: Option<String>,
}

impl PostV1ReferenceVatResolveResponseRatesItemBuilder {
    pub fn category(mut self, value: PostV1ReferenceVatResolveResponseRatesItemCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn rate_percent(mut self, value: impl Into<String>) -> Self {
        self.rate_percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceVatResolveResponseRatesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](PostV1ReferenceVatResolveResponseRatesItemBuilder::category)
    /// - [`rate_percent`](PostV1ReferenceVatResolveResponseRatesItemBuilder::rate_percent)
    pub fn build(self) -> Result<PostV1ReferenceVatResolveResponseRatesItem, BuildError> {
        Ok(PostV1ReferenceVatResolveResponseRatesItem {
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            rate_percent: self
                .rate_percent
                .ok_or_else(|| BuildError::missing_field("rate_percent"))?,
        })
    }
}
