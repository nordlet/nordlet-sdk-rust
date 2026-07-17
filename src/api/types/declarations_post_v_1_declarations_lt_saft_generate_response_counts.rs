pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtSaftGenerateResponseCounts {
    #[serde(default)]
    pub accounts: i64,
    #[serde(default)]
    pub customers: i64,
    #[serde(default)]
    pub suppliers: i64,
    #[serde(rename = "glTransactions")]
    #[serde(default)]
    pub gl_transactions: i64,
    #[serde(rename = "salesInvoices")]
    #[serde(default)]
    pub sales_invoices: i64,
    #[serde(rename = "purchaseInvoices")]
    #[serde(default)]
    pub purchase_invoices: i64,
    #[serde(default)]
    pub payments: i64,
    #[serde(rename = "stockMovements")]
    #[serde(default)]
    pub stock_movements: i64,
    #[serde(rename = "assetTransactions")]
    #[serde(default)]
    pub asset_transactions: i64,
}

impl PostV1DeclarationsLtSaftGenerateResponseCounts {
    pub fn builder() -> PostV1DeclarationsLtSaftGenerateResponseCountsBuilder {
        <PostV1DeclarationsLtSaftGenerateResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtSaftGenerateResponseCountsBuilder {
    accounts: Option<i64>,
    customers: Option<i64>,
    suppliers: Option<i64>,
    gl_transactions: Option<i64>,
    sales_invoices: Option<i64>,
    purchase_invoices: Option<i64>,
    payments: Option<i64>,
    stock_movements: Option<i64>,
    asset_transactions: Option<i64>,
}

impl PostV1DeclarationsLtSaftGenerateResponseCountsBuilder {
    pub fn accounts(mut self, value: i64) -> Self {
        self.accounts = Some(value);
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

    pub fn gl_transactions(mut self, value: i64) -> Self {
        self.gl_transactions = Some(value);
        self
    }

    pub fn sales_invoices(mut self, value: i64) -> Self {
        self.sales_invoices = Some(value);
        self
    }

    pub fn purchase_invoices(mut self, value: i64) -> Self {
        self.purchase_invoices = Some(value);
        self
    }

    pub fn payments(mut self, value: i64) -> Self {
        self.payments = Some(value);
        self
    }

    pub fn stock_movements(mut self, value: i64) -> Self {
        self.stock_movements = Some(value);
        self
    }

    pub fn asset_transactions(mut self, value: i64) -> Self {
        self.asset_transactions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtSaftGenerateResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`accounts`](PostV1DeclarationsLtSaftGenerateResponseCountsBuilder::accounts)
    /// - [`customers`](PostV1DeclarationsLtSaftGenerateResponseCountsBuilder::customers)
    /// - [`suppliers`](PostV1DeclarationsLtSaftGenerateResponseCountsBuilder::suppliers)
    /// - [`gl_transactions`](PostV1DeclarationsLtSaftGenerateResponseCountsBuilder::gl_transactions)
    /// - [`sales_invoices`](PostV1DeclarationsLtSaftGenerateResponseCountsBuilder::sales_invoices)
    /// - [`purchase_invoices`](PostV1DeclarationsLtSaftGenerateResponseCountsBuilder::purchase_invoices)
    /// - [`payments`](PostV1DeclarationsLtSaftGenerateResponseCountsBuilder::payments)
    /// - [`stock_movements`](PostV1DeclarationsLtSaftGenerateResponseCountsBuilder::stock_movements)
    /// - [`asset_transactions`](PostV1DeclarationsLtSaftGenerateResponseCountsBuilder::asset_transactions)
    pub fn build(self) -> Result<PostV1DeclarationsLtSaftGenerateResponseCounts, BuildError> {
        Ok(PostV1DeclarationsLtSaftGenerateResponseCounts {
            accounts: self
                .accounts
                .ok_or_else(|| BuildError::missing_field("accounts"))?,
            customers: self
                .customers
                .ok_or_else(|| BuildError::missing_field("customers"))?,
            suppliers: self
                .suppliers
                .ok_or_else(|| BuildError::missing_field("suppliers"))?,
            gl_transactions: self
                .gl_transactions
                .ok_or_else(|| BuildError::missing_field("gl_transactions"))?,
            sales_invoices: self
                .sales_invoices
                .ok_or_else(|| BuildError::missing_field("sales_invoices"))?,
            purchase_invoices: self
                .purchase_invoices
                .ok_or_else(|| BuildError::missing_field("purchase_invoices"))?,
            payments: self
                .payments
                .ok_or_else(|| BuildError::missing_field("payments"))?,
            stock_movements: self
                .stock_movements
                .ok_or_else(|| BuildError::missing_field("stock_movements"))?,
            asset_transactions: self
                .asset_transactions
                .ok_or_else(|| BuildError::missing_field("asset_transactions"))?,
        })
    }
}
