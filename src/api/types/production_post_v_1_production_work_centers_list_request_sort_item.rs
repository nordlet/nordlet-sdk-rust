pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionWorkCentersListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1ProductionWorkCentersListRequestSortItemDir>,
}

impl PostV1ProductionWorkCentersListRequestSortItem {
    pub fn builder() -> PostV1ProductionWorkCentersListRequestSortItemBuilder {
        <PostV1ProductionWorkCentersListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionWorkCentersListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1ProductionWorkCentersListRequestSortItemDir>,
}

impl PostV1ProductionWorkCentersListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1ProductionWorkCentersListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionWorkCentersListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ProductionWorkCentersListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1ProductionWorkCentersListRequestSortItem, BuildError> {
        Ok(PostV1ProductionWorkCentersListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
