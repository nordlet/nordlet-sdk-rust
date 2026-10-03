pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlKsefReceivedListRequest {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub from: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub to: DateTime<FixedOffset>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(rename = "pageOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_offset: Option<i64>,
}

impl PostV1DeclarationsPlKsefReceivedListRequest {
    pub fn builder() -> PostV1DeclarationsPlKsefReceivedListRequestBuilder {
        <PostV1DeclarationsPlKsefReceivedListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlKsefReceivedListRequestBuilder {
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
    page_size: Option<i64>,
    page_offset: Option<i64>,
}

impl PostV1DeclarationsPlKsefReceivedListRequestBuilder {
    pub fn from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.to = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page_offset(mut self, value: i64) -> Self {
        self.page_offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlKsefReceivedListRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from`](PostV1DeclarationsPlKsefReceivedListRequestBuilder::from)
    /// - [`to`](PostV1DeclarationsPlKsefReceivedListRequestBuilder::to)
    pub fn build(self) -> Result<PostV1DeclarationsPlKsefReceivedListRequest, BuildError> {
        Ok(PostV1DeclarationsPlKsefReceivedListRequest {
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
            page_size: self.page_size,
            page_offset: self.page_offset,
        })
    }
}
