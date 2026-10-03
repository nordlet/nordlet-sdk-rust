pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1PayrollRunsListResponseRowsItemComponentTotalsItem {
    #[serde(default)]
    pub code: String,
    pub kind: PostV1PayrollRunsListResponseRowsItemComponentTotalsItemKind,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
}

impl PostV1PayrollRunsListResponseRowsItemComponentTotalsItem {
    pub fn builder() -> PostV1PayrollRunsListResponseRowsItemComponentTotalsItemBuilder {
        <PostV1PayrollRunsListResponseRowsItemComponentTotalsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollRunsListResponseRowsItemComponentTotalsItemBuilder {
    code: Option<String>,
    kind: Option<PostV1PayrollRunsListResponseRowsItemComponentTotalsItemKind>,
    amount: Option<String>,
    rate: Option<String>,
    base: Option<String>,
}

impl PostV1PayrollRunsListResponseRowsItemComponentTotalsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn kind(
        mut self,
        value: PostV1PayrollRunsListResponseRowsItemComponentTotalsItemKind,
    ) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn rate(mut self, value: impl Into<String>) -> Self {
        self.rate = Some(value.into());
        self
    }

    pub fn base(mut self, value: impl Into<String>) -> Self {
        self.base = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollRunsListResponseRowsItemComponentTotalsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1PayrollRunsListResponseRowsItemComponentTotalsItemBuilder::code)
    /// - [`kind`](PostV1PayrollRunsListResponseRowsItemComponentTotalsItemBuilder::kind)
    /// - [`amount`](PostV1PayrollRunsListResponseRowsItemComponentTotalsItemBuilder::amount)
    pub fn build(
        self,
    ) -> Result<PostV1PayrollRunsListResponseRowsItemComponentTotalsItem, BuildError> {
        Ok(PostV1PayrollRunsListResponseRowsItemComponentTotalsItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            rate: self.rate,
            base: self.base,
        })
    }
}
