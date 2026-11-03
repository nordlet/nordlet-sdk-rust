pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VatClassifiersListReferenceResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "nameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_en: Option<String>,
    #[serde(rename = "ratePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_percent: Option<String>,
    pub side: VatClassifiersListReferenceResponseRowsItemSide,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<VatClassifiersListReferenceResponseRowsItemScheme>,
    #[serde(rename = "rateFromLine")]
    #[serde(default)]
    pub rate_from_line: VatClassifiersListReferenceResponseRowsItemRateFromLine,
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
    name_en: Option<String>,
    rate_percent: Option<String>,
    side: Option<VatClassifiersListReferenceResponseRowsItemSide>,
    scheme: Option<VatClassifiersListReferenceResponseRowsItemScheme>,
    rate_from_line: Option<VatClassifiersListReferenceResponseRowsItemRateFromLine>,
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

    pub fn name_en(mut self, value: impl Into<String>) -> Self {
        self.name_en = Some(value.into());
        self
    }

    pub fn rate_percent(mut self, value: impl Into<String>) -> Self {
        self.rate_percent = Some(value.into());
        self
    }

    pub fn side(mut self, value: VatClassifiersListReferenceResponseRowsItemSide) -> Self {
        self.side = Some(value);
        self
    }

    pub fn scheme(mut self, value: VatClassifiersListReferenceResponseRowsItemScheme) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn rate_from_line(
        mut self,
        value: VatClassifiersListReferenceResponseRowsItemRateFromLine,
    ) -> Self {
        self.rate_from_line = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VatClassifiersListReferenceResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](VatClassifiersListReferenceResponseRowsItemBuilder::code)
    /// - [`country_code`](VatClassifiersListReferenceResponseRowsItemBuilder::country_code)
    /// - [`name`](VatClassifiersListReferenceResponseRowsItemBuilder::name)
    /// - [`side`](VatClassifiersListReferenceResponseRowsItemBuilder::side)
    /// - [`rate_from_line`](VatClassifiersListReferenceResponseRowsItemBuilder::rate_from_line)
    pub fn build(self) -> Result<VatClassifiersListReferenceResponseRowsItem, BuildError> {
        Ok(VatClassifiersListReferenceResponseRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            name_en: self.name_en,
            rate_percent: self.rate_percent,
            side: self.side.ok_or_else(|| BuildError::missing_field("side"))?,
            scheme: self.scheme,
            rate_from_line: self
                .rate_from_line
                .ok_or_else(|| BuildError::missing_field("rate_from_line"))?,
        })
    }
}
