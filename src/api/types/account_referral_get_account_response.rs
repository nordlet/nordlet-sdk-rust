pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReferralGetAccountResponse {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub link: String,
    #[serde(default)]
    pub points: i64,
    #[serde(rename = "referredCount")]
    #[serde(default)]
    pub referred_count: i64,
    #[serde(default)]
    pub rates: ReferralGetAccountResponseRates,
    #[serde(default)]
    pub history: Vec<ReferralGetAccountResponseHistoryItem>,
}

impl ReferralGetAccountResponse {
    pub fn builder() -> ReferralGetAccountResponseBuilder {
        <ReferralGetAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReferralGetAccountResponseBuilder {
    code: Option<String>,
    link: Option<String>,
    points: Option<i64>,
    referred_count: Option<i64>,
    rates: Option<ReferralGetAccountResponseRates>,
    history: Option<Vec<ReferralGetAccountResponseHistoryItem>>,
}

impl ReferralGetAccountResponseBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn link(mut self, value: impl Into<String>) -> Self {
        self.link = Some(value.into());
        self
    }

    pub fn points(mut self, value: i64) -> Self {
        self.points = Some(value);
        self
    }

    pub fn referred_count(mut self, value: i64) -> Self {
        self.referred_count = Some(value);
        self
    }

    pub fn rates(mut self, value: ReferralGetAccountResponseRates) -> Self {
        self.rates = Some(value);
        self
    }

    pub fn history(mut self, value: Vec<ReferralGetAccountResponseHistoryItem>) -> Self {
        self.history = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReferralGetAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](ReferralGetAccountResponseBuilder::code)
    /// - [`link`](ReferralGetAccountResponseBuilder::link)
    /// - [`points`](ReferralGetAccountResponseBuilder::points)
    /// - [`referred_count`](ReferralGetAccountResponseBuilder::referred_count)
    /// - [`rates`](ReferralGetAccountResponseBuilder::rates)
    /// - [`history`](ReferralGetAccountResponseBuilder::history)
    pub fn build(self) -> Result<ReferralGetAccountResponse, BuildError> {
        Ok(ReferralGetAccountResponse {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            link: self.link.ok_or_else(|| BuildError::missing_field("link"))?,
            points: self
                .points
                .ok_or_else(|| BuildError::missing_field("points"))?,
            referred_count: self
                .referred_count
                .ok_or_else(|| BuildError::missing_field("referred_count"))?,
            rates: self
                .rates
                .ok_or_else(|| BuildError::missing_field("rates"))?,
            history: self
                .history
                .ok_or_else(|| BuildError::missing_field("history"))?,
        })
    }
}
