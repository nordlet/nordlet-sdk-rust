pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionQualityChecksListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1ProductionQualityChecksListRequestSortItemDir>,
}

impl PostV1ProductionQualityChecksListRequestSortItem {
    pub fn builder() -> PostV1ProductionQualityChecksListRequestSortItemBuilder {
        <PostV1ProductionQualityChecksListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionQualityChecksListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1ProductionQualityChecksListRequestSortItemDir>,
}

impl PostV1ProductionQualityChecksListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1ProductionQualityChecksListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionQualityChecksListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ProductionQualityChecksListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1ProductionQualityChecksListRequestSortItem, BuildError> {
        Ok(PostV1ProductionQualityChecksListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
