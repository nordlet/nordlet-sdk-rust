pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtFr0564ComputeDeclarationsResponseCounts {
    #[serde(rename = "salesInvoices")]
    #[serde(default)]
    pub sales_invoices: i64,
}

impl LtFr0564ComputeDeclarationsResponseCounts {
    pub fn builder() -> LtFr0564ComputeDeclarationsResponseCountsBuilder {
        <LtFr0564ComputeDeclarationsResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtFr0564ComputeDeclarationsResponseCountsBuilder {
    sales_invoices: Option<i64>,
}

impl LtFr0564ComputeDeclarationsResponseCountsBuilder {
    pub fn sales_invoices(mut self, value: i64) -> Self {
        self.sales_invoices = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtFr0564ComputeDeclarationsResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sales_invoices`](LtFr0564ComputeDeclarationsResponseCountsBuilder::sales_invoices)
    pub fn build(self) -> Result<LtFr0564ComputeDeclarationsResponseCounts, BuildError> {
        Ok(LtFr0564ComputeDeclarationsResponseCounts {
            sales_invoices: self
                .sales_invoices
                .ok_or_else(|| BuildError::missing_field("sales_invoices"))?,
        })
    }
}
