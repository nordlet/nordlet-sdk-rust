pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1SalesRecognitionRunsListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<PostV1SalesRecognitionRunsListRequestSortItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<PostV1SalesRecognitionRunsListRequestFilterItem>>,
}

impl PostV1SalesRecognitionRunsListRequest {
    pub fn builder() -> PostV1SalesRecognitionRunsListRequestBuilder {
        <PostV1SalesRecognitionRunsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesRecognitionRunsListRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    sort: Option<Vec<PostV1SalesRecognitionRunsListRequestSortItem>>,
    filter: Option<Vec<PostV1SalesRecognitionRunsListRequestFilterItem>>,
}

impl PostV1SalesRecognitionRunsListRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn sort(mut self, value: Vec<PostV1SalesRecognitionRunsListRequestSortItem>) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn filter(mut self, value: Vec<PostV1SalesRecognitionRunsListRequestFilterItem>) -> Self {
        self.filter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesRecognitionRunsListRequest`].
    pub fn build(self) -> Result<PostV1SalesRecognitionRunsListRequest, BuildError> {
        Ok(PostV1SalesRecognitionRunsListRequest {
            page: self.page,
            page_size: self.page_size,
            sort: self.sort,
            filter: self.filter,
        })
    }
}
