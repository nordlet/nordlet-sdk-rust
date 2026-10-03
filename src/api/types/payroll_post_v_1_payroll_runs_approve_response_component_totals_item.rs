pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1PayrollRunsApproveResponseComponentTotalsItem {
    #[serde(default)]
    pub code: String,
    pub kind: PostV1PayrollRunsApproveResponseComponentTotalsItemKind,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
}

impl PostV1PayrollRunsApproveResponseComponentTotalsItem {
    pub fn builder() -> PostV1PayrollRunsApproveResponseComponentTotalsItemBuilder {
        <PostV1PayrollRunsApproveResponseComponentTotalsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollRunsApproveResponseComponentTotalsItemBuilder {
    code: Option<String>,
    kind: Option<PostV1PayrollRunsApproveResponseComponentTotalsItemKind>,
    amount: Option<String>,
    rate: Option<String>,
    base: Option<String>,
}

impl PostV1PayrollRunsApproveResponseComponentTotalsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn kind(mut self, value: PostV1PayrollRunsApproveResponseComponentTotalsItemKind) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1PayrollRunsApproveResponseComponentTotalsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1PayrollRunsApproveResponseComponentTotalsItemBuilder::code)
    /// - [`kind`](PostV1PayrollRunsApproveResponseComponentTotalsItemBuilder::kind)
    /// - [`amount`](PostV1PayrollRunsApproveResponseComponentTotalsItemBuilder::amount)
    pub fn build(self) -> Result<PostV1PayrollRunsApproveResponseComponentTotalsItem, BuildError> {
        Ok(PostV1PayrollRunsApproveResponseComponentTotalsItem {
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
