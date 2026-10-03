pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtSaftSendRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(rename = "dataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<PostV1DeclarationsLtSaftSendRequestDataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
}

impl PostV1DeclarationsLtSaftSendRequest {
    pub fn builder() -> PostV1DeclarationsLtSaftSendRequestBuilder {
        <PostV1DeclarationsLtSaftSendRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtSaftSendRequestBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
    data_type: Option<PostV1DeclarationsLtSaftSendRequestDataType>,
    confirm: Option<bool>,
}

impl PostV1DeclarationsLtSaftSendRequestBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn data_type(mut self, value: PostV1DeclarationsLtSaftSendRequestDataType) -> Self {
        self.data_type = Some(value);
        self
    }

    pub fn confirm(mut self, value: bool) -> Self {
        self.confirm = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtSaftSendRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1DeclarationsLtSaftSendRequestBuilder::from_date)
    /// - [`to_date`](PostV1DeclarationsLtSaftSendRequestBuilder::to_date)
    pub fn build(self) -> Result<PostV1DeclarationsLtSaftSendRequest, BuildError> {
        Ok(PostV1DeclarationsLtSaftSendRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            data_type: self.data_type,
            confirm: self.confirm,
        })
    }
}
