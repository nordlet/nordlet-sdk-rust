pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlJpkKrGenerateDeclarationsResponseCounts {
    #[serde(default)]
    pub accounts: i64,
    #[serde(rename = "journalRows")]
    #[serde(default)]
    pub journal_rows: i64,
    #[serde(rename = "entryRows")]
    #[serde(default)]
    pub entry_rows: i64,
}

impl PlJpkKrGenerateDeclarationsResponseCounts {
    pub fn builder() -> PlJpkKrGenerateDeclarationsResponseCountsBuilder {
        <PlJpkKrGenerateDeclarationsResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlJpkKrGenerateDeclarationsResponseCountsBuilder {
    accounts: Option<i64>,
    journal_rows: Option<i64>,
    entry_rows: Option<i64>,
}

impl PlJpkKrGenerateDeclarationsResponseCountsBuilder {
    pub fn accounts(mut self, value: i64) -> Self {
        self.accounts = Some(value);
        self
    }

    pub fn journal_rows(mut self, value: i64) -> Self {
        self.journal_rows = Some(value);
        self
    }

    pub fn entry_rows(mut self, value: i64) -> Self {
        self.entry_rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlJpkKrGenerateDeclarationsResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`accounts`](PlJpkKrGenerateDeclarationsResponseCountsBuilder::accounts)
    /// - [`journal_rows`](PlJpkKrGenerateDeclarationsResponseCountsBuilder::journal_rows)
    /// - [`entry_rows`](PlJpkKrGenerateDeclarationsResponseCountsBuilder::entry_rows)
    pub fn build(self) -> Result<PlJpkKrGenerateDeclarationsResponseCounts, BuildError> {
        Ok(PlJpkKrGenerateDeclarationsResponseCounts {
            accounts: self
                .accounts
                .ok_or_else(|| BuildError::missing_field("accounts"))?,
            journal_rows: self
                .journal_rows
                .ok_or_else(|| BuildError::missing_field("journal_rows"))?,
            entry_rows: self
                .entry_rows
                .ok_or_else(|| BuildError::missing_field("entry_rows"))?,
        })
    }
}
