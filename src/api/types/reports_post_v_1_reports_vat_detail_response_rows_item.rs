pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsVatDetailResponseRowsItem {
    #[serde(rename = "documentId")]
    #[serde(default)]
    pub document_id: String,
    #[serde(rename = "documentNumber")]
    #[serde(default)]
    pub document_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "partnerName")]
    #[serde(default)]
    pub partner_name: String,
    #[serde(rename = "vatRatePercent")]
    #[serde(default)]
    pub vat_rate_percent: String,
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub vat: String,
    #[serde(default)]
    pub gross: String,
}

impl PostV1ReportsVatDetailResponseRowsItem {
    pub fn builder() -> PostV1ReportsVatDetailResponseRowsItemBuilder {
        <PostV1ReportsVatDetailResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsVatDetailResponseRowsItemBuilder {
    document_id: Option<String>,
    document_number: Option<String>,
    date: Option<String>,
    partner_name: Option<String>,
    vat_rate_percent: Option<String>,
    net: Option<String>,
    vat: Option<String>,
    gross: Option<String>,
}

impl PostV1ReportsVatDetailResponseRowsItemBuilder {
    pub fn document_id(mut self, value: impl Into<String>) -> Self {
        self.document_id = Some(value.into());
        self
    }

    pub fn document_number(mut self, value: impl Into<String>) -> Self {
        self.document_number = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn partner_name(mut self, value: impl Into<String>) -> Self {
        self.partner_name = Some(value.into());
        self
    }

    pub fn vat_rate_percent(mut self, value: impl Into<String>) -> Self {
        self.vat_rate_percent = Some(value.into());
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    pub fn vat(mut self, value: impl Into<String>) -> Self {
        self.vat = Some(value.into());
        self
    }

    pub fn gross(mut self, value: impl Into<String>) -> Self {
        self.gross = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsVatDetailResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_id`](PostV1ReportsVatDetailResponseRowsItemBuilder::document_id)
    /// - [`document_number`](PostV1ReportsVatDetailResponseRowsItemBuilder::document_number)
    /// - [`partner_name`](PostV1ReportsVatDetailResponseRowsItemBuilder::partner_name)
    /// - [`vat_rate_percent`](PostV1ReportsVatDetailResponseRowsItemBuilder::vat_rate_percent)
    /// - [`net`](PostV1ReportsVatDetailResponseRowsItemBuilder::net)
    /// - [`vat`](PostV1ReportsVatDetailResponseRowsItemBuilder::vat)
    /// - [`gross`](PostV1ReportsVatDetailResponseRowsItemBuilder::gross)
    pub fn build(self) -> Result<PostV1ReportsVatDetailResponseRowsItem, BuildError> {
        Ok(PostV1ReportsVatDetailResponseRowsItem {
            document_id: self
                .document_id
                .ok_or_else(|| BuildError::missing_field("document_id"))?,
            document_number: self
                .document_number
                .ok_or_else(|| BuildError::missing_field("document_number"))?,
            date: self.date,
            partner_name: self
                .partner_name
                .ok_or_else(|| BuildError::missing_field("partner_name"))?,
            vat_rate_percent: self
                .vat_rate_percent
                .ok_or_else(|| BuildError::missing_field("vat_rate_percent"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            vat: self.vat.ok_or_else(|| BuildError::missing_field("vat"))?,
            gross: self
                .gross
                .ok_or_else(|| BuildError::missing_field("gross"))?,
        })
    }
}
