pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListLeadsRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<ListLeadsRequestSortItemDir>,
}

impl ListLeadsRequestSortItem {
    pub fn builder() -> ListLeadsRequestSortItemBuilder {
        <ListLeadsRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListLeadsRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<ListLeadsRequestSortItemDir>,
}

impl ListLeadsRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: ListLeadsRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListLeadsRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ListLeadsRequestSortItemBuilder::field)
    pub fn build(self) -> Result<ListLeadsRequestSortItem, BuildError> {
        Ok(ListLeadsRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
