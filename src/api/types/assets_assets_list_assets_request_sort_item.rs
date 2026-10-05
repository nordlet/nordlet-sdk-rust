pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetsListAssetsRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<AssetsListAssetsRequestSortItemDir>,
}

impl AssetsListAssetsRequestSortItem {
    pub fn builder() -> AssetsListAssetsRequestSortItemBuilder {
        <AssetsListAssetsRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetsListAssetsRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<AssetsListAssetsRequestSortItemDir>,
}

impl AssetsListAssetsRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: AssetsListAssetsRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssetsListAssetsRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](AssetsListAssetsRequestSortItemBuilder::field)
    pub fn build(self) -> Result<AssetsListAssetsRequestSortItem, BuildError> {
        Ok(AssetsListAssetsRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
