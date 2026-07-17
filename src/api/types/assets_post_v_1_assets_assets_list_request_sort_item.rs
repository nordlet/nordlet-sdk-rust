pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AssetsAssetsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1AssetsAssetsListRequestSortItemDir>,
}

impl PostV1AssetsAssetsListRequestSortItem {
    pub fn builder() -> PostV1AssetsAssetsListRequestSortItemBuilder {
        <PostV1AssetsAssetsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AssetsAssetsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1AssetsAssetsListRequestSortItemDir>,
}

impl PostV1AssetsAssetsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1AssetsAssetsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AssetsAssetsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1AssetsAssetsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1AssetsAssetsListRequestSortItem, BuildError> {
        Ok(PostV1AssetsAssetsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
