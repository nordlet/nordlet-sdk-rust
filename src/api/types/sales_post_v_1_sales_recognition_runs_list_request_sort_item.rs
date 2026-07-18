pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesRecognitionRunsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1SalesRecognitionRunsListRequestSortItemDir>,
}

impl PostV1SalesRecognitionRunsListRequestSortItem {
    pub fn builder() -> PostV1SalesRecognitionRunsListRequestSortItemBuilder {
        <PostV1SalesRecognitionRunsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesRecognitionRunsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1SalesRecognitionRunsListRequestSortItemDir>,
}

impl PostV1SalesRecognitionRunsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1SalesRecognitionRunsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesRecognitionRunsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1SalesRecognitionRunsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1SalesRecognitionRunsListRequestSortItem, BuildError> {
        Ok(PostV1SalesRecognitionRunsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
