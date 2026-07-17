pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsSubmissionsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1DeclarationsSubmissionsListRequestSortItemDir>,
}

impl PostV1DeclarationsSubmissionsListRequestSortItem {
    pub fn builder() -> PostV1DeclarationsSubmissionsListRequestSortItemBuilder {
        <PostV1DeclarationsSubmissionsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsSubmissionsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1DeclarationsSubmissionsListRequestSortItemDir>,
}

impl PostV1DeclarationsSubmissionsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1DeclarationsSubmissionsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsSubmissionsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1DeclarationsSubmissionsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1DeclarationsSubmissionsListRequestSortItem, BuildError> {
        Ok(PostV1DeclarationsSubmissionsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
