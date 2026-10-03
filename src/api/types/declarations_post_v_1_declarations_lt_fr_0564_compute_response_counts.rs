pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtFr0564ComputeResponseCounts {
    #[serde(rename = "salesInvoices")]
    #[serde(default)]
    pub sales_invoices: i64,
}

impl PostV1DeclarationsLtFr0564ComputeResponseCounts {
    pub fn builder() -> PostV1DeclarationsLtFr0564ComputeResponseCountsBuilder {
        <PostV1DeclarationsLtFr0564ComputeResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtFr0564ComputeResponseCountsBuilder {
    sales_invoices: Option<i64>,
}

impl PostV1DeclarationsLtFr0564ComputeResponseCountsBuilder {
    pub fn sales_invoices(mut self, value: i64) -> Self {
        self.sales_invoices = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtFr0564ComputeResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sales_invoices`](PostV1DeclarationsLtFr0564ComputeResponseCountsBuilder::sales_invoices)
    pub fn build(self) -> Result<PostV1DeclarationsLtFr0564ComputeResponseCounts, BuildError> {
        Ok(PostV1DeclarationsLtFr0564ComputeResponseCounts {
            sales_invoices: self
                .sales_invoices
                .ok_or_else(|| BuildError::missing_field("sales_invoices"))?,
        })
    }
}
