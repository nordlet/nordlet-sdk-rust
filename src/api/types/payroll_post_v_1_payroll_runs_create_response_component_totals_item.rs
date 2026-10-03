pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1PayrollRunsCreateResponseComponentTotalsItem {
    #[serde(default)]
    pub code: String,
    pub kind: PostV1PayrollRunsCreateResponseComponentTotalsItemKind,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
}

impl PostV1PayrollRunsCreateResponseComponentTotalsItem {
    pub fn builder() -> PostV1PayrollRunsCreateResponseComponentTotalsItemBuilder {
        <PostV1PayrollRunsCreateResponseComponentTotalsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollRunsCreateResponseComponentTotalsItemBuilder {
    code: Option<String>,
    kind: Option<PostV1PayrollRunsCreateResponseComponentTotalsItemKind>,
    amount: Option<String>,
    rate: Option<String>,
    base: Option<String>,
}

impl PostV1PayrollRunsCreateResponseComponentTotalsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn kind(mut self, value: PostV1PayrollRunsCreateResponseComponentTotalsItemKind) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1PayrollRunsCreateResponseComponentTotalsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1PayrollRunsCreateResponseComponentTotalsItemBuilder::code)
    /// - [`kind`](PostV1PayrollRunsCreateResponseComponentTotalsItemBuilder::kind)
    /// - [`amount`](PostV1PayrollRunsCreateResponseComponentTotalsItemBuilder::amount)
    pub fn build(self) -> Result<PostV1PayrollRunsCreateResponseComponentTotalsItem, BuildError> {
        Ok(PostV1PayrollRunsCreateResponseComponentTotalsItem {
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
