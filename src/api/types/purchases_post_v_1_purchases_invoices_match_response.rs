pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesInvoicesMatchResponse {
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    #[serde(rename = "orderId")]
    #[serde(default)]
    pub order_id: String,
    pub status: PostV1PurchasesInvoicesMatchResponseStatus,
    #[serde(default)]
    pub rows: Vec<PostV1PurchasesInvoicesMatchResponseRowsItem>,
}

impl PostV1PurchasesInvoicesMatchResponse {
    pub fn builder() -> PostV1PurchasesInvoicesMatchResponseBuilder {
        <PostV1PurchasesInvoicesMatchResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesInvoicesMatchResponseBuilder {
    invoice_id: Option<String>,
    order_id: Option<String>,
    status: Option<PostV1PurchasesInvoicesMatchResponseStatus>,
    rows: Option<Vec<PostV1PurchasesInvoicesMatchResponseRowsItem>>,
}

impl PostV1PurchasesInvoicesMatchResponseBuilder {
    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.order_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostV1PurchasesInvoicesMatchResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<PostV1PurchasesInvoicesMatchResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesInvoicesMatchResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_id`](PostV1PurchasesInvoicesMatchResponseBuilder::invoice_id)
    /// - [`order_id`](PostV1PurchasesInvoicesMatchResponseBuilder::order_id)
    /// - [`status`](PostV1PurchasesInvoicesMatchResponseBuilder::status)
    /// - [`rows`](PostV1PurchasesInvoicesMatchResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1PurchasesInvoicesMatchResponse, BuildError> {
        Ok(PostV1PurchasesInvoicesMatchResponse {
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
