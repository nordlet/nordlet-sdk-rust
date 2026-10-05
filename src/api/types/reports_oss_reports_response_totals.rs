pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OssReportsResponseTotals {
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub vat: String,
}

impl OssReportsResponseTotals {
    pub fn builder() -> OssReportsResponseTotalsBuilder {
        <OssReportsResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OssReportsResponseTotalsBuilder {
    net: Option<String>,
    vat: Option<String>,
}

impl OssReportsResponseTotalsBuilder {
    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    pub fn vat(mut self, value: impl Into<String>) -> Self {
        self.vat = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OssReportsResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`net`](OssReportsResponseTotalsBuilder::net)
    /// - [`vat`](OssReportsResponseTotalsBuilder::vat)
    pub fn build(self) -> Result<OssReportsResponseTotals, BuildError> {
        Ok(OssReportsResponseTotals {
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            vat: self.vat.ok_or_else(|| BuildError::missing_field("vat"))?,
        })
    }
}
