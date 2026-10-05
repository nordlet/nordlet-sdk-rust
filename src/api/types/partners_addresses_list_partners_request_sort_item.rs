pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddressesListPartnersRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<AddressesListPartnersRequestSortItemDir>,
}

impl AddressesListPartnersRequestSortItem {
    pub fn builder() -> AddressesListPartnersRequestSortItemBuilder {
        <AddressesListPartnersRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddressesListPartnersRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<AddressesListPartnersRequestSortItemDir>,
}

impl AddressesListPartnersRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: AddressesListPartnersRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddressesListPartnersRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](AddressesListPartnersRequestSortItemBuilder::field)
    pub fn build(self) -> Result<AddressesListPartnersRequestSortItem, BuildError> {
        Ok(AddressesListPartnersRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
