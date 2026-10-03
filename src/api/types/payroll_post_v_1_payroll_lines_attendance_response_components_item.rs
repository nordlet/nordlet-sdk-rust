pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1PayrollLinesAttendanceResponseComponentsItem {
    #[serde(default)]
    pub code: String,
    pub kind: PostV1PayrollLinesAttendanceResponseComponentsItemKind,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
}

impl PostV1PayrollLinesAttendanceResponseComponentsItem {
    pub fn builder() -> PostV1PayrollLinesAttendanceResponseComponentsItemBuilder {
        <PostV1PayrollLinesAttendanceResponseComponentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollLinesAttendanceResponseComponentsItemBuilder {
    code: Option<String>,
    kind: Option<PostV1PayrollLinesAttendanceResponseComponentsItemKind>,
    amount: Option<String>,
    rate: Option<String>,
    base: Option<String>,
}

impl PostV1PayrollLinesAttendanceResponseComponentsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn kind(mut self, value: PostV1PayrollLinesAttendanceResponseComponentsItemKind) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1PayrollLinesAttendanceResponseComponentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1PayrollLinesAttendanceResponseComponentsItemBuilder::code)
    /// - [`kind`](PostV1PayrollLinesAttendanceResponseComponentsItemBuilder::kind)
    /// - [`amount`](PostV1PayrollLinesAttendanceResponseComponentsItemBuilder::amount)
    pub fn build(self) -> Result<PostV1PayrollLinesAttendanceResponseComponentsItem, BuildError> {
        Ok(PostV1PayrollLinesAttendanceResponseComponentsItem {
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
