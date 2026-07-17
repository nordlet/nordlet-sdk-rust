pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1BankSettlementsImportResponse {
    pub format: PostV1BankSettlementsImportResponseFormat,
    #[serde(default)]
    pub imported: i64,
    #[serde(default)]
    pub updated: i64,
    #[serde(default)]
    pub skipped: i64,
    #[serde(rename = "skippedUnassigned")]
    #[serde(default)]
    pub skipped_unassigned: i64,
    #[serde(rename = "skippedPayoutRows")]
    #[serde(default)]
    pub skipped_payout_rows: i64,
    #[serde(rename = "skippedNotSettled")]
    #[serde(default)]
    pub skipped_not_settled: i64,
    #[serde(default)]
    pub batches: Vec<PostV1BankSettlementsImportResponseBatchesItem>,
}

impl PostV1BankSettlementsImportResponse {
    pub fn builder() -> PostV1BankSettlementsImportResponseBuilder {
        <PostV1BankSettlementsImportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankSettlementsImportResponseBuilder {
    format: Option<PostV1BankSettlementsImportResponseFormat>,
    imported: Option<i64>,
    updated: Option<i64>,
    skipped: Option<i64>,
    skipped_unassigned: Option<i64>,
    skipped_payout_rows: Option<i64>,
    skipped_not_settled: Option<i64>,
    batches: Option<Vec<PostV1BankSettlementsImportResponseBatchesItem>>,
}

impl PostV1BankSettlementsImportResponseBuilder {
    pub fn format(mut self, value: PostV1BankSettlementsImportResponseFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn imported(mut self, value: i64) -> Self {
        self.imported = Some(value);
        self
    }

    pub fn updated(mut self, value: i64) -> Self {
        self.updated = Some(value);
        self
    }

    pub fn skipped(mut self, value: i64) -> Self {
        self.skipped = Some(value);
        self
    }

    pub fn skipped_unassigned(mut self, value: i64) -> Self {
        self.skipped_unassigned = Some(value);
        self
    }

    pub fn skipped_payout_rows(mut self, value: i64) -> Self {
        self.skipped_payout_rows = Some(value);
        self
    }

    pub fn skipped_not_settled(mut self, value: i64) -> Self {
        self.skipped_not_settled = Some(value);
        self
    }

    pub fn batches(mut self, value: Vec<PostV1BankSettlementsImportResponseBatchesItem>) -> Self {
        self.batches = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankSettlementsImportResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`format`](PostV1BankSettlementsImportResponseBuilder::format)
    /// - [`imported`](PostV1BankSettlementsImportResponseBuilder::imported)
    /// - [`updated`](PostV1BankSettlementsImportResponseBuilder::updated)
    /// - [`skipped`](PostV1BankSettlementsImportResponseBuilder::skipped)
    /// - [`skipped_unassigned`](PostV1BankSettlementsImportResponseBuilder::skipped_unassigned)
    /// - [`skipped_payout_rows`](PostV1BankSettlementsImportResponseBuilder::skipped_payout_rows)
    /// - [`skipped_not_settled`](PostV1BankSettlementsImportResponseBuilder::skipped_not_settled)
    /// - [`batches`](PostV1BankSettlementsImportResponseBuilder::batches)
    pub fn build(self) -> Result<PostV1BankSettlementsImportResponse, BuildError> {
        Ok(PostV1BankSettlementsImportResponse {
            format: self
                .format
                .ok_or_else(|| BuildError::missing_field("format"))?,
            imported: self
                .imported
                .ok_or_else(|| BuildError::missing_field("imported"))?,
            updated: self
                .updated
                .ok_or_else(|| BuildError::missing_field("updated"))?,
            skipped: self
                .skipped
                .ok_or_else(|| BuildError::missing_field("skipped"))?,
            skipped_unassigned: self
                .skipped_unassigned
                .ok_or_else(|| BuildError::missing_field("skipped_unassigned"))?,
            skipped_payout_rows: self
                .skipped_payout_rows
                .ok_or_else(|| BuildError::missing_field("skipped_payout_rows"))?,
            skipped_not_settled: self
                .skipped_not_settled
                .ok_or_else(|| BuildError::missing_field("skipped_not_settled"))?,
            batches: self
                .batches
                .ok_or_else(|| BuildError::missing_field("batches"))?,
        })
    }
}
