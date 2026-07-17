pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersContactsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1PartnersContactsListRequestSortItemDir>,
}

impl PostV1PartnersContactsListRequestSortItem {
    pub fn builder() -> PostV1PartnersContactsListRequestSortItemBuilder {
        <PostV1PartnersContactsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersContactsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1PartnersContactsListRequestSortItemDir>,
}

impl PostV1PartnersContactsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1PartnersContactsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersContactsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PartnersContactsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1PartnersContactsListRequestSortItem, BuildError> {
        Ok(PostV1PartnersContactsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
