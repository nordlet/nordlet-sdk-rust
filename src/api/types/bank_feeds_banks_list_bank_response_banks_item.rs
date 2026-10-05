pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FeedsBanksListBankResponseBanksItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub country: String,
    #[serde(rename = "logoUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(rename = "psuTypes")]
    #[serde(default)]
    pub psu_types: Vec<FeedsBanksListBankResponseBanksItemPsuTypesItem>,
    #[serde(rename = "maxConsentDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_consent_days: Option<i64>,
}

impl FeedsBanksListBankResponseBanksItem {
    pub fn builder() -> FeedsBanksListBankResponseBanksItemBuilder {
        <FeedsBanksListBankResponseBanksItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedsBanksListBankResponseBanksItemBuilder {
    name: Option<String>,
    country: Option<String>,
    logo_url: Option<String>,
    psu_types: Option<Vec<FeedsBanksListBankResponseBanksItemPsuTypesItem>>,
    max_consent_days: Option<i64>,
}

impl FeedsBanksListBankResponseBanksItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
        self
    }

    pub fn psu_types(
        mut self,
        value: Vec<FeedsBanksListBankResponseBanksItemPsuTypesItem>,
    ) -> Self {
        self.psu_types = Some(value);
        self
    }

    pub fn max_consent_days(mut self, value: i64) -> Self {
        self.max_consent_days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeedsBanksListBankResponseBanksItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](FeedsBanksListBankResponseBanksItemBuilder::name)
    /// - [`country`](FeedsBanksListBankResponseBanksItemBuilder::country)
    /// - [`psu_types`](FeedsBanksListBankResponseBanksItemBuilder::psu_types)
    pub fn build(self) -> Result<FeedsBanksListBankResponseBanksItem, BuildError> {
        Ok(FeedsBanksListBankResponseBanksItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            logo_url: self.logo_url,
            psu_types: self
                .psu_types
                .ok_or_else(|| BuildError::missing_field("psu_types"))?,
            max_consent_days: self.max_consent_days,
        })
    }
}
