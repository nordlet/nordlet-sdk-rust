pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FeedsSyncBankRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "feedAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed_account_id: Option<String>,
    #[serde(rename = "dateFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<NaiveDate>,
    #[serde(rename = "dateTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<NaiveDate>,
}

impl FeedsSyncBankRequest {
    pub fn builder() -> FeedsSyncBankRequestBuilder {
        <FeedsSyncBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedsSyncBankRequestBuilder {
    connection_id: Option<String>,
    feed_account_id: Option<String>,
    date_from: Option<NaiveDate>,
    date_to: Option<NaiveDate>,
}

impl FeedsSyncBankRequestBuilder {
    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    pub fn feed_account_id(mut self, value: impl Into<String>) -> Self {
        self.feed_account_id = Some(value.into());
        self
    }

    pub fn date_from(mut self, value: NaiveDate) -> Self {
        self.date_from = Some(value);
        self
    }

    pub fn date_to(mut self, value: NaiveDate) -> Self {
        self.date_to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeedsSyncBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](FeedsSyncBankRequestBuilder::connection_id)
    pub fn build(self) -> Result<FeedsSyncBankRequest, BuildError> {
        Ok(FeedsSyncBankRequest {
            connection_id: self
                .connection_id
                .ok_or_else(|| BuildError::missing_field("connection_id"))?,
            feed_account_id: self.feed_account_id,
            date_from: self.date_from,
            date_to: self.date_to,
        })
    }
}
