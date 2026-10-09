pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PerDiemRatesListHrResponseRowsItem {
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

impl PerDiemRatesListHrResponseRowsItem {
    pub fn builder() -> PerDiemRatesListHrResponseRowsItemBuilder {
        <PerDiemRatesListHrResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PerDiemRatesListHrResponseRowsItemBuilder {
    id: Option<String>,
    country_code: Option<String>,
    daily_amount: Option<String>,
    valid_from: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl PerDiemRatesListHrResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PerDiemRatesListHrResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PerDiemRatesListHrResponseRowsItemBuilder::id)
    /// - [`country_code`](PerDiemRatesListHrResponseRowsItemBuilder::country_code)
    /// - [`daily_amount`](PerDiemRatesListHrResponseRowsItemBuilder::daily_amount)
    /// - [`valid_from`](PerDiemRatesListHrResponseRowsItemBuilder::valid_from)
    /// - [`created_at`](PerDiemRatesListHrResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PerDiemRatesListHrResponseRowsItem, BuildError> {
        Ok(PerDiemRatesListHrResponseRowsItem {
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
