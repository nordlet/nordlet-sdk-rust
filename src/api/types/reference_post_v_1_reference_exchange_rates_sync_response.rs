pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceExchangeRatesSyncResponse {
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub imported: i64,
}

impl PostV1ReferenceExchangeRatesSyncResponse {
    pub fn builder() -> PostV1ReferenceExchangeRatesSyncResponseBuilder {
        <PostV1ReferenceExchangeRatesSyncResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceExchangeRatesSyncResponseBuilder {
    date: Option<String>,
    imported: Option<i64>,
}

impl PostV1ReferenceExchangeRatesSyncResponseBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn imported(mut self, value: i64) -> Self {
        self.imported = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceExchangeRatesSyncResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](PostV1ReferenceExchangeRatesSyncResponseBuilder::date)
    /// - [`imported`](PostV1ReferenceExchangeRatesSyncResponseBuilder::imported)
    pub fn build(self) -> Result<PostV1ReferenceExchangeRatesSyncResponse, BuildError> {
        Ok(PostV1ReferenceExchangeRatesSyncResponse {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            imported: self
                .imported
                .ok_or_else(|| BuildError::missing_field("imported"))?,
        })
    }
}
