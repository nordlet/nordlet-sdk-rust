pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobsListReportsRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: JobsListReportsRequestFilterItemOp,
    pub value: JobsListReportsRequestFilterItemValue,
}

impl JobsListReportsRequestFilterItem {
    pub fn builder() -> JobsListReportsRequestFilterItemBuilder {
        <JobsListReportsRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JobsListReportsRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<JobsListReportsRequestFilterItemOp>,
    value: Option<JobsListReportsRequestFilterItemValue>,
}

impl JobsListReportsRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: JobsListReportsRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: JobsListReportsRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JobsListReportsRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](JobsListReportsRequestFilterItemBuilder::field)
    /// - [`op`](JobsListReportsRequestFilterItemBuilder::op)
    /// - [`value`](JobsListReportsRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<JobsListReportsRequestFilterItem, BuildError> {
        Ok(JobsListReportsRequestFilterItem {
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
