pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EmployeesRecordsListHrResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    pub r#type: EmployeesRecordsListHrResponseRowsItemType,
    #[serde(default)]
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution: Option<String>,
    #[serde(rename = "issuedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub issued_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "validUntil")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<String>,
    #[serde(rename = "fileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl EmployeesRecordsListHrResponseRowsItem {
    pub fn builder() -> EmployeesRecordsListHrResponseRowsItemBuilder {
        <EmployeesRecordsListHrResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesRecordsListHrResponseRowsItemBuilder {
    id: Option<String>,
    employee_id: Option<String>,
    r#type: Option<EmployeesRecordsListHrResponseRowsItemType>,
    title: Option<String>,
    institution: Option<String>,
    issued_at: Option<DateTime<FixedOffset>>,
    valid_until: Option<String>,
    file_id: Option<String>,
    notes: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl EmployeesRecordsListHrResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: EmployeesRecordsListHrResponseRowsItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn institution(mut self, value: impl Into<String>) -> Self {
        self.institution = Some(value.into());
        self
    }

    pub fn issued_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.issued_at = Some(value);
        self
    }

    pub fn valid_until(mut self, value: impl Into<String>) -> Self {
        self.valid_until = Some(value.into());
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`EmployeesRecordsListHrResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](EmployeesRecordsListHrResponseRowsItemBuilder::id)
    /// - [`employee_id`](EmployeesRecordsListHrResponseRowsItemBuilder::employee_id)
    /// - [`r#type`](EmployeesRecordsListHrResponseRowsItemBuilder::r#type)
    /// - [`title`](EmployeesRecordsListHrResponseRowsItemBuilder::title)
    /// - [`created_at`](EmployeesRecordsListHrResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<EmployeesRecordsListHrResponseRowsItem, BuildError> {
        Ok(EmployeesRecordsListHrResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            institution: self.institution,
            issued_at: self.issued_at,
            valid_until: self.valid_until,
            file_id: self.file_id,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
