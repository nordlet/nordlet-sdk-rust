pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1PartnersVatReviewsResolveResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "vatCode")]
    #[serde(default)]
    pub vat_code: String,
    pub reason: PostV1PartnersVatReviewsResolveResponseReason,
    pub status: PostV1PartnersVatReviewsResolveResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<PostV1PartnersVatReviewsResolveResponseResolution>,
    #[serde(rename = "resolutionNote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<PostV1PartnersVatReviewsResolveResponseDetails>,
    #[serde(rename = "resolvedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl PostV1PartnersVatReviewsResolveResponse {
    pub fn builder() -> PostV1PartnersVatReviewsResolveResponseBuilder {
        <PostV1PartnersVatReviewsResolveResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersVatReviewsResolveResponseBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    vat_code: Option<String>,
    reason: Option<PostV1PartnersVatReviewsResolveResponseReason>,
    status: Option<PostV1PartnersVatReviewsResolveResponseStatus>,
    resolution: Option<PostV1PartnersVatReviewsResolveResponseResolution>,
    resolution_note: Option<String>,
    details: Option<PostV1PartnersVatReviewsResolveResponseDetails>,
    resolved_at: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl PostV1PartnersVatReviewsResolveResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn vat_code(mut self, value: impl Into<String>) -> Self {
        self.vat_code = Some(value.into());
        self
    }

    pub fn reason(mut self, value: PostV1PartnersVatReviewsResolveResponseReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn status(mut self, value: PostV1PartnersVatReviewsResolveResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn resolution(mut self, value: PostV1PartnersVatReviewsResolveResponseResolution) -> Self {
        self.resolution = Some(value);
        self
    }

    pub fn resolution_note(mut self, value: impl Into<String>) -> Self {
        self.resolution_note = Some(value.into());
        self
    }

    pub fn details(mut self, value: PostV1PartnersVatReviewsResolveResponseDetails) -> Self {
        self.details = Some(value);
        self
    }

    pub fn resolved_at(mut self, value: impl Into<String>) -> Self {
        self.resolved_at = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1PartnersVatReviewsResolveResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersVatReviewsResolveResponseBuilder::id)
    /// - [`partner_id`](PostV1PartnersVatReviewsResolveResponseBuilder::partner_id)
    /// - [`vat_code`](PostV1PartnersVatReviewsResolveResponseBuilder::vat_code)
    /// - [`reason`](PostV1PartnersVatReviewsResolveResponseBuilder::reason)
    /// - [`status`](PostV1PartnersVatReviewsResolveResponseBuilder::status)
    /// - [`created_at`](PostV1PartnersVatReviewsResolveResponseBuilder::created_at)
    /// - [`updated_at`](PostV1PartnersVatReviewsResolveResponseBuilder::updated_at)
    pub fn build(self) -> Result<PostV1PartnersVatReviewsResolveResponse, BuildError> {
        Ok(PostV1PartnersVatReviewsResolveResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            vat_code: self
                .vat_code
                .ok_or_else(|| BuildError::missing_field("vat_code"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            resolution: self.resolution,
            resolution_note: self.resolution_note,
            details: self.details,
            resolved_at: self.resolved_at,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
