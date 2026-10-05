pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuOssComputeDeclarationsResponseTotals {
    #[serde(rename = "taxableAmount")]
    #[serde(default)]
    pub taxable_amount: String,
    #[serde(rename = "vatAmount")]
    #[serde(default)]
    pub vat_amount: String,
}

impl EuOssComputeDeclarationsResponseTotals {
    pub fn builder() -> EuOssComputeDeclarationsResponseTotalsBuilder {
        <EuOssComputeDeclarationsResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuOssComputeDeclarationsResponseTotalsBuilder {
    taxable_amount: Option<String>,
    vat_amount: Option<String>,
}

impl EuOssComputeDeclarationsResponseTotalsBuilder {
    pub fn taxable_amount(mut self, value: impl Into<String>) -> Self {
        self.taxable_amount = Some(value.into());
        self
    }

    pub fn vat_amount(mut self, value: impl Into<String>) -> Self {
        self.vat_amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EuOssComputeDeclarationsResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`taxable_amount`](EuOssComputeDeclarationsResponseTotalsBuilder::taxable_amount)
    /// - [`vat_amount`](EuOssComputeDeclarationsResponseTotalsBuilder::vat_amount)
    pub fn build(self) -> Result<EuOssComputeDeclarationsResponseTotals, BuildError> {
        Ok(EuOssComputeDeclarationsResponseTotals {
            taxable_amount: self
                .taxable_amount
                .ok_or_else(|| BuildError::missing_field("taxable_amount"))?,
            vat_amount: self
                .vat_amount
                .ok_or_else(|| BuildError::missing_field("vat_amount"))?,
        })
    }
}
