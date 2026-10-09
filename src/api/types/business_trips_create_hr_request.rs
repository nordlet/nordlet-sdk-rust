pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BusinessTripsCreateHrRequest {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(rename = "destinationCountryCode")]
    #[serde(default)]
    pub destination_country_code: String,
    #[serde(default)]
    pub purpose: String,
    #[serde(rename = "startDate")]
    #[serde(default)]
    pub start_date: NaiveDate,
    #[serde(rename = "endDate")]
    #[serde(default)]
    pub end_date: NaiveDate,
}

impl BusinessTripsCreateHrRequest {
    pub fn builder() -> BusinessTripsCreateHrRequestBuilder {
        <BusinessTripsCreateHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessTripsCreateHrRequestBuilder {
    employee_id: Option<String>,
    destination_country_code: Option<String>,
    purpose: Option<String>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
}

impl BusinessTripsCreateHrRequestBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn destination_country_code(mut self, value: impl Into<String>) -> Self {
        self.destination_country_code = Some(value.into());
        self
    }

    pub fn purpose(mut self, value: impl Into<String>) -> Self {
        self.purpose = Some(value.into());
        self
    }

    pub fn start_date(mut self, value: NaiveDate) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn end_date(mut self, value: NaiveDate) -> Self {
        self.end_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BusinessTripsCreateHrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](BusinessTripsCreateHrRequestBuilder::employee_id)
    /// - [`destination_country_code`](BusinessTripsCreateHrRequestBuilder::destination_country_code)
    /// - [`purpose`](BusinessTripsCreateHrRequestBuilder::purpose)
    /// - [`start_date`](BusinessTripsCreateHrRequestBuilder::start_date)
    /// - [`end_date`](BusinessTripsCreateHrRequestBuilder::end_date)
    pub fn build(self) -> Result<BusinessTripsCreateHrRequest, BuildError> {
        Ok(BusinessTripsCreateHrRequest {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            destination_country_code: self
                .destination_country_code
                .ok_or_else(|| BuildError::missing_field("destination_country_code"))?,
            purpose: self
                .purpose
                .ok_or_else(|| BuildError::missing_field("purpose"))?,
            start_date: self
                .start_date
                .ok_or_else(|| BuildError::missing_field("start_date"))?,
            end_date: self
                .end_date
                .ok_or_else(|| BuildError::missing_field("end_date"))?,
        })
    }
}
