pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1HrEmployeesRecordsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1HrEmployeesRecordsListRequestFilterItemOp,
    pub value: PostV1HrEmployeesRecordsListRequestFilterItemValue,
}

impl PostV1HrEmployeesRecordsListRequestFilterItem {
    pub fn builder() -> PostV1HrEmployeesRecordsListRequestFilterItemBuilder {
        <PostV1HrEmployeesRecordsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesRecordsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1HrEmployeesRecordsListRequestFilterItemOp>,
    value: Option<PostV1HrEmployeesRecordsListRequestFilterItemValue>,
}

impl PostV1HrEmployeesRecordsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1HrEmployeesRecordsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1HrEmployeesRecordsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesRecordsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1HrEmployeesRecordsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1HrEmployeesRecordsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1HrEmployeesRecordsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1HrEmployeesRecordsListRequestFilterItem, BuildError> {
        Ok(PostV1HrEmployeesRecordsListRequestFilterItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            op: self.op.ok_or_else(|| BuildError::missing_field("op"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
