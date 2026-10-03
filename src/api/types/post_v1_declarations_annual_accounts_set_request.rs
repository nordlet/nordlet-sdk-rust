pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsAnnualAccountsSetRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub adopted: bool,
    #[serde(rename = "adoptionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adoption_date: Option<String>,
    #[serde(rename = "dateOfPreparation")]
    #[serde(default)]
    pub date_of_preparation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audited: Option<bool>,
    #[serde(rename = "auditReportQualified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_report_qualified: Option<bool>,
    #[serde(rename = "auditorNotElected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auditor_not_elected: Option<bool>,
    #[serde(rename = "notesText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes_text: Option<String>,
    #[serde(rename = "managementReportText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management_report_text: Option<String>,
    #[serde(rename = "auditorReportText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auditor_report_text: Option<String>,
    #[serde(rename = "auditorReportDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auditor_report_date: Option<String>,
    #[serde(rename = "resultToReserves")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_to_reserves: Option<String>,
    #[serde(rename = "resultToLossCompensation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_to_loss_compensation: Option<String>,
    #[serde(rename = "resultToRemainder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_to_remainder: Option<String>,
}

impl PostV1DeclarationsAnnualAccountsSetRequest {
    pub fn builder() -> PostV1DeclarationsAnnualAccountsSetRequestBuilder {
        <PostV1DeclarationsAnnualAccountsSetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsAnnualAccountsSetRequestBuilder {
    year: Option<i64>,
    adopted: Option<bool>,
    adoption_date: Option<String>,
    date_of_preparation: Option<String>,
    audited: Option<bool>,
    audit_report_qualified: Option<bool>,
    auditor_not_elected: Option<bool>,
    notes_text: Option<String>,
    management_report_text: Option<String>,
    auditor_report_text: Option<String>,
    auditor_report_date: Option<String>,
    result_to_reserves: Option<String>,
    result_to_loss_compensation: Option<String>,
    result_to_remainder: Option<String>,
}

impl PostV1DeclarationsAnnualAccountsSetRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn adopted(mut self, value: bool) -> Self {
        self.adopted = Some(value);
        self
    }

    pub fn adoption_date(mut self, value: impl Into<String>) -> Self {
        self.adoption_date = Some(value.into());
        self
    }

    pub fn date_of_preparation(mut self, value: impl Into<String>) -> Self {
        self.date_of_preparation = Some(value.into());
        self
    }

    pub fn audited(mut self, value: bool) -> Self {
        self.audited = Some(value);
        self
    }

    pub fn audit_report_qualified(mut self, value: bool) -> Self {
        self.audit_report_qualified = Some(value);
        self
    }

    pub fn auditor_not_elected(mut self, value: bool) -> Self {
        self.auditor_not_elected = Some(value);
        self
    }

    pub fn notes_text(mut self, value: impl Into<String>) -> Self {
        self.notes_text = Some(value.into());
        self
    }

    pub fn management_report_text(mut self, value: impl Into<String>) -> Self {
        self.management_report_text = Some(value.into());
        self
    }

    pub fn auditor_report_text(mut self, value: impl Into<String>) -> Self {
        self.auditor_report_text = Some(value.into());
        self
    }

    pub fn auditor_report_date(mut self, value: impl Into<String>) -> Self {
        self.auditor_report_date = Some(value.into());
        self
    }

    pub fn result_to_reserves(mut self, value: impl Into<String>) -> Self {
        self.result_to_reserves = Some(value.into());
        self
    }

    pub fn result_to_loss_compensation(mut self, value: impl Into<String>) -> Self {
        self.result_to_loss_compensation = Some(value.into());
        self
    }

    pub fn result_to_remainder(mut self, value: impl Into<String>) -> Self {
        self.result_to_remainder = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsAnnualAccountsSetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsAnnualAccountsSetRequestBuilder::year)
    /// - [`adopted`](PostV1DeclarationsAnnualAccountsSetRequestBuilder::adopted)
    /// - [`date_of_preparation`](PostV1DeclarationsAnnualAccountsSetRequestBuilder::date_of_preparation)
    pub fn build(self) -> Result<PostV1DeclarationsAnnualAccountsSetRequest, BuildError> {
        Ok(PostV1DeclarationsAnnualAccountsSetRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            adopted: self
                .adopted
                .ok_or_else(|| BuildError::missing_field("adopted"))?,
            adoption_date: self.adoption_date,
            date_of_preparation: self
                .date_of_preparation
                .ok_or_else(|| BuildError::missing_field("date_of_preparation"))?,
            audited: self.audited,
            audit_report_qualified: self.audit_report_qualified,
            auditor_not_elected: self.auditor_not_elected,
            notes_text: self.notes_text,
            management_report_text: self.management_report_text,
            auditor_report_text: self.auditor_report_text,
            auditor_report_date: self.auditor_report_date,
            result_to_reserves: self.result_to_reserves,
            result_to_loss_compensation: self.result_to_loss_compensation,
            result_to_remainder: self.result_to_remainder,
        })
    }
}
