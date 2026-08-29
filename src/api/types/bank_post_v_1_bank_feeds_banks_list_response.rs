pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankFeedsBanksListResponse {
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub banks: Vec<PostV1BankFeedsBanksListResponseBanksItem>,
}

impl PostV1BankFeedsBanksListResponse {
    pub fn builder() -> PostV1BankFeedsBanksListResponseBuilder {
        <PostV1BankFeedsBanksListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankFeedsBanksListResponseBuilder {
    provider: Option<String>,
    banks: Option<Vec<PostV1BankFeedsBanksListResponseBanksItem>>,
}

impl PostV1BankFeedsBanksListResponseBuilder {
    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn banks(mut self, value: Vec<PostV1BankFeedsBanksListResponseBanksItem>) -> Self {
        self.banks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankFeedsBanksListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](PostV1BankFeedsBanksListResponseBuilder::provider)
    /// - [`banks`](PostV1BankFeedsBanksListResponseBuilder::banks)
    pub fn build(self) -> Result<PostV1BankFeedsBanksListResponse, BuildError> {
        Ok(PostV1BankFeedsBanksListResponse {
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            banks: self
                .banks
                .ok_or_else(|| BuildError::missing_field("banks"))?,
        })
    }
}
