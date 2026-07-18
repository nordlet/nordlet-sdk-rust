pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesRecognitionSchedulesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1SalesRecognitionSchedulesListRequestSortItemDir>,
}

impl PostV1SalesRecognitionSchedulesListRequestSortItem {
    pub fn builder() -> PostV1SalesRecognitionSchedulesListRequestSortItemBuilder {
        <PostV1SalesRecognitionSchedulesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesRecognitionSchedulesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1SalesRecognitionSchedulesListRequestSortItemDir>,
}

impl PostV1SalesRecognitionSchedulesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1SalesRecognitionSchedulesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesRecognitionSchedulesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1SalesRecognitionSchedulesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1SalesRecognitionSchedulesListRequestSortItem, BuildError> {
        Ok(PostV1SalesRecognitionSchedulesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
