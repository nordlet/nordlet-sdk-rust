pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsCertificatesDeleteResponseRowsItem {
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
    pub format: PostV1DeclarationsCertificatesDeleteResponseRowsItemFormat,
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
    pub health: PostV1DeclarationsCertificatesDeleteResponseRowsItemHealth,
    #[serde(rename = "daysLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_left: Option<i64>,
    #[serde(rename = "uploadedAt")]
    #[serde(default)]
    pub uploaded_at: String,
}

impl PostV1DeclarationsCertificatesDeleteResponseRowsItem {
    pub fn builder() -> PostV1DeclarationsCertificatesDeleteResponseRowsItemBuilder {
        <PostV1DeclarationsCertificatesDeleteResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsCertificatesDeleteResponseRowsItemBuilder {
    id: Option<String>,
    system: Option<String>,
    field_key: Option<String>,
    file_name: Option<String>,
    format: Option<PostV1DeclarationsCertificatesDeleteResponseRowsItemFormat>,
    fingerprint: Option<String>,
    subject: Option<String>,
    issuer: Option<String>,
    not_before: Option<String>,
    not_after: Option<String>,
    sha256: Option<String>,
    health: Option<PostV1DeclarationsCertificatesDeleteResponseRowsItemHealth>,
    days_left: Option<i64>,
    uploaded_at: Option<String>,
}

impl PostV1DeclarationsCertificatesDeleteResponseRowsItemBuilder {
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

    pub fn format(
        mut self,
        value: PostV1DeclarationsCertificatesDeleteResponseRowsItemFormat,
    ) -> Self {
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

    pub fn health(
        mut self,
        value: PostV1DeclarationsCertificatesDeleteResponseRowsItemHealth,
    ) -> Self {
        self.health = Some(value);
        self
    }

    pub fn days_left(mut self, value: i64) -> Self {
        self.days_left = Some(value);
        self
    }

    pub fn uploaded_at(mut self, value: impl Into<String>) -> Self {
        self.uploaded_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsCertificatesDeleteResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1DeclarationsCertificatesDeleteResponseRowsItemBuilder::id)
    /// - [`system`](PostV1DeclarationsCertificatesDeleteResponseRowsItemBuilder::system)
    /// - [`field_key`](PostV1DeclarationsCertificatesDeleteResponseRowsItemBuilder::field_key)
    /// - [`file_name`](PostV1DeclarationsCertificatesDeleteResponseRowsItemBuilder::file_name)
    /// - [`format`](PostV1DeclarationsCertificatesDeleteResponseRowsItemBuilder::format)
    /// - [`sha256`](PostV1DeclarationsCertificatesDeleteResponseRowsItemBuilder::sha256)
    /// - [`health`](PostV1DeclarationsCertificatesDeleteResponseRowsItemBuilder::health)
    /// - [`uploaded_at`](PostV1DeclarationsCertificatesDeleteResponseRowsItemBuilder::uploaded_at)
    pub fn build(self) -> Result<PostV1DeclarationsCertificatesDeleteResponseRowsItem, BuildError> {
        Ok(PostV1DeclarationsCertificatesDeleteResponseRowsItem {
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
