pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompaniesHouseGetPartnersResponseAccountingReferenceDate {
    #[serde(default)]
    pub day: i64,
    #[serde(default)]
    pub month: i64,
}

impl CompaniesHouseGetPartnersResponseAccountingReferenceDate {
    pub fn builder() -> CompaniesHouseGetPartnersResponseAccountingReferenceDateBuilder {
        <CompaniesHouseGetPartnersResponseAccountingReferenceDateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesHouseGetPartnersResponseAccountingReferenceDateBuilder {
    day: Option<i64>,
    month: Option<i64>,
}

impl CompaniesHouseGetPartnersResponseAccountingReferenceDateBuilder {
    pub fn day(mut self, value: i64) -> Self {
        self.day = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompaniesHouseGetPartnersResponseAccountingReferenceDate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`day`](CompaniesHouseGetPartnersResponseAccountingReferenceDateBuilder::day)
    /// - [`month`](CompaniesHouseGetPartnersResponseAccountingReferenceDateBuilder::month)
    pub fn build(
        self,
    ) -> Result<CompaniesHouseGetPartnersResponseAccountingReferenceDate, BuildError> {
        Ok(CompaniesHouseGetPartnersResponseAccountingReferenceDate {
            day: self.day.ok_or_else(|| BuildError::missing_field("day"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
