pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeEntriesListProjectsRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: TimeEntriesListProjectsRequestFilterItemOp,
    pub value: TimeEntriesListProjectsRequestFilterItemValue,
}

impl TimeEntriesListProjectsRequestFilterItem {
    pub fn builder() -> TimeEntriesListProjectsRequestFilterItemBuilder {
        <TimeEntriesListProjectsRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimeEntriesListProjectsRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<TimeEntriesListProjectsRequestFilterItemOp>,
    value: Option<TimeEntriesListProjectsRequestFilterItemValue>,
}

impl TimeEntriesListProjectsRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: TimeEntriesListProjectsRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: TimeEntriesListProjectsRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimeEntriesListProjectsRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](TimeEntriesListProjectsRequestFilterItemBuilder::field)
    /// - [`op`](TimeEntriesListProjectsRequestFilterItemBuilder::op)
    /// - [`value`](TimeEntriesListProjectsRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<TimeEntriesListProjectsRequestFilterItem, BuildError> {
        Ok(TimeEntriesListProjectsRequestFilterItem {
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
