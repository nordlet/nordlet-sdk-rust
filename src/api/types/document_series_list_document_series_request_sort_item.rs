pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDocumentSeriesRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<ListDocumentSeriesRequestSortItemDir>,
}

impl ListDocumentSeriesRequestSortItem {
    pub fn builder() -> ListDocumentSeriesRequestSortItemBuilder {
        <ListDocumentSeriesRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDocumentSeriesRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<ListDocumentSeriesRequestSortItemDir>,
}

impl ListDocumentSeriesRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: ListDocumentSeriesRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDocumentSeriesRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ListDocumentSeriesRequestSortItemBuilder::field)
    pub fn build(self) -> Result<ListDocumentSeriesRequestSortItem, BuildError> {
        Ok(ListDocumentSeriesRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
