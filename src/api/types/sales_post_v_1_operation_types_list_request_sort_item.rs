pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1OperationTypesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1OperationTypesListRequestSortItemDir>,
}

impl PostV1OperationTypesListRequestSortItem {
    pub fn builder() -> PostV1OperationTypesListRequestSortItemBuilder {
        <PostV1OperationTypesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1OperationTypesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1OperationTypesListRequestSortItemDir>,
}

impl PostV1OperationTypesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1OperationTypesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1OperationTypesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1OperationTypesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1OperationTypesListRequestSortItem, BuildError> {
        Ok(PostV1OperationTypesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
