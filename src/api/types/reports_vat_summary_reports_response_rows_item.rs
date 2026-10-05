pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatSummaryReportsResponseRowsItem {
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

impl VatSummaryReportsResponseRowsItem {
    pub fn builder() -> VatSummaryReportsResponseRowsItemBuilder {
        <VatSummaryReportsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatSummaryReportsResponseRowsItemBuilder {
    vat_rate_percent: Option<String>,
    net: Option<String>,
    vat: Option<String>,
    gross: Option<String>,
    documents: Option<i64>,
}

impl VatSummaryReportsResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`VatSummaryReportsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`vat_rate_percent`](VatSummaryReportsResponseRowsItemBuilder::vat_rate_percent)
    /// - [`net`](VatSummaryReportsResponseRowsItemBuilder::net)
    /// - [`vat`](VatSummaryReportsResponseRowsItemBuilder::vat)
    /// - [`gross`](VatSummaryReportsResponseRowsItemBuilder::gross)
    /// - [`documents`](VatSummaryReportsResponseRowsItemBuilder::documents)
    pub fn build(self) -> Result<VatSummaryReportsResponseRowsItem, BuildError> {
        Ok(VatSummaryReportsResponseRowsItem {
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
