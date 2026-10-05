pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeReturnFactsSetDeclarationsResponseFactsParticipationsItem {
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

impl DeReturnFactsSetDeclarationsResponseFactsParticipationsItem {
    pub fn builder() -> DeReturnFactsSetDeclarationsResponseFactsParticipationsItemBuilder {
        <DeReturnFactsSetDeclarationsResponseFactsParticipationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeReturnFactsSetDeclarationsResponseFactsParticipationsItemBuilder {
    name: Option<String>,
    country_code: Option<String>,
    share_percent: Option<String>,
    dividends: Option<String>,
}

impl DeReturnFactsSetDeclarationsResponseFactsParticipationsItemBuilder {
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

    /// Consumes the builder and constructs a [`DeReturnFactsSetDeclarationsResponseFactsParticipationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DeReturnFactsSetDeclarationsResponseFactsParticipationsItemBuilder::name)
    /// - [`country_code`](DeReturnFactsSetDeclarationsResponseFactsParticipationsItemBuilder::country_code)
    /// - [`share_percent`](DeReturnFactsSetDeclarationsResponseFactsParticipationsItemBuilder::share_percent)
    /// - [`dividends`](DeReturnFactsSetDeclarationsResponseFactsParticipationsItemBuilder::dividends)
    pub fn build(
        self,
    ) -> Result<DeReturnFactsSetDeclarationsResponseFactsParticipationsItem, BuildError> {
        Ok(
            DeReturnFactsSetDeclarationsResponseFactsParticipationsItem {
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
