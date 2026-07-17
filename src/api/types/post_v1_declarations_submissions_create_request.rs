pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsSubmissionsCreateRequest {
    pub obligation: PostV1DeclarationsSubmissionsCreateRequestObligation,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(rename = "dataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<PostV1DeclarationsSubmissionsCreateRequestDataType>,
}

impl PostV1DeclarationsSubmissionsCreateRequest {
    pub fn builder() -> PostV1DeclarationsSubmissionsCreateRequestBuilder {
        <PostV1DeclarationsSubmissionsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsSubmissionsCreateRequestBuilder {
    obligation: Option<PostV1DeclarationsSubmissionsCreateRequestObligation>,
    year: Option<i64>,
    month: Option<i64>,
    data_type: Option<PostV1DeclarationsSubmissionsCreateRequestDataType>,
}

impl PostV1DeclarationsSubmissionsCreateRequestBuilder {
    pub fn obligation(
        mut self,
        value: PostV1DeclarationsSubmissionsCreateRequestObligation,
    ) -> Self {
        self.obligation = Some(value);
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn data_type(mut self, value: PostV1DeclarationsSubmissionsCreateRequestDataType) -> Self {
        self.data_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsSubmissionsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`obligation`](PostV1DeclarationsSubmissionsCreateRequestBuilder::obligation)
    /// - [`year`](PostV1DeclarationsSubmissionsCreateRequestBuilder::year)
    /// - [`month`](PostV1DeclarationsSubmissionsCreateRequestBuilder::month)
    pub fn build(self) -> Result<PostV1DeclarationsSubmissionsCreateRequest, BuildError> {
        Ok(PostV1DeclarationsSubmissionsCreateRequest {
            obligation: self
                .obligation
                .ok_or_else(|| BuildError::missing_field("obligation"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            data_type: self.data_type,
        })
    }
}
