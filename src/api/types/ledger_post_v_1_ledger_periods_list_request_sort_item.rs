pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerPeriodsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1LedgerPeriodsListRequestSortItemDir>,
}

impl PostV1LedgerPeriodsListRequestSortItem {
    pub fn builder() -> PostV1LedgerPeriodsListRequestSortItemBuilder {
        <PostV1LedgerPeriodsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerPeriodsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1LedgerPeriodsListRequestSortItemDir>,
}

impl PostV1LedgerPeriodsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1LedgerPeriodsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerPeriodsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1LedgerPeriodsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1LedgerPeriodsListRequestSortItem, BuildError> {
        Ok(PostV1LedgerPeriodsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
