pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "firstName")]
    #[serde(default)]
    pub first_name: String,
    #[serde(rename = "lastName")]
    #[serde(default)]
    pub last_name: String,
    #[serde(rename = "personalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_code: Option<String>,
    #[serde(rename = "birthDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostV1HrEmployeesListResponseRowsItemAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(rename = "socialInsuranceNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_insurance_no: Option<String>,
    #[serde(rename = "socialInsuranceStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_insurance_start: Option<String>,
    #[serde(rename = "hireDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
    #[serde(rename = "terminationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_date: Option<String>,
    #[serde(rename = "applyNpd")]
    #[serde(default)]
    pub apply_npd: bool,
    #[serde(rename = "npdOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub npd_override: Option<String>,
    #[serde(rename = "pensionAccumulation")]
    #[serde(default)]
    pub pension_accumulation: bool,
    pub status: PostV1HrEmployeesListResponseRowsItemStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1HrEmployeesListResponseRowsItem {
    pub fn builder() -> PostV1HrEmployeesListResponseRowsItemBuilder {
        <PostV1HrEmployeesListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesListResponseRowsItemBuilder {
    id: Option<String>,
    code: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    personal_code: Option<String>,
    birth_date: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    address: Option<PostV1HrEmployeesListResponseRowsItemAddress>,
    iban: Option<String>,
    social_insurance_no: Option<String>,
    social_insurance_start: Option<String>,
    hire_date: Option<String>,
    termination_date: Option<String>,
    apply_npd: Option<bool>,
    npd_override: Option<String>,
    pension_accumulation: Option<bool>,
    status: Option<PostV1HrEmployeesListResponseRowsItemStatus>,
    notes: Option<String>,
    created_at: Option<String>,
}

impl PostV1HrEmployeesListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn personal_code(mut self, value: impl Into<String>) -> Self {
        self.personal_code = Some(value.into());
        self
    }

    pub fn birth_date(mut self, value: impl Into<String>) -> Self {
        self.birth_date = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn address(mut self, value: PostV1HrEmployeesListResponseRowsItemAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn iban(mut self, value: impl Into<String>) -> Self {
        self.iban = Some(value.into());
        self
    }

    pub fn social_insurance_no(mut self, value: impl Into<String>) -> Self {
        self.social_insurance_no = Some(value.into());
        self
    }

    pub fn social_insurance_start(mut self, value: impl Into<String>) -> Self {
        self.social_insurance_start = Some(value.into());
        self
    }

    pub fn hire_date(mut self, value: impl Into<String>) -> Self {
        self.hire_date = Some(value.into());
        self
    }

    pub fn termination_date(mut self, value: impl Into<String>) -> Self {
        self.termination_date = Some(value.into());
        self
    }

    pub fn apply_npd(mut self, value: bool) -> Self {
        self.apply_npd = Some(value);
        self
    }

    pub fn npd_override(mut self, value: impl Into<String>) -> Self {
        self.npd_override = Some(value.into());
        self
    }

    pub fn pension_accumulation(mut self, value: bool) -> Self {
        self.pension_accumulation = Some(value);
        self
    }

    pub fn status(mut self, value: PostV1HrEmployeesListResponseRowsItemStatus) -> Self {
        self.status = Some(value);
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

    /// Consumes the builder and constructs a [`PostV1HrEmployeesListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1HrEmployeesListResponseRowsItemBuilder::id)
    /// - [`first_name`](PostV1HrEmployeesListResponseRowsItemBuilder::first_name)
    /// - [`last_name`](PostV1HrEmployeesListResponseRowsItemBuilder::last_name)
    /// - [`apply_npd`](PostV1HrEmployeesListResponseRowsItemBuilder::apply_npd)
    /// - [`pension_accumulation`](PostV1HrEmployeesListResponseRowsItemBuilder::pension_accumulation)
    /// - [`status`](PostV1HrEmployeesListResponseRowsItemBuilder::status)
    /// - [`created_at`](PostV1HrEmployeesListResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1HrEmployeesListResponseRowsItem, BuildError> {
        Ok(PostV1HrEmployeesListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code,
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
            personal_code: self.personal_code,
            birth_date: self.birth_date,
            email: self.email,
            phone: self.phone,
            address: self.address,
            iban: self.iban,
            social_insurance_no: self.social_insurance_no,
            social_insurance_start: self.social_insurance_start,
            hire_date: self.hire_date,
            termination_date: self.termination_date,
            apply_npd: self
                .apply_npd
                .ok_or_else(|| BuildError::missing_field("apply_npd"))?,
            npd_override: self.npd_override,
            pension_accumulation: self
                .pension_accumulation
                .ok_or_else(|| BuildError::missing_field("pension_accumulation"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
