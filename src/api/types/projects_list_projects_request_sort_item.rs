pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListProjectsRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<ListProjectsRequestSortItemDir>,
}

impl ListProjectsRequestSortItem {
    pub fn builder() -> ListProjectsRequestSortItemBuilder {
        <ListProjectsRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListProjectsRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<ListProjectsRequestSortItemDir>,
}

impl ListProjectsRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: ListProjectsRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListProjectsRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ListProjectsRequestSortItemBuilder::field)
    pub fn build(self) -> Result<ListProjectsRequestSortItem, BuildError> {
        Ok(ListProjectsRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
