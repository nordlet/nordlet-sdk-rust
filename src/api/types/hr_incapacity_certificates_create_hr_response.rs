pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IncapacityCertificatesCreateHrResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    #[serde(default)]
    pub number: String,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl IncapacityCertificatesCreateHrResponse {
    pub fn builder() -> IncapacityCertificatesCreateHrResponseBuilder {
        <IncapacityCertificatesCreateHrResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IncapacityCertificatesCreateHrResponseBuilder {
    id: Option<String>,
    employee_id: Option<String>,
    series: Option<String>,
    number: Option<String>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    reason: Option<String>,
    notes: Option<String>,
}

impl IncapacityCertificatesCreateHrResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn series(mut self, value: impl Into<String>) -> Self {
        self.series = Some(value.into());
        self
    }

    pub fn number(mut self, value: impl Into<String>) -> Self {
        self.number = Some(value.into());
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

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IncapacityCertificatesCreateHrResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](IncapacityCertificatesCreateHrResponseBuilder::id)
    /// - [`employee_id`](IncapacityCertificatesCreateHrResponseBuilder::employee_id)
    /// - [`number`](IncapacityCertificatesCreateHrResponseBuilder::number)
    /// - [`from_date`](IncapacityCertificatesCreateHrResponseBuilder::from_date)
    /// - [`to_date`](IncapacityCertificatesCreateHrResponseBuilder::to_date)
    pub fn build(self) -> Result<IncapacityCertificatesCreateHrResponse, BuildError> {
        Ok(IncapacityCertificatesCreateHrResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            series: self.series,
            number: self
                .number
                .ok_or_else(|| BuildError::missing_field("number"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            reason: self.reason,
            notes: self.notes,
        })
    }
}
