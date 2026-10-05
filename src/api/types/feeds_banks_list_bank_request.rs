pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FeedsBanksListBankRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl FeedsBanksListBankRequest {
    pub fn builder() -> FeedsBanksListBankRequestBuilder {
        <FeedsBanksListBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedsBanksListBankRequestBuilder {
    country: Option<String>,
}

impl FeedsBanksListBankRequestBuilder {
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FeedsBanksListBankRequest`].
    pub fn build(self) -> Result<FeedsBanksListBankRequest, BuildError> {
        Ok(FeedsBanksListBankRequest {
            country: self.country,
        })
    }
}
