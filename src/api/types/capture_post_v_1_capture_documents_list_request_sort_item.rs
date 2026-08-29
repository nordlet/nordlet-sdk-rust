pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CaptureDocumentsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1CaptureDocumentsListRequestSortItemDir>,
}

impl PostV1CaptureDocumentsListRequestSortItem {
    pub fn builder() -> PostV1CaptureDocumentsListRequestSortItemBuilder {
        <PostV1CaptureDocumentsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CaptureDocumentsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1CaptureDocumentsListRequestSortItemDir>,
}

impl PostV1CaptureDocumentsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1CaptureDocumentsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CaptureDocumentsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1CaptureDocumentsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1CaptureDocumentsListRequestSortItem, BuildError> {
        Ok(PostV1CaptureDocumentsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
