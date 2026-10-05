pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtFr0600ComputeDeclarationsResponseCounts {
    #[serde(rename = "salesInvoices")]
    #[serde(default)]
    pub sales_invoices: i64,
    #[serde(rename = "purchaseInvoices")]
    #[serde(default)]
    pub purchase_invoices: i64,
}

impl LtFr0600ComputeDeclarationsResponseCounts {
    pub fn builder() -> LtFr0600ComputeDeclarationsResponseCountsBuilder {
        <LtFr0600ComputeDeclarationsResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtFr0600ComputeDeclarationsResponseCountsBuilder {
    sales_invoices: Option<i64>,
    purchase_invoices: Option<i64>,
}

impl LtFr0600ComputeDeclarationsResponseCountsBuilder {
    pub fn sales_invoices(mut self, value: i64) -> Self {
        self.sales_invoices = Some(value);
        self
    }

    pub fn purchase_invoices(mut self, value: i64) -> Self {
        self.purchase_invoices = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtFr0600ComputeDeclarationsResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sales_invoices`](LtFr0600ComputeDeclarationsResponseCountsBuilder::sales_invoices)
    /// - [`purchase_invoices`](LtFr0600ComputeDeclarationsResponseCountsBuilder::purchase_invoices)
    pub fn build(self) -> Result<LtFr0600ComputeDeclarationsResponseCounts, BuildError> {
        Ok(LtFr0600ComputeDeclarationsResponseCounts {
            sales_invoices: self
                .sales_invoices
                .ok_or_else(|| BuildError::missing_field("sales_invoices"))?,
            purchase_invoices: self
                .purchase_invoices
                .ok_or_else(|| BuildError::missing_field("purchase_invoices"))?,
        })
    }
}
