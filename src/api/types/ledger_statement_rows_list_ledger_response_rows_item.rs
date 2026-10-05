pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StatementRowsListLedgerResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub label: String,
    pub statement: StatementRowsListLedgerResponseRowsItemStatement,
    #[serde(default)]
    pub amount: String,
}

impl StatementRowsListLedgerResponseRowsItem {
    pub fn builder() -> StatementRowsListLedgerResponseRowsItemBuilder {
        <StatementRowsListLedgerResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatementRowsListLedgerResponseRowsItemBuilder {
    code: Option<String>,
    label: Option<String>,
    statement: Option<StatementRowsListLedgerResponseRowsItemStatement>,
    amount: Option<String>,
}

impl StatementRowsListLedgerResponseRowsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn statement(mut self, value: StatementRowsListLedgerResponseRowsItemStatement) -> Self {
        self.statement = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StatementRowsListLedgerResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](StatementRowsListLedgerResponseRowsItemBuilder::code)
    /// - [`label`](StatementRowsListLedgerResponseRowsItemBuilder::label)
    /// - [`statement`](StatementRowsListLedgerResponseRowsItemBuilder::statement)
    /// - [`amount`](StatementRowsListLedgerResponseRowsItemBuilder::amount)
    pub fn build(self) -> Result<StatementRowsListLedgerResponseRowsItem, BuildError> {
        Ok(StatementRowsListLedgerResponseRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            statement: self
                .statement
                .ok_or_else(|| BuildError::missing_field("statement"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
