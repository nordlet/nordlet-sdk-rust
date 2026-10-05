pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AssetsDisposeAssetsRequest {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub date: NaiveDate,
    pub reason: AssetsDisposeAssetsRequestReason,
    /// Sale price excluding VAT; 0 when scrapped or written off
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proceeds: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl AssetsDisposeAssetsRequest {
    pub fn builder() -> AssetsDisposeAssetsRequestBuilder {
        <AssetsDisposeAssetsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetsDisposeAssetsRequestBuilder {
    id: Option<String>,
    date: Option<NaiveDate>,
    reason: Option<AssetsDisposeAssetsRequestReason>,
    proceeds: Option<String>,
    notes: Option<String>,
}

impl AssetsDisposeAssetsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn reason(mut self, value: AssetsDisposeAssetsRequestReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn proceeds(mut self, value: impl Into<String>) -> Self {
        self.proceeds = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetsDisposeAssetsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AssetsDisposeAssetsRequestBuilder::id)
    /// - [`date`](AssetsDisposeAssetsRequestBuilder::date)
    /// - [`reason`](AssetsDisposeAssetsRequestBuilder::reason)
    pub fn build(self) -> Result<AssetsDisposeAssetsRequest, BuildError> {
        Ok(AssetsDisposeAssetsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            proceeds: self.proceeds,
            notes: self.notes,
        })
    }
}
