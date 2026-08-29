pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankFeedsSyncRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "feedAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed_account_id: Option<String>,
    #[serde(rename = "dateFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    #[serde(rename = "dateTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
}

impl PostV1BankFeedsSyncRequest {
    pub fn builder() -> PostV1BankFeedsSyncRequestBuilder {
        <PostV1BankFeedsSyncRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankFeedsSyncRequestBuilder {
    connection_id: Option<String>,
    feed_account_id: Option<String>,
    date_from: Option<String>,
    date_to: Option<String>,
}

impl PostV1BankFeedsSyncRequestBuilder {
    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    pub fn feed_account_id(mut self, value: impl Into<String>) -> Self {
        self.feed_account_id = Some(value.into());
        self
    }

    pub fn date_from(mut self, value: impl Into<String>) -> Self {
        self.date_from = Some(value.into());
        self
    }

    pub fn date_to(mut self, value: impl Into<String>) -> Self {
        self.date_to = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankFeedsSyncRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](PostV1BankFeedsSyncRequestBuilder::connection_id)
    pub fn build(self) -> Result<PostV1BankFeedsSyncRequest, BuildError> {
        Ok(PostV1BankFeedsSyncRequest {
            connection_id: self
                .connection_id
                .ok_or_else(|| BuildError::missing_field("connection_id"))?,
            feed_account_id: self.feed_account_id,
            date_from: self.date_from,
            date_to: self.date_to,
        })
    }
}
