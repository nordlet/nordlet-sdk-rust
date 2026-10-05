pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentsListCaptureRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<DocumentsListCaptureRequestSortItemDir>,
}

impl DocumentsListCaptureRequestSortItem {
    pub fn builder() -> DocumentsListCaptureRequestSortItemBuilder {
        <DocumentsListCaptureRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentsListCaptureRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<DocumentsListCaptureRequestSortItemDir>,
}

impl DocumentsListCaptureRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: DocumentsListCaptureRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentsListCaptureRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](DocumentsListCaptureRequestSortItemBuilder::field)
    pub fn build(self) -> Result<DocumentsListCaptureRequestSortItem, BuildError> {
        Ok(DocumentsListCaptureRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
