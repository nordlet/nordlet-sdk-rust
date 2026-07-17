pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtSamComputeResponsePersonsItem {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(rename = "personalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_code: Option<String>,
    #[serde(rename = "socialInsuranceNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_insurance_no: Option<String>,
    #[serde(rename = "firstName")]
    #[serde(default)]
    pub first_name: String,
    #[serde(rename = "lastName")]
    #[serde(default)]
    pub last_name: String,
    #[serde(rename = "insuredIncome")]
    #[serde(default)]
    pub insured_income: String,
    #[serde(default)]
    pub contributions: String,
    #[serde(rename = "tariffPercent")]
    #[serde(default)]
    pub tariff_percent: String,
}

impl PostV1DeclarationsLtSamComputeResponsePersonsItem {
    pub fn builder() -> PostV1DeclarationsLtSamComputeResponsePersonsItemBuilder {
        <PostV1DeclarationsLtSamComputeResponsePersonsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtSamComputeResponsePersonsItemBuilder {
    employee_id: Option<String>,
    personal_code: Option<String>,
    social_insurance_no: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    insured_income: Option<String>,
    contributions: Option<String>,
    tariff_percent: Option<String>,
}

impl PostV1DeclarationsLtSamComputeResponsePersonsItemBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn personal_code(mut self, value: impl Into<String>) -> Self {
        self.personal_code = Some(value.into());
        self
    }

    pub fn social_insurance_no(mut self, value: impl Into<String>) -> Self {
        self.social_insurance_no = Some(value.into());
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

    pub fn insured_income(mut self, value: impl Into<String>) -> Self {
        self.insured_income = Some(value.into());
        self
    }

    pub fn contributions(mut self, value: impl Into<String>) -> Self {
        self.contributions = Some(value.into());
        self
    }

    pub fn tariff_percent(mut self, value: impl Into<String>) -> Self {
        self.tariff_percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtSamComputeResponsePersonsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](PostV1DeclarationsLtSamComputeResponsePersonsItemBuilder::employee_id)
    /// - [`first_name`](PostV1DeclarationsLtSamComputeResponsePersonsItemBuilder::first_name)
    /// - [`last_name`](PostV1DeclarationsLtSamComputeResponsePersonsItemBuilder::last_name)
    /// - [`insured_income`](PostV1DeclarationsLtSamComputeResponsePersonsItemBuilder::insured_income)
    /// - [`contributions`](PostV1DeclarationsLtSamComputeResponsePersonsItemBuilder::contributions)
    /// - [`tariff_percent`](PostV1DeclarationsLtSamComputeResponsePersonsItemBuilder::tariff_percent)
    pub fn build(self) -> Result<PostV1DeclarationsLtSamComputeResponsePersonsItem, BuildError> {
        Ok(PostV1DeclarationsLtSamComputeResponsePersonsItem {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            personal_code: self.personal_code,
            social_insurance_no: self.social_insurance_no,
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
            insured_income: self
                .insured_income
                .ok_or_else(|| BuildError::missing_field("insured_income"))?,
            contributions: self
                .contributions
                .ok_or_else(|| BuildError::missing_field("contributions"))?,
            tariff_percent: self
                .tariff_percent
                .ok_or_else(|| BuildError::missing_field("tariff_percent"))?,
        })
    }
}
