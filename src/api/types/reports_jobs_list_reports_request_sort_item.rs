pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct JobsListReportsRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<JobsListReportsRequestSortItemDir>,
}

impl JobsListReportsRequestSortItem {
    pub fn builder() -> JobsListReportsRequestSortItemBuilder {
        <JobsListReportsRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JobsListReportsRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<JobsListReportsRequestSortItemDir>,
}

impl JobsListReportsRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: JobsListReportsRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JobsListReportsRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](JobsListReportsRequestSortItemBuilder::field)
    pub fn build(self) -> Result<JobsListReportsRequestSortItem, BuildError> {
        Ok(JobsListReportsRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
