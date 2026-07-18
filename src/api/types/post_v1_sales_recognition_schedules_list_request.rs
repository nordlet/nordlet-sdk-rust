pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1SalesRecognitionSchedulesListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<PostV1SalesRecognitionSchedulesListRequestSortItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<PostV1SalesRecognitionSchedulesListRequestFilterItem>>,
}

impl PostV1SalesRecognitionSchedulesListRequest {
    pub fn builder() -> PostV1SalesRecognitionSchedulesListRequestBuilder {
        <PostV1SalesRecognitionSchedulesListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesRecognitionSchedulesListRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    sort: Option<Vec<PostV1SalesRecognitionSchedulesListRequestSortItem>>,
    filter: Option<Vec<PostV1SalesRecognitionSchedulesListRequestFilterItem>>,
}

impl PostV1SalesRecognitionSchedulesListRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn sort(mut self, value: Vec<PostV1SalesRecognitionSchedulesListRequestSortItem>) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn filter(
        mut self,
        value: Vec<PostV1SalesRecognitionSchedulesListRequestFilterItem>,
    ) -> Self {
        self.filter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesRecognitionSchedulesListRequest`].
    pub fn build(self) -> Result<PostV1SalesRecognitionSchedulesListRequest, BuildError> {
        Ok(PostV1SalesRecognitionSchedulesListRequest {
            page: self.page,
            page_size: self.page_size,
            sort: self.sort,
            filter: self.filter,
        })
    }
}
