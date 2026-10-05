pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StatementRowsListLedgerResponseAccountsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "rowCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<StatementRowsListLedgerResponseAccountsItemSource>,
    #[serde(default)]
    pub amount: String,
}

impl StatementRowsListLedgerResponseAccountsItem {
    pub fn builder() -> StatementRowsListLedgerResponseAccountsItemBuilder {
        <StatementRowsListLedgerResponseAccountsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatementRowsListLedgerResponseAccountsItemBuilder {
    code: Option<String>,
    name: Option<String>,
    r#type: Option<String>,
    row_code: Option<String>,
    source: Option<StatementRowsListLedgerResponseAccountsItemSource>,
    amount: Option<String>,
}

impl StatementRowsListLedgerResponseAccountsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn row_code(mut self, value: impl Into<String>) -> Self {
        self.row_code = Some(value.into());
        self
    }

    pub fn source(mut self, value: StatementRowsListLedgerResponseAccountsItemSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StatementRowsListLedgerResponseAccountsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](StatementRowsListLedgerResponseAccountsItemBuilder::code)
    /// - [`name`](StatementRowsListLedgerResponseAccountsItemBuilder::name)
    /// - [`r#type`](StatementRowsListLedgerResponseAccountsItemBuilder::r#type)
    /// - [`amount`](StatementRowsListLedgerResponseAccountsItemBuilder::amount)
    pub fn build(self) -> Result<StatementRowsListLedgerResponseAccountsItem, BuildError> {
        Ok(StatementRowsListLedgerResponseAccountsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            row_code: self.row_code,
            source: self.source,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
