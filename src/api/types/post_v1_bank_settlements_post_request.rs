pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankSettlementsPostRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "commissionPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_percent: Option<String>,
}

impl PostV1BankSettlementsPostRequest {
    pub fn builder() -> PostV1BankSettlementsPostRequestBuilder {
        <PostV1BankSettlementsPostRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankSettlementsPostRequestBuilder {
    id: Option<String>,
    date: Option<String>,
    commission_percent: Option<String>,
}

impl PostV1BankSettlementsPostRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn commission_percent(mut self, value: impl Into<String>) -> Self {
        self.commission_percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankSettlementsPostRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankSettlementsPostRequestBuilder::id)
    pub fn build(self) -> Result<PostV1BankSettlementsPostRequest, BuildError> {
        Ok(PostV1BankSettlementsPostRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            date: self.date,
            commission_percent: self.commission_percent,
        })
    }
}
