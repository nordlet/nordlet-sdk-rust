pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtSdGenerateResponseRowsItem {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(rename = "contractId")]
    #[serde(default)]
    pub contract_id: String,
    #[serde(rename = "contractNo")]
    #[serde(default)]
    pub contract_no: String,
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
    #[serde(default)]
    pub date: String,
    #[serde(rename = "professionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profession_code: Option<String>,
    #[serde(rename = "endReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_reason: Option<String>,
    #[serde(rename = "finalInsuredIncome")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_insured_income: Option<String>,
    #[serde(rename = "finalContributions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_contributions: Option<String>,
}

impl PostV1DeclarationsLtSdGenerateResponseRowsItem {
    pub fn builder() -> PostV1DeclarationsLtSdGenerateResponseRowsItemBuilder {
        <PostV1DeclarationsLtSdGenerateResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtSdGenerateResponseRowsItemBuilder {
    employee_id: Option<String>,
    contract_id: Option<String>,
    contract_no: Option<String>,
    personal_code: Option<String>,
    social_insurance_no: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    date: Option<String>,
    profession_code: Option<String>,
    end_reason: Option<String>,
    final_insured_income: Option<String>,
    final_contributions: Option<String>,
}

impl PostV1DeclarationsLtSdGenerateResponseRowsItemBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn contract_id(mut self, value: impl Into<String>) -> Self {
        self.contract_id = Some(value.into());
        self
    }

    pub fn contract_no(mut self, value: impl Into<String>) -> Self {
        self.contract_no = Some(value.into());
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

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn profession_code(mut self, value: impl Into<String>) -> Self {
        self.profession_code = Some(value.into());
        self
    }

    pub fn end_reason(mut self, value: impl Into<String>) -> Self {
        self.end_reason = Some(value.into());
        self
    }

    pub fn final_insured_income(mut self, value: impl Into<String>) -> Self {
        self.final_insured_income = Some(value.into());
        self
    }

    pub fn final_contributions(mut self, value: impl Into<String>) -> Self {
        self.final_contributions = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtSdGenerateResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](PostV1DeclarationsLtSdGenerateResponseRowsItemBuilder::employee_id)
    /// - [`contract_id`](PostV1DeclarationsLtSdGenerateResponseRowsItemBuilder::contract_id)
    /// - [`contract_no`](PostV1DeclarationsLtSdGenerateResponseRowsItemBuilder::contract_no)
    /// - [`first_name`](PostV1DeclarationsLtSdGenerateResponseRowsItemBuilder::first_name)
    /// - [`last_name`](PostV1DeclarationsLtSdGenerateResponseRowsItemBuilder::last_name)
    /// - [`date`](PostV1DeclarationsLtSdGenerateResponseRowsItemBuilder::date)
    pub fn build(self) -> Result<PostV1DeclarationsLtSdGenerateResponseRowsItem, BuildError> {
        Ok(PostV1DeclarationsLtSdGenerateResponseRowsItem {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            contract_id: self
                .contract_id
                .ok_or_else(|| BuildError::missing_field("contract_id"))?,
            contract_no: self
                .contract_no
                .ok_or_else(|| BuildError::missing_field("contract_no"))?,
            personal_code: self.personal_code,
            social_insurance_no: self.social_insurance_no,
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            profession_code: self.profession_code,
            end_reason: self.end_reason,
            final_insured_income: self.final_insured_income,
            final_contributions: self.final_contributions,
        })
    }
}
