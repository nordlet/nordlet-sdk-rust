pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollLinesAttendanceResponseDeductionsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub amount: String,
}

impl PostV1PayrollLinesAttendanceResponseDeductionsItem {
    pub fn builder() -> PostV1PayrollLinesAttendanceResponseDeductionsItemBuilder {
        <PostV1PayrollLinesAttendanceResponseDeductionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollLinesAttendanceResponseDeductionsItemBuilder {
    name: Option<String>,
    amount: Option<String>,
}

impl PostV1PayrollLinesAttendanceResponseDeductionsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollLinesAttendanceResponseDeductionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1PayrollLinesAttendanceResponseDeductionsItemBuilder::name)
    /// - [`amount`](PostV1PayrollLinesAttendanceResponseDeductionsItemBuilder::amount)
    pub fn build(self) -> Result<PostV1PayrollLinesAttendanceResponseDeductionsItem, BuildError> {
        Ok(PostV1PayrollLinesAttendanceResponseDeductionsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
