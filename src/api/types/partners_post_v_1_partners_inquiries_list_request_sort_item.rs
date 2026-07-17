pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersInquiriesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1PartnersInquiriesListRequestSortItemDir>,
}

impl PostV1PartnersInquiriesListRequestSortItem {
    pub fn builder() -> PostV1PartnersInquiriesListRequestSortItemBuilder {
        <PostV1PartnersInquiriesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersInquiriesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1PartnersInquiriesListRequestSortItemDir>,
}

impl PostV1PartnersInquiriesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1PartnersInquiriesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersInquiriesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PartnersInquiriesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1PartnersInquiriesListRequestSortItem, BuildError> {
        Ok(PostV1PartnersInquiriesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
