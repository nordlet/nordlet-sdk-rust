pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1FilesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1FilesListRequestSortItemDir>,
}

impl PostV1FilesListRequestSortItem {
    pub fn builder() -> PostV1FilesListRequestSortItemBuilder {
        <PostV1FilesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FilesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1FilesListRequestSortItemDir>,
}

impl PostV1FilesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1FilesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1FilesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1FilesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1FilesListRequestSortItem, BuildError> {
        Ok(PostV1FilesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
