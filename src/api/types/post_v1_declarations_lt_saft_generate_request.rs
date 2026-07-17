pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtSaftGenerateRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(rename = "dataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<PostV1DeclarationsLtSaftGenerateRequestDataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persist: Option<bool>,
}

impl PostV1DeclarationsLtSaftGenerateRequest {
    pub fn builder() -> PostV1DeclarationsLtSaftGenerateRequestBuilder {
        <PostV1DeclarationsLtSaftGenerateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtSaftGenerateRequestBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
    data_type: Option<PostV1DeclarationsLtSaftGenerateRequestDataType>,
    persist: Option<bool>,
}

impl PostV1DeclarationsLtSaftGenerateRequestBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn data_type(mut self, value: PostV1DeclarationsLtSaftGenerateRequestDataType) -> Self {
        self.data_type = Some(value);
        self
    }

    pub fn persist(mut self, value: bool) -> Self {
        self.persist = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtSaftGenerateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1DeclarationsLtSaftGenerateRequestBuilder::from_date)
    /// - [`to_date`](PostV1DeclarationsLtSaftGenerateRequestBuilder::to_date)
    pub fn build(self) -> Result<PostV1DeclarationsLtSaftGenerateRequest, BuildError> {
        Ok(PostV1DeclarationsLtSaftGenerateRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            data_type: self.data_type,
            persist: self.persist,
        })
    }
}
