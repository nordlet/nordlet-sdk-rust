pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPartnersRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<ListPartnersRequestSortItemDir>,
}

impl ListPartnersRequestSortItem {
    pub fn builder() -> ListPartnersRequestSortItemBuilder {
        <ListPartnersRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPartnersRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<ListPartnersRequestSortItemDir>,
}

impl ListPartnersRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: ListPartnersRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPartnersRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ListPartnersRequestSortItemBuilder::field)
    pub fn build(self) -> Result<ListPartnersRequestSortItem, BuildError> {
        Ok(ListPartnersRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
