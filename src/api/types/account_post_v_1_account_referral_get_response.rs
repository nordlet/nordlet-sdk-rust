pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountReferralGetResponse {
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
    pub history: Vec<PostV1AccountReferralGetResponseHistoryItem>,
}

impl PostV1AccountReferralGetResponse {
    pub fn builder() -> PostV1AccountReferralGetResponseBuilder {
        <PostV1AccountReferralGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountReferralGetResponseBuilder {
    code: Option<String>,
    link: Option<String>,
    points: Option<i64>,
    referred_count: Option<i64>,
    history: Option<Vec<PostV1AccountReferralGetResponseHistoryItem>>,
}

impl PostV1AccountReferralGetResponseBuilder {
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

    pub fn history(mut self, value: Vec<PostV1AccountReferralGetResponseHistoryItem>) -> Self {
        self.history = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountReferralGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1AccountReferralGetResponseBuilder::code)
    /// - [`link`](PostV1AccountReferralGetResponseBuilder::link)
    /// - [`points`](PostV1AccountReferralGetResponseBuilder::points)
    /// - [`referred_count`](PostV1AccountReferralGetResponseBuilder::referred_count)
    /// - [`history`](PostV1AccountReferralGetResponseBuilder::history)
    pub fn build(self) -> Result<PostV1AccountReferralGetResponse, BuildError> {
        Ok(PostV1AccountReferralGetResponse {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            link: self.link.ok_or_else(|| BuildError::missing_field("link"))?,
            points: self
                .points
                .ok_or_else(|| BuildError::missing_field("points"))?,
            referred_count: self
                .referred_count
                .ok_or_else(|| BuildError::missing_field("referred_count"))?,
            history: self
                .history
                .ok_or_else(|| BuildError::missing_field("history"))?,
        })
    }
}
