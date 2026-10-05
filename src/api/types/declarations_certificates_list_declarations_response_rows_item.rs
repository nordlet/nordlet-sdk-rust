pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CertificatesListDeclarationsResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub system: String,
    #[serde(rename = "fieldKey")]
    #[serde(default)]
    pub field_key: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    pub format: CertificatesListDeclarationsResponseRowsItemFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "notBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    #[serde(rename = "notAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after: Option<String>,
    #[serde(default)]
    pub sha256: String,
    pub health: CertificatesListDeclarationsResponseRowsItemHealth,
    #[serde(rename = "daysLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_left: Option<i64>,
    #[serde(rename = "uploadedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub uploaded_at: DateTime<FixedOffset>,
}

impl CertificatesListDeclarationsResponseRowsItem {
    pub fn builder() -> CertificatesListDeclarationsResponseRowsItemBuilder {
        <CertificatesListDeclarationsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CertificatesListDeclarationsResponseRowsItemBuilder {
    id: Option<String>,
    system: Option<String>,
    field_key: Option<String>,
    file_name: Option<String>,
    format: Option<CertificatesListDeclarationsResponseRowsItemFormat>,
    fingerprint: Option<String>,
    subject: Option<String>,
    issuer: Option<String>,
    not_before: Option<String>,
    not_after: Option<String>,
    sha256: Option<String>,
    health: Option<CertificatesListDeclarationsResponseRowsItemHealth>,
    days_left: Option<i64>,
    uploaded_at: Option<DateTime<FixedOffset>>,
}

impl CertificatesListDeclarationsResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn system(mut self, value: impl Into<String>) -> Self {
        self.system = Some(value.into());
        self
    }

    pub fn field_key(mut self, value: impl Into<String>) -> Self {
        self.field_key = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn format(mut self, value: CertificatesListDeclarationsResponseRowsItemFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.fingerprint = Some(value.into());
        self
    }

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    pub fn issuer(mut self, value: impl Into<String>) -> Self {
        self.issuer = Some(value.into());
        self
    }

    pub fn not_before(mut self, value: impl Into<String>) -> Self {
        self.not_before = Some(value.into());
        self
    }

    pub fn not_after(mut self, value: impl Into<String>) -> Self {
        self.not_after = Some(value.into());
        self
    }

    pub fn sha256(mut self, value: impl Into<String>) -> Self {
        self.sha256 = Some(value.into());
        self
    }

    pub fn health(mut self, value: CertificatesListDeclarationsResponseRowsItemHealth) -> Self {
        self.health = Some(value);
        self
    }

    pub fn days_left(mut self, value: i64) -> Self {
        self.days_left = Some(value);
        self
    }

    pub fn uploaded_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.uploaded_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CertificatesListDeclarationsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CertificatesListDeclarationsResponseRowsItemBuilder::id)
    /// - [`system`](CertificatesListDeclarationsResponseRowsItemBuilder::system)
    /// - [`field_key`](CertificatesListDeclarationsResponseRowsItemBuilder::field_key)
    /// - [`file_name`](CertificatesListDeclarationsResponseRowsItemBuilder::file_name)
    /// - [`format`](CertificatesListDeclarationsResponseRowsItemBuilder::format)
    /// - [`sha256`](CertificatesListDeclarationsResponseRowsItemBuilder::sha256)
    /// - [`health`](CertificatesListDeclarationsResponseRowsItemBuilder::health)
    /// - [`uploaded_at`](CertificatesListDeclarationsResponseRowsItemBuilder::uploaded_at)
    pub fn build(self) -> Result<CertificatesListDeclarationsResponseRowsItem, BuildError> {
        Ok(CertificatesListDeclarationsResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            system: self
                .system
                .ok_or_else(|| BuildError::missing_field("system"))?,
            field_key: self
                .field_key
                .ok_or_else(|| BuildError::missing_field("field_key"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            format: self
                .format
                .ok_or_else(|| BuildError::missing_field("format"))?,
            fingerprint: self.fingerprint,
            subject: self.subject,
            issuer: self.issuer,
            not_before: self.not_before,
            not_after: self.not_after,
            sha256: self
                .sha256
                .ok_or_else(|| BuildError::missing_field("sha256"))?,
            health: self
                .health
                .ok_or_else(|| BuildError::missing_field("health"))?,
            days_left: self.days_left,
            uploaded_at: self
                .uploaded_at
                .ok_or_else(|| BuildError::missing_field("uploaded_at"))?,
        })
    }
}
