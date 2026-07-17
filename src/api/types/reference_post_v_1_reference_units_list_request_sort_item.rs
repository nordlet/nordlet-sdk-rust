pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceUnitsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1ReferenceUnitsListRequestSortItemDir>,
}

impl PostV1ReferenceUnitsListRequestSortItem {
    pub fn builder() -> PostV1ReferenceUnitsListRequestSortItemBuilder {
        <PostV1ReferenceUnitsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceUnitsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1ReferenceUnitsListRequestSortItemDir>,
}

impl PostV1ReferenceUnitsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1ReferenceUnitsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceUnitsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ReferenceUnitsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1ReferenceUnitsListRequestSortItem, BuildError> {
        Ok(PostV1ReferenceUnitsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
