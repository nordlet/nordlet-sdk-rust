pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProjectsTimeEntriesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1ProjectsTimeEntriesListRequestSortItemDir>,
}

impl PostV1ProjectsTimeEntriesListRequestSortItem {
    pub fn builder() -> PostV1ProjectsTimeEntriesListRequestSortItemBuilder {
        <PostV1ProjectsTimeEntriesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProjectsTimeEntriesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1ProjectsTimeEntriesListRequestSortItemDir>,
}

impl PostV1ProjectsTimeEntriesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1ProjectsTimeEntriesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProjectsTimeEntriesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ProjectsTimeEntriesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1ProjectsTimeEntriesListRequestSortItem, BuildError> {
        Ok(PostV1ProjectsTimeEntriesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
