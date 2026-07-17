pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1LedgerPeriodsLockResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    pub status: PostV1LedgerPeriodsLockResponseStatus,
}

impl PostV1LedgerPeriodsLockResponse {
    pub fn builder() -> PostV1LedgerPeriodsLockResponseBuilder {
        <PostV1LedgerPeriodsLockResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerPeriodsLockResponseBuilder {
    id: Option<String>,
    year: Option<i64>,
    month: Option<i64>,
    status: Option<PostV1LedgerPeriodsLockResponseStatus>,
}

impl PostV1LedgerPeriodsLockResponseBuilder {
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

    pub fn status(mut self, value: PostV1LedgerPeriodsLockResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerPeriodsLockResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LedgerPeriodsLockResponseBuilder::id)
    /// - [`year`](PostV1LedgerPeriodsLockResponseBuilder::year)
    /// - [`month`](PostV1LedgerPeriodsLockResponseBuilder::month)
    /// - [`status`](PostV1LedgerPeriodsLockResponseBuilder::status)
    pub fn build(self) -> Result<PostV1LedgerPeriodsLockResponse, BuildError> {
        Ok(PostV1LedgerPeriodsLockResponse {
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
