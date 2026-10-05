pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAuditRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<ListAuditRequestSortItemDir>,
}

impl ListAuditRequestSortItem {
    pub fn builder() -> ListAuditRequestSortItemBuilder {
        <ListAuditRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAuditRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<ListAuditRequestSortItemDir>,
}

impl ListAuditRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: ListAuditRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAuditRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ListAuditRequestSortItemBuilder::field)
    pub fn build(self) -> Result<ListAuditRequestSortItem, BuildError> {
        Ok(ListAuditRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
