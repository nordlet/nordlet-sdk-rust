pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1LedgerStatementRowsListResponseRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub label: String,
    pub statement: PostV1LedgerStatementRowsListResponseRowsItemStatement,
    #[serde(default)]
    pub amount: String,
}

impl PostV1LedgerStatementRowsListResponseRowsItem {
    pub fn builder() -> PostV1LedgerStatementRowsListResponseRowsItemBuilder {
        <PostV1LedgerStatementRowsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerStatementRowsListResponseRowsItemBuilder {
    code: Option<String>,
    label: Option<String>,
    statement: Option<PostV1LedgerStatementRowsListResponseRowsItemStatement>,
    amount: Option<String>,
}

impl PostV1LedgerStatementRowsListResponseRowsItemBuilder {
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
        value: PostV1LedgerStatementRowsListResponseRowsItemStatement,
    ) -> Self {
        self.statement = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerStatementRowsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1LedgerStatementRowsListResponseRowsItemBuilder::code)
    /// - [`label`](PostV1LedgerStatementRowsListResponseRowsItemBuilder::label)
    /// - [`statement`](PostV1LedgerStatementRowsListResponseRowsItemBuilder::statement)
    /// - [`amount`](PostV1LedgerStatementRowsListResponseRowsItemBuilder::amount)
    pub fn build(self) -> Result<PostV1LedgerStatementRowsListResponseRowsItem, BuildError> {
        Ok(PostV1LedgerStatementRowsListResponseRowsItem {
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
