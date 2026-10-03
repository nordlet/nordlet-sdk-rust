pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollLinesAttendanceResponseAdditionsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub taxable: bool,
}

impl PostV1PayrollLinesAttendanceResponseAdditionsItem {
    pub fn builder() -> PostV1PayrollLinesAttendanceResponseAdditionsItemBuilder {
        <PostV1PayrollLinesAttendanceResponseAdditionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollLinesAttendanceResponseAdditionsItemBuilder {
    name: Option<String>,
    amount: Option<String>,
    taxable: Option<bool>,
}

impl PostV1PayrollLinesAttendanceResponseAdditionsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn taxable(mut self, value: bool) -> Self {
        self.taxable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollLinesAttendanceResponseAdditionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1PayrollLinesAttendanceResponseAdditionsItemBuilder::name)
    /// - [`amount`](PostV1PayrollLinesAttendanceResponseAdditionsItemBuilder::amount)
    /// - [`taxable`](PostV1PayrollLinesAttendanceResponseAdditionsItemBuilder::taxable)
    pub fn build(self) -> Result<PostV1PayrollLinesAttendanceResponseAdditionsItem, BuildError> {
        Ok(PostV1PayrollLinesAttendanceResponseAdditionsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            taxable: self
                .taxable
                .ok_or_else(|| BuildError::missing_field("taxable"))?,
        })
    }
}
