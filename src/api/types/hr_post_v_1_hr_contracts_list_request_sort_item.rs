pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrContractsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1HrContractsListRequestSortItemDir>,
}

impl PostV1HrContractsListRequestSortItem {
    pub fn builder() -> PostV1HrContractsListRequestSortItemBuilder {
        <PostV1HrContractsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrContractsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1HrContractsListRequestSortItemDir>,
}

impl PostV1HrContractsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1HrContractsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrContractsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1HrContractsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1HrContractsListRequestSortItem, BuildError> {
        Ok(PostV1HrContractsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
