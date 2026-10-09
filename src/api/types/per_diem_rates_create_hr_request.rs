pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PerDiemRatesCreateHrRequest {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "dailyAmount")]
    #[serde(default)]
    pub daily_amount: String,
    #[serde(rename = "validFrom")]
    #[serde(default)]
    pub valid_from: NaiveDate,
}

impl PerDiemRatesCreateHrRequest {
    pub fn builder() -> PerDiemRatesCreateHrRequestBuilder {
        <PerDiemRatesCreateHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PerDiemRatesCreateHrRequestBuilder {
    country_code: Option<String>,
    daily_amount: Option<String>,
    valid_from: Option<NaiveDate>,
}

impl PerDiemRatesCreateHrRequestBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn daily_amount(mut self, value: impl Into<String>) -> Self {
        self.daily_amount = Some(value.into());
        self
    }

    pub fn valid_from(mut self, value: NaiveDate) -> Self {
        self.valid_from = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PerDiemRatesCreateHrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](PerDiemRatesCreateHrRequestBuilder::country_code)
    /// - [`daily_amount`](PerDiemRatesCreateHrRequestBuilder::daily_amount)
    /// - [`valid_from`](PerDiemRatesCreateHrRequestBuilder::valid_from)
    pub fn build(self) -> Result<PerDiemRatesCreateHrRequest, BuildError> {
        Ok(PerDiemRatesCreateHrRequest {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            daily_amount: self
                .daily_amount
                .ok_or_else(|| BuildError::missing_field("daily_amount"))?,
            valid_from: self
                .valid_from
                .ok_or_else(|| BuildError::missing_field("valid_from"))?,
        })
    }
}
