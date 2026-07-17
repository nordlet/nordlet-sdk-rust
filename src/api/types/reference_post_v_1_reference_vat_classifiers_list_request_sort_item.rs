pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceVatClassifiersListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1ReferenceVatClassifiersListRequestSortItemDir>,
}

impl PostV1ReferenceVatClassifiersListRequestSortItem {
    pub fn builder() -> PostV1ReferenceVatClassifiersListRequestSortItemBuilder {
        <PostV1ReferenceVatClassifiersListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceVatClassifiersListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1ReferenceVatClassifiersListRequestSortItemDir>,
}

impl PostV1ReferenceVatClassifiersListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1ReferenceVatClassifiersListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceVatClassifiersListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ReferenceVatClassifiersListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1ReferenceVatClassifiersListRequestSortItem, BuildError> {
        Ok(PostV1ReferenceVatClassifiersListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
