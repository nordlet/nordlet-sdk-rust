pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersAddressesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1PartnersAddressesListRequestSortItemDir>,
}

impl PostV1PartnersAddressesListRequestSortItem {
    pub fn builder() -> PostV1PartnersAddressesListRequestSortItemBuilder {
        <PostV1PartnersAddressesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersAddressesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1PartnersAddressesListRequestSortItemDir>,
}

impl PostV1PartnersAddressesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1PartnersAddressesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersAddressesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PartnersAddressesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1PartnersAddressesListRequestSortItem, BuildError> {
        Ok(PostV1PartnersAddressesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
