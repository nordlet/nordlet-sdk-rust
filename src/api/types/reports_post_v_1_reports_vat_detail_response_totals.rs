pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsVatDetailResponseTotals {
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub vat: String,
    #[serde(default)]
    pub gross: String,
}

impl PostV1ReportsVatDetailResponseTotals {
    pub fn builder() -> PostV1ReportsVatDetailResponseTotalsBuilder {
        <PostV1ReportsVatDetailResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsVatDetailResponseTotalsBuilder {
    net: Option<String>,
    vat: Option<String>,
    gross: Option<String>,
}

impl PostV1ReportsVatDetailResponseTotalsBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ReportsVatDetailResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`net`](PostV1ReportsVatDetailResponseTotalsBuilder::net)
    /// - [`vat`](PostV1ReportsVatDetailResponseTotalsBuilder::vat)
    /// - [`gross`](PostV1ReportsVatDetailResponseTotalsBuilder::gross)
    pub fn build(self) -> Result<PostV1ReportsVatDetailResponseTotals, BuildError> {
        Ok(PostV1ReportsVatDetailResponseTotals {
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            vat: self.vat.ok_or_else(|| BuildError::missing_field("vat"))?,
            gross: self
                .gross
                .ok_or_else(|| BuildError::missing_field("gross"))?,
        })
    }
}
