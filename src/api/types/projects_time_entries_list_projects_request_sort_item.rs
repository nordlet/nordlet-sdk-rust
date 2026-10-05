pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimeEntriesListProjectsRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<TimeEntriesListProjectsRequestSortItemDir>,
}

impl TimeEntriesListProjectsRequestSortItem {
    pub fn builder() -> TimeEntriesListProjectsRequestSortItemBuilder {
        <TimeEntriesListProjectsRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimeEntriesListProjectsRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<TimeEntriesListProjectsRequestSortItemDir>,
}

impl TimeEntriesListProjectsRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: TimeEntriesListProjectsRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimeEntriesListProjectsRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](TimeEntriesListProjectsRequestSortItemBuilder::field)
    pub fn build(self) -> Result<TimeEntriesListProjectsRequestSortItem, BuildError> {
        Ok(TimeEntriesListProjectsRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
