pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VehiclesUpdateFleetResponseCurrentAssignment {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(rename = "employeeName")]
    #[serde(default)]
    pub employee_name: String,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "privateUse")]
    #[serde(default)]
    pub private_use: bool,
    #[serde(rename = "employerPaysFuel")]
    #[serde(default)]
    pub employer_pays_fuel: bool,
}

impl VehiclesUpdateFleetResponseCurrentAssignment {
    pub fn builder() -> VehiclesUpdateFleetResponseCurrentAssignmentBuilder {
        <VehiclesUpdateFleetResponseCurrentAssignmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VehiclesUpdateFleetResponseCurrentAssignmentBuilder {
    id: Option<String>,
    employee_id: Option<String>,
    employee_name: Option<String>,
    from_date: Option<NaiveDate>,
    private_use: Option<bool>,
    employer_pays_fuel: Option<bool>,
}

impl VehiclesUpdateFleetResponseCurrentAssignmentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn private_use(mut self, value: bool) -> Self {
        self.private_use = Some(value);
        self
    }

    pub fn employer_pays_fuel(mut self, value: bool) -> Self {
        self.employer_pays_fuel = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VehiclesUpdateFleetResponseCurrentAssignment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](VehiclesUpdateFleetResponseCurrentAssignmentBuilder::id)
    /// - [`employee_id`](VehiclesUpdateFleetResponseCurrentAssignmentBuilder::employee_id)
    /// - [`employee_name`](VehiclesUpdateFleetResponseCurrentAssignmentBuilder::employee_name)
    /// - [`from_date`](VehiclesUpdateFleetResponseCurrentAssignmentBuilder::from_date)
    /// - [`private_use`](VehiclesUpdateFleetResponseCurrentAssignmentBuilder::private_use)
    /// - [`employer_pays_fuel`](VehiclesUpdateFleetResponseCurrentAssignmentBuilder::employer_pays_fuel)
    pub fn build(self) -> Result<VehiclesUpdateFleetResponseCurrentAssignment, BuildError> {
        Ok(VehiclesUpdateFleetResponseCurrentAssignment {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            employee_name: self
                .employee_name
                .ok_or_else(|| BuildError::missing_field("employee_name"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            private_use: self
                .private_use
                .ok_or_else(|| BuildError::missing_field("private_use"))?,
            employer_pays_fuel: self
                .employer_pays_fuel
                .ok_or_else(|| BuildError::missing_field("employer_pays_fuel"))?,
        })
    }
}
