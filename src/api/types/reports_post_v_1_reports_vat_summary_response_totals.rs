pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsVatSummaryResponseTotals {
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub vat: String,
    #[serde(default)]
    pub gross: String,
}

impl PostV1ReportsVatSummaryResponseTotals {
    pub fn builder() -> PostV1ReportsVatSummaryResponseTotalsBuilder {
        <PostV1ReportsVatSummaryResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsVatSummaryResponseTotalsBuilder {
    net: Option<String>,
    vat: Option<String>,
    gross: Option<String>,
}

impl PostV1ReportsVatSummaryResponseTotalsBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ReportsVatSummaryResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`net`](PostV1ReportsVatSummaryResponseTotalsBuilder::net)
    /// - [`vat`](PostV1ReportsVatSummaryResponseTotalsBuilder::vat)
    /// - [`gross`](PostV1ReportsVatSummaryResponseTotalsBuilder::gross)
    pub fn build(self) -> Result<PostV1ReportsVatSummaryResponseTotals, BuildError> {
        Ok(PostV1ReportsVatSummaryResponseTotals {
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            vat: self.vat.ok_or_else(|| BuildError::missing_field("vat"))?,
            gross: self
                .gross
                .ok_or_else(|| BuildError::missing_field("gross"))?,
        })
    }
}
