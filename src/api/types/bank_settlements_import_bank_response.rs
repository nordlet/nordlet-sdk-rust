pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SettlementsImportBankResponse {
    pub format: SettlementsImportBankResponseFormat,
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
    pub batches: Vec<SettlementsImportBankResponseBatchesItem>,
}

impl SettlementsImportBankResponse {
    pub fn builder() -> SettlementsImportBankResponseBuilder {
        <SettlementsImportBankResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettlementsImportBankResponseBuilder {
    format: Option<SettlementsImportBankResponseFormat>,
    imported: Option<i64>,
    updated: Option<i64>,
    skipped: Option<i64>,
    skipped_unassigned: Option<i64>,
    skipped_payout_rows: Option<i64>,
    skipped_not_settled: Option<i64>,
    batches: Option<Vec<SettlementsImportBankResponseBatchesItem>>,
}

impl SettlementsImportBankResponseBuilder {
    pub fn format(mut self, value: SettlementsImportBankResponseFormat) -> Self {
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

    pub fn batches(mut self, value: Vec<SettlementsImportBankResponseBatchesItem>) -> Self {
        self.batches = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SettlementsImportBankResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`format`](SettlementsImportBankResponseBuilder::format)
    /// - [`imported`](SettlementsImportBankResponseBuilder::imported)
    /// - [`updated`](SettlementsImportBankResponseBuilder::updated)
    /// - [`skipped`](SettlementsImportBankResponseBuilder::skipped)
    /// - [`skipped_unassigned`](SettlementsImportBankResponseBuilder::skipped_unassigned)
    /// - [`skipped_payout_rows`](SettlementsImportBankResponseBuilder::skipped_payout_rows)
    /// - [`skipped_not_settled`](SettlementsImportBankResponseBuilder::skipped_not_settled)
    /// - [`batches`](SettlementsImportBankResponseBuilder::batches)
    pub fn build(self) -> Result<SettlementsImportBankResponse, BuildError> {
        Ok(SettlementsImportBankResponse {
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
