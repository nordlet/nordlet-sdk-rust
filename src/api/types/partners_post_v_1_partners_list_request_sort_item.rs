pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1PartnersListRequestSortItemDir>,
}

impl PostV1PartnersListRequestSortItem {
    pub fn builder() -> PostV1PartnersListRequestSortItemBuilder {
        <PostV1PartnersListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1PartnersListRequestSortItemDir>,
}

impl PostV1PartnersListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1PartnersListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PartnersListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1PartnersListRequestSortItem, BuildError> {
        Ok(PostV1PartnersListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
