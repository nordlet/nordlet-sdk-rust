pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtSdGenerateRequest {
    pub r#type: PostV1DeclarationsLtSdGenerateRequestType,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
}

impl PostV1DeclarationsLtSdGenerateRequest {
    pub fn builder() -> PostV1DeclarationsLtSdGenerateRequestBuilder {
        <PostV1DeclarationsLtSdGenerateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtSdGenerateRequestBuilder {
    r#type: Option<PostV1DeclarationsLtSdGenerateRequestType>,
    from_date: Option<String>,
    to_date: Option<String>,
}

impl PostV1DeclarationsLtSdGenerateRequestBuilder {
    pub fn r#type(mut self, value: PostV1DeclarationsLtSdGenerateRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtSdGenerateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](PostV1DeclarationsLtSdGenerateRequestBuilder::r#type)
    /// - [`from_date`](PostV1DeclarationsLtSdGenerateRequestBuilder::from_date)
    /// - [`to_date`](PostV1DeclarationsLtSdGenerateRequestBuilder::to_date)
    pub fn build(self) -> Result<PostV1DeclarationsLtSdGenerateRequest, BuildError> {
        Ok(PostV1DeclarationsLtSdGenerateRequest {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
        })
    }
}
