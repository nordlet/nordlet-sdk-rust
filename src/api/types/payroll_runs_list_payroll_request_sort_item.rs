pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunsListPayrollRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<RunsListPayrollRequestSortItemDir>,
}

impl RunsListPayrollRequestSortItem {
    pub fn builder() -> RunsListPayrollRequestSortItemBuilder {
        <RunsListPayrollRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunsListPayrollRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<RunsListPayrollRequestSortItemDir>,
}

impl RunsListPayrollRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: RunsListPayrollRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RunsListPayrollRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](RunsListPayrollRequestSortItemBuilder::field)
    pub fn build(self) -> Result<RunsListPayrollRequestSortItem, BuildError> {
        Ok(RunsListPayrollRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
