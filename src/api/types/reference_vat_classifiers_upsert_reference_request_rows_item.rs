pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatClassifiersUpsertReferenceRequestRowsItem {
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

impl VatClassifiersUpsertReferenceRequestRowsItem {
    pub fn builder() -> VatClassifiersUpsertReferenceRequestRowsItemBuilder {
        <VatClassifiersUpsertReferenceRequestRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatClassifiersUpsertReferenceRequestRowsItemBuilder {
    code: Option<String>,
    country_code: Option<String>,
    name: Option<String>,
    rate_percent: Option<String>,
}

impl VatClassifiersUpsertReferenceRequestRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`VatClassifiersUpsertReferenceRequestRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](VatClassifiersUpsertReferenceRequestRowsItemBuilder::code)
    /// - [`name`](VatClassifiersUpsertReferenceRequestRowsItemBuilder::name)
    pub fn build(self) -> Result<VatClassifiersUpsertReferenceRequestRowsItem, BuildError> {
        Ok(VatClassifiersUpsertReferenceRequestRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            country_code: self.country_code,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            rate_percent: self.rate_percent,
        })
    }
}
