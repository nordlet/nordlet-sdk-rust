pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListFilesRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<ListFilesRequestSortItemDir>,
}

impl ListFilesRequestSortItem {
    pub fn builder() -> ListFilesRequestSortItemBuilder {
        <ListFilesRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFilesRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<ListFilesRequestSortItemDir>,
}

impl ListFilesRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: ListFilesRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFilesRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ListFilesRequestSortItemBuilder::field)
    pub fn build(self) -> Result<ListFilesRequestSortItem, BuildError> {
        Ok(ListFilesRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
