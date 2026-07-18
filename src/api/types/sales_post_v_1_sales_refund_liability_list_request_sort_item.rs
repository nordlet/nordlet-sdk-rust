pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesRefundLiabilityListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1SalesRefundLiabilityListRequestSortItemDir>,
}

impl PostV1SalesRefundLiabilityListRequestSortItem {
    pub fn builder() -> PostV1SalesRefundLiabilityListRequestSortItemBuilder {
        <PostV1SalesRefundLiabilityListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesRefundLiabilityListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1SalesRefundLiabilityListRequestSortItemDir>,
}

impl PostV1SalesRefundLiabilityListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1SalesRefundLiabilityListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesRefundLiabilityListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1SalesRefundLiabilityListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1SalesRefundLiabilityListRequestSortItem, BuildError> {
        Ok(PostV1SalesRefundLiabilityListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
