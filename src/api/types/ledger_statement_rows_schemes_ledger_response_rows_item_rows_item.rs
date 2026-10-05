pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StatementRowsSchemesLedgerResponseRowsItemRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub label: String,
    pub statement: StatementRowsSchemesLedgerResponseRowsItemRowsItemStatement,
}

impl StatementRowsSchemesLedgerResponseRowsItemRowsItem {
    pub fn builder() -> StatementRowsSchemesLedgerResponseRowsItemRowsItemBuilder {
        <StatementRowsSchemesLedgerResponseRowsItemRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatementRowsSchemesLedgerResponseRowsItemRowsItemBuilder {
    code: Option<String>,
    label: Option<String>,
    statement: Option<StatementRowsSchemesLedgerResponseRowsItemRowsItemStatement>,
}

impl StatementRowsSchemesLedgerResponseRowsItemRowsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn statement(
        mut self,
        value: StatementRowsSchemesLedgerResponseRowsItemRowsItemStatement,
    ) -> Self {
        self.statement = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StatementRowsSchemesLedgerResponseRowsItemRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](StatementRowsSchemesLedgerResponseRowsItemRowsItemBuilder::code)
    /// - [`label`](StatementRowsSchemesLedgerResponseRowsItemRowsItemBuilder::label)
    /// - [`statement`](StatementRowsSchemesLedgerResponseRowsItemRowsItemBuilder::statement)
    pub fn build(self) -> Result<StatementRowsSchemesLedgerResponseRowsItemRowsItem, BuildError> {
        Ok(StatementRowsSchemesLedgerResponseRowsItemRowsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            statement: self
                .statement
                .ok_or_else(|| BuildError::missing_field("statement"))?,
        })
    }
}
