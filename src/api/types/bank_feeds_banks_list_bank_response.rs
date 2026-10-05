pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FeedsBanksListBankResponse {
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub banks: Vec<FeedsBanksListBankResponseBanksItem>,
}

impl FeedsBanksListBankResponse {
    pub fn builder() -> FeedsBanksListBankResponseBuilder {
        <FeedsBanksListBankResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedsBanksListBankResponseBuilder {
    provider: Option<String>,
    banks: Option<Vec<FeedsBanksListBankResponseBanksItem>>,
}

impl FeedsBanksListBankResponseBuilder {
    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn banks(mut self, value: Vec<FeedsBanksListBankResponseBanksItem>) -> Self {
        self.banks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeedsBanksListBankResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`provider`](FeedsBanksListBankResponseBuilder::provider)
    /// - [`banks`](FeedsBanksListBankResponseBuilder::banks)
    pub fn build(self) -> Result<FeedsBanksListBankResponse, BuildError> {
        Ok(FeedsBanksListBankResponse {
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            banks: self
                .banks
                .ok_or_else(|| BuildError::missing_field("banks"))?,
        })
    }
}
