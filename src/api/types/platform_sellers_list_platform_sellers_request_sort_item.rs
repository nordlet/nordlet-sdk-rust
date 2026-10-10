pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPlatformSellersRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<ListPlatformSellersRequestSortItemDir>,
}

impl ListPlatformSellersRequestSortItem {
    pub fn builder() -> ListPlatformSellersRequestSortItemBuilder {
        <ListPlatformSellersRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPlatformSellersRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<ListPlatformSellersRequestSortItemDir>,
}

impl ListPlatformSellersRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: ListPlatformSellersRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPlatformSellersRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ListPlatformSellersRequestSortItemBuilder::field)
    pub fn build(self) -> Result<ListPlatformSellersRequestSortItem, BuildError> {
        Ok(ListPlatformSellersRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
