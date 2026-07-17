pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsEuPurchasesResponseTotals {
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub vat: String,
}

impl PostV1ReportsEuPurchasesResponseTotals {
    pub fn builder() -> PostV1ReportsEuPurchasesResponseTotalsBuilder {
        <PostV1ReportsEuPurchasesResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsEuPurchasesResponseTotalsBuilder {
    net: Option<String>,
    vat: Option<String>,
}

impl PostV1ReportsEuPurchasesResponseTotalsBuilder {
    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    pub fn vat(mut self, value: impl Into<String>) -> Self {
        self.vat = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsEuPurchasesResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`net`](PostV1ReportsEuPurchasesResponseTotalsBuilder::net)
    /// - [`vat`](PostV1ReportsEuPurchasesResponseTotalsBuilder::vat)
    pub fn build(self) -> Result<PostV1ReportsEuPurchasesResponseTotals, BuildError> {
        Ok(PostV1ReportsEuPurchasesResponseTotals {
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            vat: self.vat.ok_or_else(|| BuildError::missing_field("vat"))?,
        })
    }
}
