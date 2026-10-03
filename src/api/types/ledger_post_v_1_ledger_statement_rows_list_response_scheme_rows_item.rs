pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1LedgerStatementRowsListResponseSchemeRowsItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub label: String,
    pub statement: PostV1LedgerStatementRowsListResponseSchemeRowsItemStatement,
}

impl PostV1LedgerStatementRowsListResponseSchemeRowsItem {
    pub fn builder() -> PostV1LedgerStatementRowsListResponseSchemeRowsItemBuilder {
        <PostV1LedgerStatementRowsListResponseSchemeRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerStatementRowsListResponseSchemeRowsItemBuilder {
    code: Option<String>,
    label: Option<String>,
    statement: Option<PostV1LedgerStatementRowsListResponseSchemeRowsItemStatement>,
}

impl PostV1LedgerStatementRowsListResponseSchemeRowsItemBuilder {
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
        value: PostV1LedgerStatementRowsListResponseSchemeRowsItemStatement,
    ) -> Self {
        self.statement = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerStatementRowsListResponseSchemeRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1LedgerStatementRowsListResponseSchemeRowsItemBuilder::code)
    /// - [`label`](PostV1LedgerStatementRowsListResponseSchemeRowsItemBuilder::label)
    /// - [`statement`](PostV1LedgerStatementRowsListResponseSchemeRowsItemBuilder::statement)
    pub fn build(self) -> Result<PostV1LedgerStatementRowsListResponseSchemeRowsItem, BuildError> {
        Ok(PostV1LedgerStatementRowsListResponseSchemeRowsItem {
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
