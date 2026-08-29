pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1FleetAssignmentsCreateRequest {
    #[serde(rename = "vehicleId")]
    #[serde(default)]
    pub vehicle_id: String,
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,
    #[serde(rename = "privateUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_use: Option<bool>,
    #[serde(rename = "employerPaysFuel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employer_pays_fuel: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1FleetAssignmentsCreateRequest {
    pub fn builder() -> PostV1FleetAssignmentsCreateRequestBuilder {
        <PostV1FleetAssignmentsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FleetAssignmentsCreateRequestBuilder {
    vehicle_id: Option<String>,
    employee_id: Option<String>,
    from_date: Option<String>,
    to_date: Option<String>,
    private_use: Option<bool>,
    employer_pays_fuel: Option<bool>,
    notes: Option<String>,
}

impl PostV1FleetAssignmentsCreateRequestBuilder {
    pub fn vehicle_id(mut self, value: impl Into<String>) -> Self {
        self.vehicle_id = Some(value.into());
        self
    }

    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn private_use(mut self, value: bool) -> Self {
        self.private_use = Some(value);
        self
    }

    pub fn employer_pays_fuel(mut self, value: bool) -> Self {
        self.employer_pays_fuel = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1FleetAssignmentsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`vehicle_id`](PostV1FleetAssignmentsCreateRequestBuilder::vehicle_id)
    /// - [`employee_id`](PostV1FleetAssignmentsCreateRequestBuilder::employee_id)
    /// - [`from_date`](PostV1FleetAssignmentsCreateRequestBuilder::from_date)
    pub fn build(self) -> Result<PostV1FleetAssignmentsCreateRequest, BuildError> {
        Ok(PostV1FleetAssignmentsCreateRequest {
            vehicle_id: self
                .vehicle_id
                .ok_or_else(|| BuildError::missing_field("vehicle_id"))?,
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self.to_date,
            private_use: self.private_use,
            employer_pays_fuel: self.employer_pays_fuel,
            notes: self.notes,
        })
    }
}
