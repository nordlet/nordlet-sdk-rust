pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssignmentsCreateFleetResponse {
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
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_date: Option<NaiveDate>,
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
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl AssignmentsCreateFleetResponse {
    pub fn builder() -> AssignmentsCreateFleetResponseBuilder {
        <AssignmentsCreateFleetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssignmentsCreateFleetResponseBuilder {
    id: Option<String>,
    vehicle_id: Option<String>,
    plate_number: Option<String>,
    employee_id: Option<String>,
    employee_name: Option<String>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    private_use: Option<bool>,
    employer_pays_fuel: Option<bool>,
    notes: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl AssignmentsCreateFleetResponseBuilder {
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

    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssignmentsCreateFleetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AssignmentsCreateFleetResponseBuilder::id)
    /// - [`vehicle_id`](AssignmentsCreateFleetResponseBuilder::vehicle_id)
    /// - [`plate_number`](AssignmentsCreateFleetResponseBuilder::plate_number)
    /// - [`employee_id`](AssignmentsCreateFleetResponseBuilder::employee_id)
    /// - [`employee_name`](AssignmentsCreateFleetResponseBuilder::employee_name)
    /// - [`from_date`](AssignmentsCreateFleetResponseBuilder::from_date)
    /// - [`private_use`](AssignmentsCreateFleetResponseBuilder::private_use)
    /// - [`employer_pays_fuel`](AssignmentsCreateFleetResponseBuilder::employer_pays_fuel)
    /// - [`created_at`](AssignmentsCreateFleetResponseBuilder::created_at)
    pub fn build(self) -> Result<AssignmentsCreateFleetResponse, BuildError> {
        Ok(AssignmentsCreateFleetResponse {
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
