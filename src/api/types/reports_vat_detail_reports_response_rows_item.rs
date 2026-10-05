pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatDetailReportsResponseRowsItem {
    #[serde(rename = "documentId")]
    #[serde(default)]
    pub document_id: String,
    #[serde(rename = "documentNumber")]
    #[serde(default)]
    pub document_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
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

impl VatDetailReportsResponseRowsItem {
    pub fn builder() -> VatDetailReportsResponseRowsItemBuilder {
        <VatDetailReportsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatDetailReportsResponseRowsItemBuilder {
    document_id: Option<String>,
    document_number: Option<String>,
    date: Option<NaiveDate>,
    partner_name: Option<String>,
    vat_rate_percent: Option<String>,
    net: Option<String>,
    vat: Option<String>,
    gross: Option<String>,
}

impl VatDetailReportsResponseRowsItemBuilder {
    pub fn document_id(mut self, value: impl Into<String>) -> Self {
        self.document_id = Some(value.into());
        self
    }

    pub fn document_number(mut self, value: impl Into<String>) -> Self {
        self.document_number = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
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

    /// Consumes the builder and constructs a [`VatDetailReportsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_id`](VatDetailReportsResponseRowsItemBuilder::document_id)
    /// - [`document_number`](VatDetailReportsResponseRowsItemBuilder::document_number)
    /// - [`partner_name`](VatDetailReportsResponseRowsItemBuilder::partner_name)
    /// - [`vat_rate_percent`](VatDetailReportsResponseRowsItemBuilder::vat_rate_percent)
    /// - [`net`](VatDetailReportsResponseRowsItemBuilder::net)
    /// - [`vat`](VatDetailReportsResponseRowsItemBuilder::vat)
    /// - [`gross`](VatDetailReportsResponseRowsItemBuilder::gross)
    pub fn build(self) -> Result<VatDetailReportsResponseRowsItem, BuildError> {
        Ok(VatDetailReportsResponseRowsItem {
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
