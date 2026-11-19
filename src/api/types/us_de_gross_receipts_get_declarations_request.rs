pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UsDeGrossReceiptsGetDeclarationsRequest {
    #[serde(default)]
    pub period: String,
    pub activity: UsDeGrossReceiptsGetDeclarationsRequestActivity,
    #[serde(rename = "excludedReceipts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_receipts: Option<String>,
}

impl UsDeGrossReceiptsGetDeclarationsRequest {
    pub fn builder() -> UsDeGrossReceiptsGetDeclarationsRequestBuilder {
        <UsDeGrossReceiptsGetDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsDeGrossReceiptsGetDeclarationsRequestBuilder {
    period: Option<String>,
    activity: Option<UsDeGrossReceiptsGetDeclarationsRequestActivity>,
    excluded_receipts: Option<String>,
}

impl UsDeGrossReceiptsGetDeclarationsRequestBuilder {
    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
        self
    }

    pub fn activity(mut self, value: UsDeGrossReceiptsGetDeclarationsRequestActivity) -> Self {
        self.activity = Some(value);
        self
    }

    pub fn excluded_receipts(mut self, value: impl Into<String>) -> Self {
        self.excluded_receipts = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsDeGrossReceiptsGetDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`period`](UsDeGrossReceiptsGetDeclarationsRequestBuilder::period)
    /// - [`activity`](UsDeGrossReceiptsGetDeclarationsRequestBuilder::activity)
    pub fn build(self) -> Result<UsDeGrossReceiptsGetDeclarationsRequest, BuildError> {
        Ok(UsDeGrossReceiptsGetDeclarationsRequest {
            period: self
                .period
                .ok_or_else(|| BuildError::missing_field("period"))?,
            activity: self
                .activity
                .ok_or_else(|| BuildError::missing_field("activity"))?,
            excluded_receipts: self.excluded_receipts,
        })
    }
}
