pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssignmentsListFleetResponseRowsItem {
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

impl AssignmentsListFleetResponseRowsItem {
    pub fn builder() -> AssignmentsListFleetResponseRowsItemBuilder {
        <AssignmentsListFleetResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssignmentsListFleetResponseRowsItemBuilder {
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

impl AssignmentsListFleetResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`AssignmentsListFleetResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AssignmentsListFleetResponseRowsItemBuilder::id)
    /// - [`vehicle_id`](AssignmentsListFleetResponseRowsItemBuilder::vehicle_id)
    /// - [`plate_number`](AssignmentsListFleetResponseRowsItemBuilder::plate_number)
    /// - [`employee_id`](AssignmentsListFleetResponseRowsItemBuilder::employee_id)
    /// - [`employee_name`](AssignmentsListFleetResponseRowsItemBuilder::employee_name)
    /// - [`from_date`](AssignmentsListFleetResponseRowsItemBuilder::from_date)
    /// - [`private_use`](AssignmentsListFleetResponseRowsItemBuilder::private_use)
    /// - [`employer_pays_fuel`](AssignmentsListFleetResponseRowsItemBuilder::employer_pays_fuel)
    /// - [`created_at`](AssignmentsListFleetResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<AssignmentsListFleetResponseRowsItem, BuildError> {
        Ok(AssignmentsListFleetResponseRowsItem {
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
