pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GroupsListAssetsRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<GroupsListAssetsRequestSortItemDir>,
}

impl GroupsListAssetsRequestSortItem {
    pub fn builder() -> GroupsListAssetsRequestSortItemBuilder {
        <GroupsListAssetsRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupsListAssetsRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<GroupsListAssetsRequestSortItemDir>,
}

impl GroupsListAssetsRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: GroupsListAssetsRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GroupsListAssetsRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](GroupsListAssetsRequestSortItemBuilder::field)
    pub fn build(self) -> Result<GroupsListAssetsRequestSortItem, BuildError> {
        Ok(GroupsListAssetsRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
