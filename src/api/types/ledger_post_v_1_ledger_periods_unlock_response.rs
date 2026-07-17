pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1LedgerPeriodsUnlockResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    pub status: PostV1LedgerPeriodsUnlockResponseStatus,
}

impl PostV1LedgerPeriodsUnlockResponse {
    pub fn builder() -> PostV1LedgerPeriodsUnlockResponseBuilder {
        <PostV1LedgerPeriodsUnlockResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerPeriodsUnlockResponseBuilder {
    id: Option<String>,
    year: Option<i64>,
    month: Option<i64>,
    status: Option<PostV1LedgerPeriodsUnlockResponseStatus>,
}

impl PostV1LedgerPeriodsUnlockResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn status(mut self, value: PostV1LedgerPeriodsUnlockResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerPeriodsUnlockResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LedgerPeriodsUnlockResponseBuilder::id)
    /// - [`year`](PostV1LedgerPeriodsUnlockResponseBuilder::year)
    /// - [`month`](PostV1LedgerPeriodsUnlockResponseBuilder::month)
    /// - [`status`](PostV1LedgerPeriodsUnlockResponseBuilder::status)
    pub fn build(self) -> Result<PostV1LedgerPeriodsUnlockResponse, BuildError> {
        Ok(PostV1LedgerPeriodsUnlockResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
