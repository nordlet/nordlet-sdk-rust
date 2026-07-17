pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsVatSummaryResponseRowsItem {
    #[serde(rename = "vatRatePercent")]
    #[serde(default)]
    pub vat_rate_percent: String,
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub vat: String,
    #[serde(default)]
    pub gross: String,
    #[serde(default)]
    pub documents: i64,
}

impl PostV1ReportsVatSummaryResponseRowsItem {
    pub fn builder() -> PostV1ReportsVatSummaryResponseRowsItemBuilder {
        <PostV1ReportsVatSummaryResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsVatSummaryResponseRowsItemBuilder {
    vat_rate_percent: Option<String>,
    net: Option<String>,
    vat: Option<String>,
    gross: Option<String>,
    documents: Option<i64>,
}

impl PostV1ReportsVatSummaryResponseRowsItemBuilder {
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

    pub fn documents(mut self, value: i64) -> Self {
        self.documents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsVatSummaryResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`vat_rate_percent`](PostV1ReportsVatSummaryResponseRowsItemBuilder::vat_rate_percent)
    /// - [`net`](PostV1ReportsVatSummaryResponseRowsItemBuilder::net)
    /// - [`vat`](PostV1ReportsVatSummaryResponseRowsItemBuilder::vat)
    /// - [`gross`](PostV1ReportsVatSummaryResponseRowsItemBuilder::gross)
    /// - [`documents`](PostV1ReportsVatSummaryResponseRowsItemBuilder::documents)
    pub fn build(self) -> Result<PostV1ReportsVatSummaryResponseRowsItem, BuildError> {
        Ok(PostV1ReportsVatSummaryResponseRowsItem {
            vat_rate_percent: self
                .vat_rate_percent
                .ok_or_else(|| BuildError::missing_field("vat_rate_percent"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            vat: self.vat.ok_or_else(|| BuildError::missing_field("vat"))?,
            gross: self
                .gross
                .ok_or_else(|| BuildError::missing_field("gross"))?,
            documents: self
                .documents
                .ok_or_else(|| BuildError::missing_field("documents"))?,
        })
    }
}
