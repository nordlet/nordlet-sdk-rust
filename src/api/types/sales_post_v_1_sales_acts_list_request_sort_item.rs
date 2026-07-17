pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesActsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1SalesActsListRequestSortItemDir>,
}

impl PostV1SalesActsListRequestSortItem {
    pub fn builder() -> PostV1SalesActsListRequestSortItemBuilder {
        <PostV1SalesActsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesActsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1SalesActsListRequestSortItemDir>,
}

impl PostV1SalesActsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1SalesActsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesActsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1SalesActsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1SalesActsListRequestSortItem, BuildError> {
        Ok(PostV1SalesActsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
