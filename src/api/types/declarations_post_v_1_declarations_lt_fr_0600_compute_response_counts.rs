pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtFr0600ComputeResponseCounts {
    #[serde(rename = "salesInvoices")]
    #[serde(default)]
    pub sales_invoices: i64,
    #[serde(rename = "purchaseInvoices")]
    #[serde(default)]
    pub purchase_invoices: i64,
}

impl PostV1DeclarationsLtFr0600ComputeResponseCounts {
    pub fn builder() -> PostV1DeclarationsLtFr0600ComputeResponseCountsBuilder {
        <PostV1DeclarationsLtFr0600ComputeResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtFr0600ComputeResponseCountsBuilder {
    sales_invoices: Option<i64>,
    purchase_invoices: Option<i64>,
}

impl PostV1DeclarationsLtFr0600ComputeResponseCountsBuilder {
    pub fn sales_invoices(mut self, value: i64) -> Self {
        self.sales_invoices = Some(value);
        self
    }

    pub fn purchase_invoices(mut self, value: i64) -> Self {
        self.purchase_invoices = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtFr0600ComputeResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sales_invoices`](PostV1DeclarationsLtFr0600ComputeResponseCountsBuilder::sales_invoices)
    /// - [`purchase_invoices`](PostV1DeclarationsLtFr0600ComputeResponseCountsBuilder::purchase_invoices)
    pub fn build(self) -> Result<PostV1DeclarationsLtFr0600ComputeResponseCounts, BuildError> {
        Ok(PostV1DeclarationsLtFr0600ComputeResponseCounts {
            sales_invoices: self
                .sales_invoices
                .ok_or_else(|| BuildError::missing_field("sales_invoices"))?,
            purchase_invoices: self
                .purchase_invoices
                .ok_or_else(|| BuildError::missing_field("purchase_invoices"))?,
        })
    }
}
