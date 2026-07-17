pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsOssResponseTotals {
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub vat: String,
}

impl PostV1ReportsOssResponseTotals {
    pub fn builder() -> PostV1ReportsOssResponseTotalsBuilder {
        <PostV1ReportsOssResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsOssResponseTotalsBuilder {
    net: Option<String>,
    vat: Option<String>,
}

impl PostV1ReportsOssResponseTotalsBuilder {
    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    pub fn vat(mut self, value: impl Into<String>) -> Self {
        self.vat = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsOssResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`net`](PostV1ReportsOssResponseTotalsBuilder::net)
    /// - [`vat`](PostV1ReportsOssResponseTotalsBuilder::vat)
    pub fn build(self) -> Result<PostV1ReportsOssResponseTotals, BuildError> {
        Ok(PostV1ReportsOssResponseTotals {
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            vat: self.vat.ok_or_else(|| BuildError::missing_field("vat"))?,
        })
    }
}
