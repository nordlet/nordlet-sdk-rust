pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1FleetAssignmentsCreateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "vehicleId")]
    #[serde(default)]
    pub vehicle_id: String,
    #[serde(rename = "plateNumber")]
    #[serde(default)]
    pub plate_number: String,
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(rename = "employeeName")]
    #[serde(default)]
    pub employee_name: String,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,
    #[serde(rename = "privateUse")]
    #[serde(default)]
    pub private_use: bool,
    #[serde(rename = "employerPaysFuel")]
    #[serde(default)]
    pub employer_pays_fuel: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1FleetAssignmentsCreateResponse {
    pub fn builder() -> PostV1FleetAssignmentsCreateResponseBuilder {
        <PostV1FleetAssignmentsCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FleetAssignmentsCreateResponseBuilder {
    id: Option<String>,
    vehicle_id: Option<String>,
    plate_number: Option<String>,
    employee_id: Option<String>,
    employee_name: Option<String>,
    from_date: Option<String>,
    to_date: Option<String>,
    private_use: Option<bool>,
    employer_pays_fuel: Option<bool>,
    notes: Option<String>,
    created_at: Option<String>,
}

impl PostV1FleetAssignmentsCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn vehicle_id(mut self, value: impl Into<String>) -> Self {
        self.vehicle_id = Some(value.into());
        self
    }

    pub fn plate_number(mut self, value: impl Into<String>) -> Self {
        self.plate_number = Some(value.into());
        self
    }

    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn employee_name(mut self, value: impl Into<String>) -> Self {
        self.employee_name = Some(value.into());
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1FleetAssignmentsCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1FleetAssignmentsCreateResponseBuilder::id)
    /// - [`vehicle_id`](PostV1FleetAssignmentsCreateResponseBuilder::vehicle_id)
    /// - [`plate_number`](PostV1FleetAssignmentsCreateResponseBuilder::plate_number)
    /// - [`employee_id`](PostV1FleetAssignmentsCreateResponseBuilder::employee_id)
    /// - [`employee_name`](PostV1FleetAssignmentsCreateResponseBuilder::employee_name)
    /// - [`from_date`](PostV1FleetAssignmentsCreateResponseBuilder::from_date)
    /// - [`private_use`](PostV1FleetAssignmentsCreateResponseBuilder::private_use)
    /// - [`employer_pays_fuel`](PostV1FleetAssignmentsCreateResponseBuilder::employer_pays_fuel)
    /// - [`created_at`](PostV1FleetAssignmentsCreateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1FleetAssignmentsCreateResponse, BuildError> {
        Ok(PostV1FleetAssignmentsCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            vehicle_id: self
                .vehicle_id
                .ok_or_else(|| BuildError::missing_field("vehicle_id"))?,
            plate_number: self
                .plate_number
                .ok_or_else(|| BuildError::missing_field("plate_number"))?,
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            employee_name: self
                .employee_name
                .ok_or_else(|| BuildError::missing_field("employee_name"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self.to_date,
            private_use: self
                .private_use
                .ok_or_else(|| BuildError::missing_field("private_use"))?,
            employer_pays_fuel: self
                .employer_pays_fuel
                .ok_or_else(|| BuildError::missing_field("employer_pays_fuel"))?,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
