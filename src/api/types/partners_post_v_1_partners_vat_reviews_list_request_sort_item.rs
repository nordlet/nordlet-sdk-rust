pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersVatReviewsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1PartnersVatReviewsListRequestSortItemDir>,
}

impl PostV1PartnersVatReviewsListRequestSortItem {
    pub fn builder() -> PostV1PartnersVatReviewsListRequestSortItemBuilder {
        <PostV1PartnersVatReviewsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersVatReviewsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1PartnersVatReviewsListRequestSortItemDir>,
}

impl PostV1PartnersVatReviewsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1PartnersVatReviewsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersVatReviewsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PartnersVatReviewsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1PartnersVatReviewsListRequestSortItem, BuildError> {
        Ok(PostV1PartnersVatReviewsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
