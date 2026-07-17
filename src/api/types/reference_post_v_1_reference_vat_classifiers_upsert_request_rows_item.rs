pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceVatClassifiersUpsertRequestRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ratePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_percent: Option<String>,
}

impl PostV1ReferenceVatClassifiersUpsertRequestRowsItem {
    pub fn builder() -> PostV1ReferenceVatClassifiersUpsertRequestRowsItemBuilder {
        <PostV1ReferenceVatClassifiersUpsertRequestRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceVatClassifiersUpsertRequestRowsItemBuilder {
    code: Option<String>,
    country_code: Option<String>,
    name: Option<String>,
    rate_percent: Option<String>,
}

impl PostV1ReferenceVatClassifiersUpsertRequestRowsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn rate_percent(mut self, value: impl Into<String>) -> Self {
        self.rate_percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceVatClassifiersUpsertRequestRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1ReferenceVatClassifiersUpsertRequestRowsItemBuilder::code)
    /// - [`name`](PostV1ReferenceVatClassifiersUpsertRequestRowsItemBuilder::name)
    pub fn build(self) -> Result<PostV1ReferenceVatClassifiersUpsertRequestRowsItem, BuildError> {
        Ok(PostV1ReferenceVatClassifiersUpsertRequestRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            country_code: self.country_code,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            rate_percent: self.rate_percent,
        })
    }
}
