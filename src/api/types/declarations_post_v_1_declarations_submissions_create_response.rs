pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsSubmissionsCreateResponse {
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
    pub status: PostV1DeclarationsSubmissionsCreateResponseStatus,
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
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl PostV1DeclarationsSubmissionsCreateResponse {
    pub fn builder() -> PostV1DeclarationsSubmissionsCreateResponseBuilder {
        <PostV1DeclarationsSubmissionsCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsSubmissionsCreateResponseBuilder {
    id: Option<String>,
    obligation: Option<String>,
    period_year: Option<i64>,
    period_month: Option<i64>,
    variant: Option<String>,
    status: Option<PostV1DeclarationsSubmissionsCreateResponseStatus>,
    file_name: Option<String>,
    file_id: Option<String>,
    external_ref: Option<String>,
    message: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    warnings: Option<Vec<String>>,
}

impl PostV1DeclarationsSubmissionsCreateResponseBuilder {
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

    pub fn status(mut self, value: PostV1DeclarationsSubmissionsCreateResponseStatus) -> Self {
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsSubmissionsCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1DeclarationsSubmissionsCreateResponseBuilder::id)
    /// - [`obligation`](PostV1DeclarationsSubmissionsCreateResponseBuilder::obligation)
    /// - [`period_year`](PostV1DeclarationsSubmissionsCreateResponseBuilder::period_year)
    /// - [`status`](PostV1DeclarationsSubmissionsCreateResponseBuilder::status)
    /// - [`file_name`](PostV1DeclarationsSubmissionsCreateResponseBuilder::file_name)
    /// - [`created_at`](PostV1DeclarationsSubmissionsCreateResponseBuilder::created_at)
    /// - [`updated_at`](PostV1DeclarationsSubmissionsCreateResponseBuilder::updated_at)
    /// - [`warnings`](PostV1DeclarationsSubmissionsCreateResponseBuilder::warnings)
    pub fn build(self) -> Result<PostV1DeclarationsSubmissionsCreateResponse, BuildError> {
        Ok(PostV1DeclarationsSubmissionsCreateResponse {
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
