pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeReturnFactsSetRequestFactsParticipationsItem {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "sharePercent")]
    #[serde(default)]
    pub share_percent: String,
    #[serde(default)]
    pub dividends: String,
}

impl PostV1DeclarationsDeReturnFactsSetRequestFactsParticipationsItem {
    pub fn builder() -> PostV1DeclarationsDeReturnFactsSetRequestFactsParticipationsItemBuilder {
        <PostV1DeclarationsDeReturnFactsSetRequestFactsParticipationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeReturnFactsSetRequestFactsParticipationsItemBuilder {
    name: Option<String>,
    country_code: Option<String>,
    share_percent: Option<String>,
    dividends: Option<String>,
}

impl PostV1DeclarationsDeReturnFactsSetRequestFactsParticipationsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn share_percent(mut self, value: impl Into<String>) -> Self {
        self.share_percent = Some(value.into());
        self
    }

    pub fn dividends(mut self, value: impl Into<String>) -> Self {
        self.dividends = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeReturnFactsSetRequestFactsParticipationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1DeclarationsDeReturnFactsSetRequestFactsParticipationsItemBuilder::name)
    /// - [`country_code`](PostV1DeclarationsDeReturnFactsSetRequestFactsParticipationsItemBuilder::country_code)
    /// - [`share_percent`](PostV1DeclarationsDeReturnFactsSetRequestFactsParticipationsItemBuilder::share_percent)
    /// - [`dividends`](PostV1DeclarationsDeReturnFactsSetRequestFactsParticipationsItemBuilder::dividends)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsDeReturnFactsSetRequestFactsParticipationsItem, BuildError> {
        Ok(
            PostV1DeclarationsDeReturnFactsSetRequestFactsParticipationsItem {
                name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
                country_code: self
                    .country_code
                    .ok_or_else(|| BuildError::missing_field("country_code"))?,
                share_percent: self
                    .share_percent
                    .ok_or_else(|| BuildError::missing_field("share_percent"))?,
                dividends: self
                    .dividends
                    .ok_or_else(|| BuildError::missing_field("dividends"))?,
            },
        )
    }
}
