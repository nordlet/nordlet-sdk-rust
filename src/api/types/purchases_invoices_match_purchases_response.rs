pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvoicesMatchPurchasesResponse {
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    #[serde(rename = "orderId")]
    #[serde(default)]
    pub order_id: String,
    pub status: InvoicesMatchPurchasesResponseStatus,
    #[serde(default)]
    pub rows: Vec<InvoicesMatchPurchasesResponseRowsItem>,
}

impl InvoicesMatchPurchasesResponse {
    pub fn builder() -> InvoicesMatchPurchasesResponseBuilder {
        <InvoicesMatchPurchasesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesMatchPurchasesResponseBuilder {
    invoice_id: Option<String>,
    order_id: Option<String>,
    status: Option<InvoicesMatchPurchasesResponseStatus>,
    rows: Option<Vec<InvoicesMatchPurchasesResponseRowsItem>>,
}

impl InvoicesMatchPurchasesResponseBuilder {
    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.order_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: InvoicesMatchPurchasesResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<InvoicesMatchPurchasesResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesMatchPurchasesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_id`](InvoicesMatchPurchasesResponseBuilder::invoice_id)
    /// - [`order_id`](InvoicesMatchPurchasesResponseBuilder::order_id)
    /// - [`status`](InvoicesMatchPurchasesResponseBuilder::status)
    /// - [`rows`](InvoicesMatchPurchasesResponseBuilder::rows)
    pub fn build(self) -> Result<InvoicesMatchPurchasesResponse, BuildError> {
        Ok(InvoicesMatchPurchasesResponse {
            invoice_id: self
                .invoice_id
                .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
            order_id: self
                .order_id
                .ok_or_else(|| BuildError::missing_field("order_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
