pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AnnualAccountsGetDeclarationsResponseApproval {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub adopted: bool,
    #[serde(rename = "adoptionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adoption_date: Option<NaiveDate>,
    #[serde(rename = "dateOfPreparation")]
    #[serde(default)]
    pub date_of_preparation: String,
    #[serde(default)]
    pub audited: bool,
    #[serde(rename = "auditReportQualified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_report_qualified: Option<bool>,
    #[serde(rename = "auditorNotElected")]
    #[serde(default)]
    pub auditor_not_elected: bool,
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
    pub auditor_report_date: Option<NaiveDate>,
    #[serde(rename = "resultToReserves")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_to_reserves: Option<String>,
    #[serde(rename = "resultToLossCompensation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_to_loss_compensation: Option<String>,
    #[serde(rename = "resultToRemainder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_to_remainder: Option<String>,
    #[serde(default)]
    pub signatures: Vec<AnnualAccountsGetDeclarationsResponseApprovalSignaturesItem>,
    #[serde(default)]
    pub distributions: Vec<AnnualAccountsGetDeclarationsResponseApprovalDistributionsItem>,
    #[serde(default)]
    pub attachments: Vec<AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItem>,
}

impl AnnualAccountsGetDeclarationsResponseApproval {
    pub fn builder() -> AnnualAccountsGetDeclarationsResponseApprovalBuilder {
        <AnnualAccountsGetDeclarationsResponseApprovalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnnualAccountsGetDeclarationsResponseApprovalBuilder {
    id: Option<String>,
    year: Option<i64>,
    adopted: Option<bool>,
    adoption_date: Option<NaiveDate>,
    date_of_preparation: Option<String>,
    audited: Option<bool>,
    audit_report_qualified: Option<bool>,
    auditor_not_elected: Option<bool>,
    notes_text: Option<String>,
    management_report_text: Option<String>,
    auditor_report_text: Option<String>,
    auditor_report_date: Option<NaiveDate>,
    result_to_reserves: Option<String>,
    result_to_loss_compensation: Option<String>,
    result_to_remainder: Option<String>,
    signatures: Option<Vec<AnnualAccountsGetDeclarationsResponseApprovalSignaturesItem>>,
    distributions: Option<Vec<AnnualAccountsGetDeclarationsResponseApprovalDistributionsItem>>,
    attachments: Option<Vec<AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItem>>,
}

impl AnnualAccountsGetDeclarationsResponseApprovalBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn adopted(mut self, value: bool) -> Self {
        self.adopted = Some(value);
        self
    }

    pub fn adoption_date(mut self, value: NaiveDate) -> Self {
        self.adoption_date = Some(value);
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

    pub fn auditor_report_date(mut self, value: NaiveDate) -> Self {
        self.auditor_report_date = Some(value);
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

    pub fn signatures(
        mut self,
        value: Vec<AnnualAccountsGetDeclarationsResponseApprovalSignaturesItem>,
    ) -> Self {
        self.signatures = Some(value);
        self
    }

    pub fn distributions(
        mut self,
        value: Vec<AnnualAccountsGetDeclarationsResponseApprovalDistributionsItem>,
    ) -> Self {
        self.distributions = Some(value);
        self
    }

    pub fn attachments(
        mut self,
        value: Vec<AnnualAccountsGetDeclarationsResponseApprovalAttachmentsItem>,
    ) -> Self {
        self.attachments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AnnualAccountsGetDeclarationsResponseApproval`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AnnualAccountsGetDeclarationsResponseApprovalBuilder::id)
    /// - [`year`](AnnualAccountsGetDeclarationsResponseApprovalBuilder::year)
    /// - [`adopted`](AnnualAccountsGetDeclarationsResponseApprovalBuilder::adopted)
    /// - [`date_of_preparation`](AnnualAccountsGetDeclarationsResponseApprovalBuilder::date_of_preparation)
    /// - [`audited`](AnnualAccountsGetDeclarationsResponseApprovalBuilder::audited)
    /// - [`auditor_not_elected`](AnnualAccountsGetDeclarationsResponseApprovalBuilder::auditor_not_elected)
    /// - [`signatures`](AnnualAccountsGetDeclarationsResponseApprovalBuilder::signatures)
    /// - [`distributions`](AnnualAccountsGetDeclarationsResponseApprovalBuilder::distributions)
    /// - [`attachments`](AnnualAccountsGetDeclarationsResponseApprovalBuilder::attachments)
    pub fn build(self) -> Result<AnnualAccountsGetDeclarationsResponseApproval, BuildError> {
        Ok(AnnualAccountsGetDeclarationsResponseApproval {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            adopted: self
                .adopted
                .ok_or_else(|| BuildError::missing_field("adopted"))?,
            adoption_date: self.adoption_date,
            date_of_preparation: self
                .date_of_preparation
                .ok_or_else(|| BuildError::missing_field("date_of_preparation"))?,
            audited: self
                .audited
                .ok_or_else(|| BuildError::missing_field("audited"))?,
            audit_report_qualified: self.audit_report_qualified,
            auditor_not_elected: self
                .auditor_not_elected
                .ok_or_else(|| BuildError::missing_field("auditor_not_elected"))?,
            notes_text: self.notes_text,
            management_report_text: self.management_report_text,
            auditor_report_text: self.auditor_report_text,
            auditor_report_date: self.auditor_report_date,
            result_to_reserves: self.result_to_reserves,
            result_to_loss_compensation: self.result_to_loss_compensation,
            result_to_remainder: self.result_to_remainder,
            signatures: self
                .signatures
                .ok_or_else(|| BuildError::missing_field("signatures"))?,
            distributions: self
                .distributions
                .ok_or_else(|| BuildError::missing_field("distributions"))?,
            attachments: self
                .attachments
                .ok_or_else(|| BuildError::missing_field("attachments"))?,
        })
    }
}
