pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListOperationTypesRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<ListOperationTypesRequestSortItemDir>,
}

impl ListOperationTypesRequestSortItem {
    pub fn builder() -> ListOperationTypesRequestSortItemBuilder {
        <ListOperationTypesRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOperationTypesRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<ListOperationTypesRequestSortItemDir>,
}

impl ListOperationTypesRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: ListOperationTypesRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOperationTypesRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ListOperationTypesRequestSortItemBuilder::field)
    pub fn build(self) -> Result<ListOperationTypesRequestSortItem, BuildError> {
        Ok(ListOperationTypesRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
