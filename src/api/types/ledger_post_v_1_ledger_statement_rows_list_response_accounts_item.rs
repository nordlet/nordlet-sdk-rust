pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerStatementRowsListResponseAccountsItem {
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
    pub source: Option<PostV1LedgerStatementRowsListResponseAccountsItemSource>,
    #[serde(default)]
    pub amount: String,
}

impl PostV1LedgerStatementRowsListResponseAccountsItem {
    pub fn builder() -> PostV1LedgerStatementRowsListResponseAccountsItemBuilder {
        <PostV1LedgerStatementRowsListResponseAccountsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerStatementRowsListResponseAccountsItemBuilder {
    code: Option<String>,
    name: Option<String>,
    r#type: Option<String>,
    row_code: Option<String>,
    source: Option<PostV1LedgerStatementRowsListResponseAccountsItemSource>,
    amount: Option<String>,
}

impl PostV1LedgerStatementRowsListResponseAccountsItemBuilder {
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

    pub fn source(
        mut self,
        value: PostV1LedgerStatementRowsListResponseAccountsItemSource,
    ) -> Self {
        self.source = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerStatementRowsListResponseAccountsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1LedgerStatementRowsListResponseAccountsItemBuilder::code)
    /// - [`name`](PostV1LedgerStatementRowsListResponseAccountsItemBuilder::name)
    /// - [`r#type`](PostV1LedgerStatementRowsListResponseAccountsItemBuilder::r#type)
    /// - [`amount`](PostV1LedgerStatementRowsListResponseAccountsItemBuilder::amount)
    pub fn build(self) -> Result<PostV1LedgerStatementRowsListResponseAccountsItem, BuildError> {
        Ok(PostV1LedgerStatementRowsListResponseAccountsItem {
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
