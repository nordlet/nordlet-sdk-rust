pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionBomsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1ProductionBomsListRequestSortItemDir>,
}

impl PostV1ProductionBomsListRequestSortItem {
    pub fn builder() -> PostV1ProductionBomsListRequestSortItemBuilder {
        <PostV1ProductionBomsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionBomsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1ProductionBomsListRequestSortItemDir>,
}

impl PostV1ProductionBomsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1ProductionBomsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionBomsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ProductionBomsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1ProductionBomsListRequestSortItem, BuildError> {
        Ok(PostV1ProductionBomsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
