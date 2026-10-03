pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlVatUeGenerateResponseRowsItem {
    pub section: PostV1DeclarationsPlVatUeGenerateResponseRowsItemSection,
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

impl PostV1DeclarationsPlVatUeGenerateResponseRowsItem {
    pub fn builder() -> PostV1DeclarationsPlVatUeGenerateResponseRowsItemBuilder {
        <PostV1DeclarationsPlVatUeGenerateResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlVatUeGenerateResponseRowsItemBuilder {
    section: Option<PostV1DeclarationsPlVatUeGenerateResponseRowsItemSection>,
    country_code: Option<String>,
    vat_number: Option<String>,
    partner_name: Option<String>,
    amount: Option<String>,
    documents: Option<Vec<String>>,
}

impl PostV1DeclarationsPlVatUeGenerateResponseRowsItemBuilder {
    pub fn section(
        mut self,
        value: PostV1DeclarationsPlVatUeGenerateResponseRowsItemSection,
    ) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlVatUeGenerateResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`section`](PostV1DeclarationsPlVatUeGenerateResponseRowsItemBuilder::section)
    /// - [`country_code`](PostV1DeclarationsPlVatUeGenerateResponseRowsItemBuilder::country_code)
    /// - [`vat_number`](PostV1DeclarationsPlVatUeGenerateResponseRowsItemBuilder::vat_number)
    /// - [`partner_name`](PostV1DeclarationsPlVatUeGenerateResponseRowsItemBuilder::partner_name)
    /// - [`amount`](PostV1DeclarationsPlVatUeGenerateResponseRowsItemBuilder::amount)
    /// - [`documents`](PostV1DeclarationsPlVatUeGenerateResponseRowsItemBuilder::documents)
    pub fn build(self) -> Result<PostV1DeclarationsPlVatUeGenerateResponseRowsItem, BuildError> {
        Ok(PostV1DeclarationsPlVatUeGenerateResponseRowsItem {
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
