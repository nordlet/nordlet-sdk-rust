pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ContactsListPartnersRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<ContactsListPartnersRequestSortItemDir>,
}

impl ContactsListPartnersRequestSortItem {
    pub fn builder() -> ContactsListPartnersRequestSortItemBuilder {
        <ContactsListPartnersRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContactsListPartnersRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<ContactsListPartnersRequestSortItemDir>,
}

impl ContactsListPartnersRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: ContactsListPartnersRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ContactsListPartnersRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ContactsListPartnersRequestSortItemBuilder::field)
    pub fn build(self) -> Result<ContactsListPartnersRequestSortItem, BuildError> {
        Ok(ContactsListPartnersRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
