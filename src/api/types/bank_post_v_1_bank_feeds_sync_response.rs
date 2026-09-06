pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankFeedsSyncResponse {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(default)]
    pub imported: i64,
    #[serde(default)]
    pub skipped: i64,
    #[serde(default)]
    pub posted: i64,
    #[serde(rename = "partnersCreated")]
    #[serde(default)]
    pub partners_created: i64,
    #[serde(rename = "invoicesCreated")]
    #[serde(default)]
    pub invoices_created: i64,
    #[serde(rename = "invoicesLinked")]
    #[serde(default)]
    pub invoices_linked: i64,
    #[serde(rename = "paymentsMatched")]
    #[serde(default)]
    pub payments_matched: i64,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub accounts: Vec<PostV1BankFeedsSyncResponseAccountsItem>,
}

impl PostV1BankFeedsSyncResponse {
    pub fn builder() -> PostV1BankFeedsSyncResponseBuilder {
        <PostV1BankFeedsSyncResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankFeedsSyncResponseBuilder {
    connection_id: Option<String>,
    imported: Option<i64>,
    skipped: Option<i64>,
    posted: Option<i64>,
    partners_created: Option<i64>,
    invoices_created: Option<i64>,
    invoices_linked: Option<i64>,
    payments_matched: Option<i64>,
    warnings: Option<Vec<String>>,
    accounts: Option<Vec<PostV1BankFeedsSyncResponseAccountsItem>>,
}

impl PostV1BankFeedsSyncResponseBuilder {
    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    pub fn imported(mut self, value: i64) -> Self {
        self.imported = Some(value);
        self
    }

    pub fn skipped(mut self, value: i64) -> Self {
        self.skipped = Some(value);
        self
    }

    pub fn posted(mut self, value: i64) -> Self {
        self.posted = Some(value);
        self
    }

    pub fn partners_created(mut self, value: i64) -> Self {
        self.partners_created = Some(value);
        self
    }

    pub fn invoices_created(mut self, value: i64) -> Self {
        self.invoices_created = Some(value);
        self
    }

    pub fn invoices_linked(mut self, value: i64) -> Self {
        self.invoices_linked = Some(value);
        self
    }

    pub fn payments_matched(mut self, value: i64) -> Self {
        self.payments_matched = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn accounts(mut self, value: Vec<PostV1BankFeedsSyncResponseAccountsItem>) -> Self {
        self.accounts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankFeedsSyncResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](PostV1BankFeedsSyncResponseBuilder::connection_id)
    /// - [`imported`](PostV1BankFeedsSyncResponseBuilder::imported)
    /// - [`skipped`](PostV1BankFeedsSyncResponseBuilder::skipped)
    /// - [`posted`](PostV1BankFeedsSyncResponseBuilder::posted)
    /// - [`partners_created`](PostV1BankFeedsSyncResponseBuilder::partners_created)
    /// - [`invoices_created`](PostV1BankFeedsSyncResponseBuilder::invoices_created)
    /// - [`invoices_linked`](PostV1BankFeedsSyncResponseBuilder::invoices_linked)
    /// - [`payments_matched`](PostV1BankFeedsSyncResponseBuilder::payments_matched)
    /// - [`warnings`](PostV1BankFeedsSyncResponseBuilder::warnings)
    /// - [`accounts`](PostV1BankFeedsSyncResponseBuilder::accounts)
    pub fn build(self) -> Result<PostV1BankFeedsSyncResponse, BuildError> {
        Ok(PostV1BankFeedsSyncResponse {
            connection_id: self
                .connection_id
                .ok_or_else(|| BuildError::missing_field("connection_id"))?,
            imported: self
                .imported
                .ok_or_else(|| BuildError::missing_field("imported"))?,
            skipped: self
                .skipped
                .ok_or_else(|| BuildError::missing_field("skipped"))?,
            posted: self
                .posted
                .ok_or_else(|| BuildError::missing_field("posted"))?,
            partners_created: self
                .partners_created
                .ok_or_else(|| BuildError::missing_field("partners_created"))?,
            invoices_created: self
                .invoices_created
                .ok_or_else(|| BuildError::missing_field("invoices_created"))?,
            invoices_linked: self
                .invoices_linked
                .ok_or_else(|| BuildError::missing_field("invoices_linked"))?,
            payments_matched: self
                .payments_matched
                .ok_or_else(|| BuildError::missing_field("payments_matched"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            accounts: self
                .accounts
                .ok_or_else(|| BuildError::missing_field("accounts"))?,
        })
    }
}
