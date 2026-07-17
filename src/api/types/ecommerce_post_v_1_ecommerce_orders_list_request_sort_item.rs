pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1EcommerceOrdersListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1EcommerceOrdersListRequestSortItemDir>,
}

impl PostV1EcommerceOrdersListRequestSortItem {
    pub fn builder() -> PostV1EcommerceOrdersListRequestSortItemBuilder {
        <PostV1EcommerceOrdersListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1EcommerceOrdersListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1EcommerceOrdersListRequestSortItemDir>,
}

impl PostV1EcommerceOrdersListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1EcommerceOrdersListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1EcommerceOrdersListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1EcommerceOrdersListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1EcommerceOrdersListRequestSortItem, BuildError> {
        Ok(PostV1EcommerceOrdersListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
