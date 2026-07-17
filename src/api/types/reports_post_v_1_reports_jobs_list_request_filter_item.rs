pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1ReportsJobsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1ReportsJobsListRequestFilterItemOp,
    pub value: PostV1ReportsJobsListRequestFilterItemValue,
}

impl PostV1ReportsJobsListRequestFilterItem {
    pub fn builder() -> PostV1ReportsJobsListRequestFilterItemBuilder {
        <PostV1ReportsJobsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsJobsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1ReportsJobsListRequestFilterItemOp>,
    value: Option<PostV1ReportsJobsListRequestFilterItemValue>,
}

impl PostV1ReportsJobsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1ReportsJobsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1ReportsJobsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsJobsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ReportsJobsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1ReportsJobsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1ReportsJobsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1ReportsJobsListRequestFilterItem, BuildError> {
        Ok(PostV1ReportsJobsListRequestFilterItem {
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
