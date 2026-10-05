pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StatementRowsListLedgerResponseSchemeRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub label: String,
    pub statement: StatementRowsListLedgerResponseSchemeRowsItemStatement,
}

impl StatementRowsListLedgerResponseSchemeRowsItem {
    pub fn builder() -> StatementRowsListLedgerResponseSchemeRowsItemBuilder {
        <StatementRowsListLedgerResponseSchemeRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatementRowsListLedgerResponseSchemeRowsItemBuilder {
    code: Option<String>,
    label: Option<String>,
    statement: Option<StatementRowsListLedgerResponseSchemeRowsItemStatement>,
}

impl StatementRowsListLedgerResponseSchemeRowsItemBuilder {
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
        value: StatementRowsListLedgerResponseSchemeRowsItemStatement,
    ) -> Self {
        self.statement = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StatementRowsListLedgerResponseSchemeRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](StatementRowsListLedgerResponseSchemeRowsItemBuilder::code)
    /// - [`label`](StatementRowsListLedgerResponseSchemeRowsItemBuilder::label)
    /// - [`statement`](StatementRowsListLedgerResponseSchemeRowsItemBuilder::statement)
    pub fn build(self) -> Result<StatementRowsListLedgerResponseSchemeRowsItem, BuildError> {
        Ok(StatementRowsListLedgerResponseSchemeRowsItem {
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
