pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PlVatUeGenerateDeclarationsResponseRowsItem {
    pub section: PlVatUeGenerateDeclarationsResponseRowsItemSection,
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "vatNumber")]
    #[serde(default)]
    pub vat_number: String,
    #[serde(rename = "partnerName")]
    #[serde(default)]
    pub partner_name: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub documents: Vec<String>,
}

impl PlVatUeGenerateDeclarationsResponseRowsItem {
    pub fn builder() -> PlVatUeGenerateDeclarationsResponseRowsItemBuilder {
        <PlVatUeGenerateDeclarationsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlVatUeGenerateDeclarationsResponseRowsItemBuilder {
    section: Option<PlVatUeGenerateDeclarationsResponseRowsItemSection>,
    country_code: Option<String>,
    vat_number: Option<String>,
    partner_name: Option<String>,
    amount: Option<String>,
    documents: Option<Vec<String>>,
}

impl PlVatUeGenerateDeclarationsResponseRowsItemBuilder {
    pub fn section(mut self, value: PlVatUeGenerateDeclarationsResponseRowsItemSection) -> Self {
        self.section = Some(value);
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn vat_number(mut self, value: impl Into<String>) -> Self {
        self.vat_number = Some(value.into());
        self
    }

    pub fn partner_name(mut self, value: impl Into<String>) -> Self {
        self.partner_name = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn documents(mut self, value: Vec<String>) -> Self {
        self.documents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlVatUeGenerateDeclarationsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`section`](PlVatUeGenerateDeclarationsResponseRowsItemBuilder::section)
    /// - [`country_code`](PlVatUeGenerateDeclarationsResponseRowsItemBuilder::country_code)
    /// - [`vat_number`](PlVatUeGenerateDeclarationsResponseRowsItemBuilder::vat_number)
    /// - [`partner_name`](PlVatUeGenerateDeclarationsResponseRowsItemBuilder::partner_name)
    /// - [`amount`](PlVatUeGenerateDeclarationsResponseRowsItemBuilder::amount)
    /// - [`documents`](PlVatUeGenerateDeclarationsResponseRowsItemBuilder::documents)
    pub fn build(self) -> Result<PlVatUeGenerateDeclarationsResponseRowsItem, BuildError> {
        Ok(PlVatUeGenerateDeclarationsResponseRowsItem {
            section: self
                .section
                .ok_or_else(|| BuildError::missing_field("section"))?,
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            vat_number: self
                .vat_number
                .ok_or_else(|| BuildError::missing_field("vat_number"))?,
            partner_name: self
                .partner_name
                .ok_or_else(|| BuildError::missing_field("partner_name"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            documents: self
                .documents
                .ok_or_else(|| BuildError::missing_field("documents"))?,
        })
    }
}
