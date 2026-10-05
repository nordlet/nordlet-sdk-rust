pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtIsafGenerateDeclarationsResponseCounts {
    #[serde(rename = "salesInvoices")]
    #[serde(default)]
    pub sales_invoices: i64,
    #[serde(rename = "purchaseInvoices")]
    #[serde(default)]
    pub purchase_invoices: i64,
    #[serde(default)]
    pub customers: i64,
    #[serde(default)]
    pub suppliers: i64,
}

impl LtIsafGenerateDeclarationsResponseCounts {
    pub fn builder() -> LtIsafGenerateDeclarationsResponseCountsBuilder {
        <LtIsafGenerateDeclarationsResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtIsafGenerateDeclarationsResponseCountsBuilder {
    sales_invoices: Option<i64>,
    purchase_invoices: Option<i64>,
    customers: Option<i64>,
    suppliers: Option<i64>,
}

impl LtIsafGenerateDeclarationsResponseCountsBuilder {
    pub fn sales_invoices(mut self, value: i64) -> Self {
        self.sales_invoices = Some(value);
        self
    }

    pub fn purchase_invoices(mut self, value: i64) -> Self {
        self.purchase_invoices = Some(value);
        self
    }

    pub fn customers(mut self, value: i64) -> Self {
        self.customers = Some(value);
        self
    }

    pub fn suppliers(mut self, value: i64) -> Self {
        self.suppliers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtIsafGenerateDeclarationsResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sales_invoices`](LtIsafGenerateDeclarationsResponseCountsBuilder::sales_invoices)
    /// - [`purchase_invoices`](LtIsafGenerateDeclarationsResponseCountsBuilder::purchase_invoices)
    /// - [`customers`](LtIsafGenerateDeclarationsResponseCountsBuilder::customers)
    /// - [`suppliers`](LtIsafGenerateDeclarationsResponseCountsBuilder::suppliers)
    pub fn build(self) -> Result<LtIsafGenerateDeclarationsResponseCounts, BuildError> {
        Ok(LtIsafGenerateDeclarationsResponseCounts {
            sales_invoices: self
                .sales_invoices
                .ok_or_else(|| BuildError::missing_field("sales_invoices"))?,
            purchase_invoices: self
                .purchase_invoices
                .ok_or_else(|| BuildError::missing_field("purchase_invoices"))?,
            customers: self
                .customers
                .ok_or_else(|| BuildError::missing_field("customers"))?,
            suppliers: self
                .suppliers
                .ok_or_else(|| BuildError::missing_field("suppliers"))?,
        })
    }
}
