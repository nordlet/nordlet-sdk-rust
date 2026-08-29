pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceEuVatRatesSyncResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "situationOn")]
    #[serde(default)]
    pub situation_on: String,
    pub status: PostV1ReferenceEuVatRatesSyncResponseStatus,
    #[serde(rename = "ratesFetched")]
    #[serde(default)]
    pub rates_fetched: i64,
    #[serde(rename = "ratesInserted")]
    #[serde(default)]
    pub rates_inserted: i64,
    #[serde(rename = "ratesClosed")]
    #[serde(default)]
    pub rates_closed: i64,
}

impl PostV1ReferenceEuVatRatesSyncResponse {
    pub fn builder() -> PostV1ReferenceEuVatRatesSyncResponseBuilder {
        <PostV1ReferenceEuVatRatesSyncResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceEuVatRatesSyncResponseBuilder {
    id: Option<String>,
    situation_on: Option<String>,
    status: Option<PostV1ReferenceEuVatRatesSyncResponseStatus>,
    rates_fetched: Option<i64>,
    rates_inserted: Option<i64>,
    rates_closed: Option<i64>,
}

impl PostV1ReferenceEuVatRatesSyncResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn situation_on(mut self, value: impl Into<String>) -> Self {
        self.situation_on = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostV1ReferenceEuVatRatesSyncResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn rates_fetched(mut self, value: i64) -> Self {
        self.rates_fetched = Some(value);
        self
    }

    pub fn rates_inserted(mut self, value: i64) -> Self {
        self.rates_inserted = Some(value);
        self
    }

    pub fn rates_closed(mut self, value: i64) -> Self {
        self.rates_closed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceEuVatRatesSyncResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ReferenceEuVatRatesSyncResponseBuilder::id)
    /// - [`situation_on`](PostV1ReferenceEuVatRatesSyncResponseBuilder::situation_on)
    /// - [`status`](PostV1ReferenceEuVatRatesSyncResponseBuilder::status)
    /// - [`rates_fetched`](PostV1ReferenceEuVatRatesSyncResponseBuilder::rates_fetched)
    /// - [`rates_inserted`](PostV1ReferenceEuVatRatesSyncResponseBuilder::rates_inserted)
    /// - [`rates_closed`](PostV1ReferenceEuVatRatesSyncResponseBuilder::rates_closed)
    pub fn build(self) -> Result<PostV1ReferenceEuVatRatesSyncResponse, BuildError> {
        Ok(PostV1ReferenceEuVatRatesSyncResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            situation_on: self
                .situation_on
                .ok_or_else(|| BuildError::missing_field("situation_on"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            rates_fetched: self
                .rates_fetched
                .ok_or_else(|| BuildError::missing_field("rates_fetched"))?,
            rates_inserted: self
                .rates_inserted
                .ok_or_else(|| BuildError::missing_field("rates_inserted"))?,
            rates_closed: self
                .rates_closed
                .ok_or_else(|| BuildError::missing_field("rates_closed"))?,
        })
    }
}
