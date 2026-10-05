pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuIossComputeDeclarationsResponseTotals {
    #[serde(rename = "taxableAmount")]
    #[serde(default)]
    pub taxable_amount: String,
    #[serde(rename = "vatAmount")]
    #[serde(default)]
    pub vat_amount: String,
}

impl EuIossComputeDeclarationsResponseTotals {
    pub fn builder() -> EuIossComputeDeclarationsResponseTotalsBuilder {
        <EuIossComputeDeclarationsResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuIossComputeDeclarationsResponseTotalsBuilder {
    taxable_amount: Option<String>,
    vat_amount: Option<String>,
}

impl EuIossComputeDeclarationsResponseTotalsBuilder {
    pub fn taxable_amount(mut self, value: impl Into<String>) -> Self {
        self.taxable_amount = Some(value.into());
        self
    }

    pub fn vat_amount(mut self, value: impl Into<String>) -> Self {
        self.vat_amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EuIossComputeDeclarationsResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`taxable_amount`](EuIossComputeDeclarationsResponseTotalsBuilder::taxable_amount)
    /// - [`vat_amount`](EuIossComputeDeclarationsResponseTotalsBuilder::vat_amount)
    pub fn build(self) -> Result<EuIossComputeDeclarationsResponseTotals, BuildError> {
        Ok(EuIossComputeDeclarationsResponseTotals {
            taxable_amount: self
                .taxable_amount
                .ok_or_else(|| BuildError::missing_field("taxable_amount"))?,
            vat_amount: self
                .vat_amount
                .ok_or_else(|| BuildError::missing_field("vat_amount"))?,
        })
    }
}
