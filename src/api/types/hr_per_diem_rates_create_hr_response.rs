pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PerDiemRatesCreateHrResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "dailyAmount")]
    #[serde(default)]
    pub daily_amount: String,
    #[serde(rename = "validFrom")]
    #[serde(default)]
    pub valid_from: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl PerDiemRatesCreateHrResponse {
    pub fn builder() -> PerDiemRatesCreateHrResponseBuilder {
        <PerDiemRatesCreateHrResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PerDiemRatesCreateHrResponseBuilder {
    id: Option<String>,
    country_code: Option<String>,
    daily_amount: Option<String>,
    valid_from: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl PerDiemRatesCreateHrResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn daily_amount(mut self, value: impl Into<String>) -> Self {
        self.daily_amount = Some(value.into());
        self
    }

    pub fn valid_from(mut self, value: impl Into<String>) -> Self {
        self.valid_from = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PerDiemRatesCreateHrResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PerDiemRatesCreateHrResponseBuilder::id)
    /// - [`country_code`](PerDiemRatesCreateHrResponseBuilder::country_code)
    /// - [`daily_amount`](PerDiemRatesCreateHrResponseBuilder::daily_amount)
    /// - [`valid_from`](PerDiemRatesCreateHrResponseBuilder::valid_from)
    /// - [`created_at`](PerDiemRatesCreateHrResponseBuilder::created_at)
    pub fn build(self) -> Result<PerDiemRatesCreateHrResponse, BuildError> {
        Ok(PerDiemRatesCreateHrResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            daily_amount: self
                .daily_amount
                .ok_or_else(|| BuildError::missing_field("daily_amount"))?,
            valid_from: self
                .valid_from
                .ok_or_else(|| BuildError::missing_field("valid_from"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
