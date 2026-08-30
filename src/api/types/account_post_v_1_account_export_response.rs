pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountExportResponse {
    #[serde(rename = "generatedAt")]
    #[serde(default)]
    pub generated_at: String,
    #[serde(default)]
    pub user: PostV1AccountExportResponseUser,
    #[serde(default)]
    pub consent: PostV1AccountExportResponseConsent,
    #[serde(default)]
    pub memberships: Vec<PostV1AccountExportResponseMembershipsItem>,
    #[serde(default)]
    pub sessions: Vec<PostV1AccountExportResponseSessionsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<PostV1AccountExportResponseBilling>,
    #[serde(rename = "creditTransactions")]
    #[serde(default)]
    pub credit_transactions: Vec<PostV1AccountExportResponseCreditTransactionsItem>,
    #[serde(rename = "auditEntries")]
    #[serde(default)]
    pub audit_entries: Vec<PostV1AccountExportResponseAuditEntriesItem>,
}

impl PostV1AccountExportResponse {
    pub fn builder() -> PostV1AccountExportResponseBuilder {
        <PostV1AccountExportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountExportResponseBuilder {
    generated_at: Option<String>,
    user: Option<PostV1AccountExportResponseUser>,
    consent: Option<PostV1AccountExportResponseConsent>,
    memberships: Option<Vec<PostV1AccountExportResponseMembershipsItem>>,
    sessions: Option<Vec<PostV1AccountExportResponseSessionsItem>>,
    billing: Option<PostV1AccountExportResponseBilling>,
    credit_transactions: Option<Vec<PostV1AccountExportResponseCreditTransactionsItem>>,
    audit_entries: Option<Vec<PostV1AccountExportResponseAuditEntriesItem>>,
}

impl PostV1AccountExportResponseBuilder {
    pub fn generated_at(mut self, value: impl Into<String>) -> Self {
        self.generated_at = Some(value.into());
        self
    }

    pub fn user(mut self, value: PostV1AccountExportResponseUser) -> Self {
        self.user = Some(value);
        self
    }

    pub fn consent(mut self, value: PostV1AccountExportResponseConsent) -> Self {
        self.consent = Some(value);
        self
    }

    pub fn memberships(mut self, value: Vec<PostV1AccountExportResponseMembershipsItem>) -> Self {
        self.memberships = Some(value);
        self
    }

    pub fn sessions(mut self, value: Vec<PostV1AccountExportResponseSessionsItem>) -> Self {
        self.sessions = Some(value);
        self
    }

    pub fn billing(mut self, value: PostV1AccountExportResponseBilling) -> Self {
        self.billing = Some(value);
        self
    }

    pub fn credit_transactions(
        mut self,
        value: Vec<PostV1AccountExportResponseCreditTransactionsItem>,
    ) -> Self {
        self.credit_transactions = Some(value);
        self
    }

    pub fn audit_entries(
        mut self,
        value: Vec<PostV1AccountExportResponseAuditEntriesItem>,
    ) -> Self {
        self.audit_entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountExportResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`generated_at`](PostV1AccountExportResponseBuilder::generated_at)
    /// - [`user`](PostV1AccountExportResponseBuilder::user)
    /// - [`consent`](PostV1AccountExportResponseBuilder::consent)
    /// - [`memberships`](PostV1AccountExportResponseBuilder::memberships)
    /// - [`sessions`](PostV1AccountExportResponseBuilder::sessions)
    /// - [`credit_transactions`](PostV1AccountExportResponseBuilder::credit_transactions)
    /// - [`audit_entries`](PostV1AccountExportResponseBuilder::audit_entries)
    pub fn build(self) -> Result<PostV1AccountExportResponse, BuildError> {
        Ok(PostV1AccountExportResponse {
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
