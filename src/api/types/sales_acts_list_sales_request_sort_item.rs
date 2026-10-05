pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ActsListSalesRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<ActsListSalesRequestSortItemDir>,
}

impl ActsListSalesRequestSortItem {
    pub fn builder() -> ActsListSalesRequestSortItemBuilder {
        <ActsListSalesRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActsListSalesRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<ActsListSalesRequestSortItemDir>,
}

impl ActsListSalesRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: ActsListSalesRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ActsListSalesRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ActsListSalesRequestSortItemBuilder::field)
    pub fn build(self) -> Result<ActsListSalesRequestSortItem, BuildError> {
        Ok(ActsListSalesRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
