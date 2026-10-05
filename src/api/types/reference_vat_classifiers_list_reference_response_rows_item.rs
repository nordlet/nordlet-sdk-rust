pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatClassifiersListReferenceResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ratePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_percent: Option<String>,
}

impl VatClassifiersListReferenceResponseRowsItem {
    pub fn builder() -> VatClassifiersListReferenceResponseRowsItemBuilder {
        <VatClassifiersListReferenceResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatClassifiersListReferenceResponseRowsItemBuilder {
    code: Option<String>,
    country_code: Option<String>,
    name: Option<String>,
    rate_percent: Option<String>,
}

impl VatClassifiersListReferenceResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`VatClassifiersListReferenceResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](VatClassifiersListReferenceResponseRowsItemBuilder::code)
    /// - [`country_code`](VatClassifiersListReferenceResponseRowsItemBuilder::country_code)
    /// - [`name`](VatClassifiersListReferenceResponseRowsItemBuilder::name)
    pub fn build(self) -> Result<VatClassifiersListReferenceResponseRowsItem, BuildError> {
        Ok(VatClassifiersListReferenceResponseRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            rate_percent: self.rate_percent,
        })
    }
}
