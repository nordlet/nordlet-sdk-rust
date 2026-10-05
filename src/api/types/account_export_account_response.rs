pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExportAccountResponse {
    #[serde(rename = "generatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub generated_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub user: ExportAccountResponseUser,
    #[serde(default)]
    pub consent: ExportAccountResponseConsent,
    #[serde(default)]
    pub memberships: Vec<ExportAccountResponseMembershipsItem>,
    #[serde(default)]
    pub sessions: Vec<ExportAccountResponseSessionsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<ExportAccountResponseBilling>,
    #[serde(rename = "creditTransactions")]
    #[serde(default)]
    pub credit_transactions: Vec<ExportAccountResponseCreditTransactionsItem>,
    #[serde(rename = "auditEntries")]
    #[serde(default)]
    pub audit_entries: Vec<ExportAccountResponseAuditEntriesItem>,
}

impl ExportAccountResponse {
    pub fn builder() -> ExportAccountResponseBuilder {
        <ExportAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportAccountResponseBuilder {
    generated_at: Option<DateTime<FixedOffset>>,
    user: Option<ExportAccountResponseUser>,
    consent: Option<ExportAccountResponseConsent>,
    memberships: Option<Vec<ExportAccountResponseMembershipsItem>>,
    sessions: Option<Vec<ExportAccountResponseSessionsItem>>,
    billing: Option<ExportAccountResponseBilling>,
    credit_transactions: Option<Vec<ExportAccountResponseCreditTransactionsItem>>,
    audit_entries: Option<Vec<ExportAccountResponseAuditEntriesItem>>,
}

impl ExportAccountResponseBuilder {
    pub fn generated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.generated_at = Some(value);
        self
    }

    pub fn user(mut self, value: ExportAccountResponseUser) -> Self {
        self.user = Some(value);
        self
    }

    pub fn consent(mut self, value: ExportAccountResponseConsent) -> Self {
        self.consent = Some(value);
        self
    }

    pub fn memberships(mut self, value: Vec<ExportAccountResponseMembershipsItem>) -> Self {
        self.memberships = Some(value);
        self
    }

    pub fn sessions(mut self, value: Vec<ExportAccountResponseSessionsItem>) -> Self {
        self.sessions = Some(value);
        self
    }

    pub fn billing(mut self, value: ExportAccountResponseBilling) -> Self {
        self.billing = Some(value);
        self
    }

    pub fn credit_transactions(
        mut self,
        value: Vec<ExportAccountResponseCreditTransactionsItem>,
    ) -> Self {
        self.credit_transactions = Some(value);
        self
    }

    pub fn audit_entries(mut self, value: Vec<ExportAccountResponseAuditEntriesItem>) -> Self {
        self.audit_entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExportAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`generated_at`](ExportAccountResponseBuilder::generated_at)
    /// - [`user`](ExportAccountResponseBuilder::user)
    /// - [`consent`](ExportAccountResponseBuilder::consent)
    /// - [`memberships`](ExportAccountResponseBuilder::memberships)
    /// - [`sessions`](ExportAccountResponseBuilder::sessions)
    /// - [`credit_transactions`](ExportAccountResponseBuilder::credit_transactions)
    /// - [`audit_entries`](ExportAccountResponseBuilder::audit_entries)
    pub fn build(self) -> Result<ExportAccountResponse, BuildError> {
        Ok(ExportAccountResponse {
            generated_at: self
                .generated_at
                .ok_or_else(|| BuildError::missing_field("generated_at"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
            consent: self
                .consent
                .ok_or_else(|| BuildError::missing_field("consent"))?,
            memberships: self
                .memberships
                .ok_or_else(|| BuildError::missing_field("memberships"))?,
            sessions: self
                .sessions
                .ok_or_else(|| BuildError::missing_field("sessions"))?,
            billing: self.billing,
            credit_transactions: self
                .credit_transactions
                .ok_or_else(|| BuildError::missing_field("credit_transactions"))?,
            audit_entries: self
                .audit_entries
                .ok_or_else(|| BuildError::missing_field("audit_entries"))?,
        })
    }
}
