pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1HrEmployeesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1HrEmployeesListRequestFilterItemOp,
    pub value: PostV1HrEmployeesListRequestFilterItemValue,
}

impl PostV1HrEmployeesListRequestFilterItem {
    pub fn builder() -> PostV1HrEmployeesListRequestFilterItemBuilder {
        <PostV1HrEmployeesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1HrEmployeesListRequestFilterItemOp>,
    value: Option<PostV1HrEmployeesListRequestFilterItemValue>,
}

impl PostV1HrEmployeesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1HrEmployeesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1HrEmployeesListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1HrEmployeesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1HrEmployeesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1HrEmployeesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1HrEmployeesListRequestFilterItem, BuildError> {
        Ok(PostV1HrEmployeesListRequestFilterItem {
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
