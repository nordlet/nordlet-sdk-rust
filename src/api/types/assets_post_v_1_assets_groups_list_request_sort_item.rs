pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AssetsGroupsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1AssetsGroupsListRequestSortItemDir>,
}

impl PostV1AssetsGroupsListRequestSortItem {
    pub fn builder() -> PostV1AssetsGroupsListRequestSortItemBuilder {
        <PostV1AssetsGroupsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsGroupsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1AssetsGroupsListRequestSortItemDir>,
}

impl PostV1AssetsGroupsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1AssetsGroupsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsGroupsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1AssetsGroupsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1AssetsGroupsListRequestSortItem, BuildError> {
        Ok(PostV1AssetsGroupsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
