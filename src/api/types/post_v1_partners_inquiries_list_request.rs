pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1PartnersInquiriesListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<PostV1PartnersInquiriesListRequestSortItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<PostV1PartnersInquiriesListRequestFilterItem>>,
}

impl PostV1PartnersInquiriesListRequest {
    pub fn builder() -> PostV1PartnersInquiriesListRequestBuilder {
        <PostV1PartnersInquiriesListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersInquiriesListRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    sort: Option<Vec<PostV1PartnersInquiriesListRequestSortItem>>,
    filter: Option<Vec<PostV1PartnersInquiriesListRequestFilterItem>>,
}

impl PostV1PartnersInquiriesListRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn sort(mut self, value: Vec<PostV1PartnersInquiriesListRequestSortItem>) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn filter(mut self, value: Vec<PostV1PartnersInquiriesListRequestFilterItem>) -> Self {
        self.filter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersInquiriesListRequest`].
    pub fn build(self) -> Result<PostV1PartnersInquiriesListRequest, BuildError> {
        Ok(PostV1PartnersInquiriesListRequest {
            page: self.page,
            page_size: self.page_size,
            sort: self.sort,
            filter: self.filter,
        })
    }
}
