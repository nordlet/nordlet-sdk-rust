pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionRoutingsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1ProductionRoutingsListRequestSortItemDir>,
}

impl PostV1ProductionRoutingsListRequestSortItem {
    pub fn builder() -> PostV1ProductionRoutingsListRequestSortItemBuilder {
        <PostV1ProductionRoutingsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionRoutingsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1ProductionRoutingsListRequestSortItemDir>,
}

impl PostV1ProductionRoutingsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1ProductionRoutingsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionRoutingsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ProductionRoutingsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1ProductionRoutingsListRequestSortItem, BuildError> {
        Ok(PostV1ProductionRoutingsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
