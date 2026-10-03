pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1LedgerStatementRowsSchemesResponseRowsItemRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub label: String,
    pub statement: PostV1LedgerStatementRowsSchemesResponseRowsItemRowsItemStatement,
}

impl PostV1LedgerStatementRowsSchemesResponseRowsItemRowsItem {
    pub fn builder() -> PostV1LedgerStatementRowsSchemesResponseRowsItemRowsItemBuilder {
        <PostV1LedgerStatementRowsSchemesResponseRowsItemRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerStatementRowsSchemesResponseRowsItemRowsItemBuilder {
    code: Option<String>,
    label: Option<String>,
    statement: Option<PostV1LedgerStatementRowsSchemesResponseRowsItemRowsItemStatement>,
}

impl PostV1LedgerStatementRowsSchemesResponseRowsItemRowsItemBuilder {
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
        value: PostV1LedgerStatementRowsSchemesResponseRowsItemRowsItemStatement,
    ) -> Self {
        self.statement = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerStatementRowsSchemesResponseRowsItemRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1LedgerStatementRowsSchemesResponseRowsItemRowsItemBuilder::code)
    /// - [`label`](PostV1LedgerStatementRowsSchemesResponseRowsItemRowsItemBuilder::label)
    /// - [`statement`](PostV1LedgerStatementRowsSchemesResponseRowsItemRowsItemBuilder::statement)
    pub fn build(
        self,
    ) -> Result<PostV1LedgerStatementRowsSchemesResponseRowsItemRowsItem, BuildError> {
        Ok(PostV1LedgerStatementRowsSchemesResponseRowsItemRowsItem {
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
