pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrContractsEndRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "endDate")]
    #[serde(default)]
    pub end_date: String,
    #[serde(rename = "endReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_reason: Option<String>,
}

impl PostV1HrContractsEndRequest {
    pub fn builder() -> PostV1HrContractsEndRequestBuilder {
        <PostV1HrContractsEndRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrContractsEndRequestBuilder {
    id: Option<String>,
    end_date: Option<String>,
    end_reason: Option<String>,
}

impl PostV1HrContractsEndRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    pub fn end_reason(mut self, value: impl Into<String>) -> Self {
        self.end_reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrContractsEndRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1HrContractsEndRequestBuilder::id)
    /// - [`end_date`](PostV1HrContractsEndRequestBuilder::end_date)
    pub fn build(self) -> Result<PostV1HrContractsEndRequest, BuildError> {
        Ok(PostV1HrContractsEndRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            end_date: self
                .end_date
                .ok_or_else(|| BuildError::missing_field("end_date"))?,
            end_reason: self.end_reason,
        })
    }
}
