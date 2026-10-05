pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubmissionsCreateDeclarationsResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub obligation: String,
    #[serde(rename = "periodYear")]
    #[serde(default)]
    pub period_year: i64,
    #[serde(rename = "periodMonth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_month: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    pub status: SubmissionsCreateDeclarationsResponseStatus,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "fileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(rename = "externalRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "ruleKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(rename = "documentKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_key: Option<String>,
    #[serde(default)]
    pub amendment: i64,
    #[serde(default)]
    pub origin: String,
    #[serde(rename = "transportSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<SubmissionsCreateDeclarationsResponseEnvironment>,
    #[serde(rename = "submittedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub submitted_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "acceptedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub accepted_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "rejectedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub rejected_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "checkedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub checked_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "nextCheckAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub next_check_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub attempts: i64,
    #[serde(rename = "deliveryError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_error: Option<String>,
    #[serde(rename = "sentSha256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_sha256: Option<String>,
    #[serde(rename = "certificateFingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_fingerprint: Option<String>,
    #[serde(rename = "submittedByActorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_by_actor_type: Option<String>,
    #[serde(rename = "submittedByActorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_by_actor_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl SubmissionsCreateDeclarationsResponse {
    pub fn builder() -> SubmissionsCreateDeclarationsResponseBuilder {
        <SubmissionsCreateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmissionsCreateDeclarationsResponseBuilder {
    id: Option<String>,
    obligation: Option<String>,
    period_year: Option<i64>,
    period_month: Option<i64>,
    variant: Option<String>,
    status: Option<SubmissionsCreateDeclarationsResponseStatus>,
    file_name: Option<String>,
    file_id: Option<String>,
    external_ref: Option<String>,
    message: Option<String>,
    rule_key: Option<String>,
    period: Option<String>,
    document_key: Option<String>,
    amendment: Option<i64>,
    origin: Option<String>,
    transport_system: Option<String>,
    environment: Option<SubmissionsCreateDeclarationsResponseEnvironment>,
    submitted_at: Option<DateTime<FixedOffset>>,
    accepted_at: Option<DateTime<FixedOffset>>,
    rejected_at: Option<DateTime<FixedOffset>>,
    checked_at: Option<DateTime<FixedOffset>>,
    next_check_at: Option<DateTime<FixedOffset>>,
    attempts: Option<i64>,
    delivery_error: Option<String>,
    sent_sha256: Option<String>,
    certificate_fingerprint: Option<String>,
    submitted_by_actor_type: Option<String>,
    submitted_by_actor_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    warnings: Option<Vec<String>>,
}

impl SubmissionsCreateDeclarationsResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn obligation(mut self, value: impl Into<String>) -> Self {
        self.obligation = Some(value.into());
        self
    }

    pub fn period_year(mut self, value: i64) -> Self {
        self.period_year = Some(value);
        self
    }

    pub fn period_month(mut self, value: i64) -> Self {
        self.period_month = Some(value);
        self
    }

    pub fn variant(mut self, value: impl Into<String>) -> Self {
        self.variant = Some(value.into());
        self
    }

    pub fn status(mut self, value: SubmissionsCreateDeclarationsResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn external_ref(mut self, value: impl Into<String>) -> Self {
        self.external_ref = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn rule_key(mut self, value: impl Into<String>) -> Self {
        self.rule_key = Some(value.into());
        self
    }

    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
        self
    }

    pub fn document_key(mut self, value: impl Into<String>) -> Self {
        self.document_key = Some(value.into());
        self
    }

    pub fn amendment(mut self, value: i64) -> Self {
        self.amendment = Some(value);
        self
    }

    pub fn origin(mut self, value: impl Into<String>) -> Self {
        self.origin = Some(value.into());
        self
    }

    pub fn transport_system(mut self, value: impl Into<String>) -> Self {
        self.transport_system = Some(value.into());
        self
    }

    pub fn environment(mut self, value: SubmissionsCreateDeclarationsResponseEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn submitted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.submitted_at = Some(value);
        self
    }

    pub fn accepted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.accepted_at = Some(value);
        self
    }

    pub fn rejected_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.rejected_at = Some(value);
        self
    }

    pub fn checked_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.checked_at = Some(value);
        self
    }

    pub fn next_check_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.next_check_at = Some(value);
        self
    }

    pub fn attempts(mut self, value: i64) -> Self {
        self.attempts = Some(value);
        self
    }

    pub fn delivery_error(mut self, value: impl Into<String>) -> Self {
        self.delivery_error = Some(value.into());
        self
    }

    pub fn sent_sha256(mut self, value: impl Into<String>) -> Self {
        self.sent_sha256 = Some(value.into());
        self
    }

    pub fn certificate_fingerprint(mut self, value: impl Into<String>) -> Self {
        self.certificate_fingerprint = Some(value.into());
        self
    }

    pub fn submitted_by_actor_type(mut self, value: impl Into<String>) -> Self {
        self.submitted_by_actor_type = Some(value.into());
        self
    }

    pub fn submitted_by_actor_id(mut self, value: impl Into<String>) -> Self {
        self.submitted_by_actor_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmissionsCreateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SubmissionsCreateDeclarationsResponseBuilder::id)
    /// - [`obligation`](SubmissionsCreateDeclarationsResponseBuilder::obligation)
    /// - [`period_year`](SubmissionsCreateDeclarationsResponseBuilder::period_year)
    /// - [`status`](SubmissionsCreateDeclarationsResponseBuilder::status)
    /// - [`file_name`](SubmissionsCreateDeclarationsResponseBuilder::file_name)
    /// - [`amendment`](SubmissionsCreateDeclarationsResponseBuilder::amendment)
    /// - [`origin`](SubmissionsCreateDeclarationsResponseBuilder::origin)
    /// - [`attempts`](SubmissionsCreateDeclarationsResponseBuilder::attempts)
    /// - [`created_at`](SubmissionsCreateDeclarationsResponseBuilder::created_at)
    /// - [`updated_at`](SubmissionsCreateDeclarationsResponseBuilder::updated_at)
    /// - [`warnings`](SubmissionsCreateDeclarationsResponseBuilder::warnings)
    pub fn build(self) -> Result<SubmissionsCreateDeclarationsResponse, BuildError> {
        Ok(SubmissionsCreateDeclarationsResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            obligation: self
                .obligation
                .ok_or_else(|| BuildError::missing_field("obligation"))?,
            period_year: self
                .period_year
                .ok_or_else(|| BuildError::missing_field("period_year"))?,
            period_month: self.period_month,
            variant: self.variant,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            file_id: self.file_id,
            external_ref: self.external_ref,
            message: self.message,
            rule_key: self.rule_key,
            period: self.period,
            document_key: self.document_key,
            amendment: self
                .amendment
                .ok_or_else(|| BuildError::missing_field("amendment"))?,
            origin: self
                .origin
                .ok_or_else(|| BuildError::missing_field("origin"))?,
            transport_system: self.transport_system,
            environment: self.environment,
            submitted_at: self.submitted_at,
            accepted_at: self.accepted_at,
            rejected_at: self.rejected_at,
            checked_at: self.checked_at,
            next_check_at: self.next_check_at,
            attempts: self
                .attempts
                .ok_or_else(|| BuildError::missing_field("attempts"))?,
            delivery_error: self.delivery_error,
            sent_sha256: self.sent_sha256,
            certificate_fingerprint: self.certificate_fingerprint,
            submitted_by_actor_type: self.submitted_by_actor_type,
            submitted_by_actor_id: self.submitted_by_actor_id,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
