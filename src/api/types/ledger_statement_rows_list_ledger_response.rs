pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StatementRowsListLedgerResponse {
    #[serde(default)]
    pub scheme: StatementRowsListLedgerResponseScheme,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(default)]
    pub accounts: Vec<StatementRowsListLedgerResponseAccountsItem>,
    #[serde(default)]
    pub rows: Vec<StatementRowsListLedgerResponseRowsItem>,
    #[serde(default)]
    pub unmapped: Vec<String>,
}

impl StatementRowsListLedgerResponse {
    pub fn builder() -> StatementRowsListLedgerResponseBuilder {
        <StatementRowsListLedgerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatementRowsListLedgerResponseBuilder {
    scheme: Option<StatementRowsListLedgerResponseScheme>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    accounts: Option<Vec<StatementRowsListLedgerResponseAccountsItem>>,
    rows: Option<Vec<StatementRowsListLedgerResponseRowsItem>>,
    unmapped: Option<Vec<String>>,
}

impl StatementRowsListLedgerResponseBuilder {
    pub fn scheme(mut self, value: StatementRowsListLedgerResponseScheme) -> Self {
        self.scheme = Some(value);
        self
    }

    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn accounts(mut self, value: Vec<StatementRowsListLedgerResponseAccountsItem>) -> Self {
        self.accounts = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<StatementRowsListLedgerResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn unmapped(mut self, value: Vec<String>) -> Self {
        self.unmapped = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StatementRowsListLedgerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scheme`](StatementRowsListLedgerResponseBuilder::scheme)
    /// - [`from_date`](StatementRowsListLedgerResponseBuilder::from_date)
    /// - [`to_date`](StatementRowsListLedgerResponseBuilder::to_date)
    /// - [`accounts`](StatementRowsListLedgerResponseBuilder::accounts)
    /// - [`rows`](StatementRowsListLedgerResponseBuilder::rows)
    /// - [`unmapped`](StatementRowsListLedgerResponseBuilder::unmapped)
    pub fn build(self) -> Result<StatementRowsListLedgerResponse, BuildError> {
        Ok(StatementRowsListLedgerResponse {
            scheme: self
                .scheme
                .ok_or_else(|| BuildError::missing_field("scheme"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            accounts: self
                .accounts
                .ok_or_else(|| BuildError::missing_field("accounts"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            unmapped: self
                .unmapped
                .ok_or_else(|| BuildError::missing_field("unmapped"))?,
        })
    }
}
