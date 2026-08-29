pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1PartnersVatReviewsListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "vatCode")]
    #[serde(default)]
    pub vat_code: String,
    pub reason: PostV1PartnersVatReviewsListResponseRowsItemReason,
    pub status: PostV1PartnersVatReviewsListResponseRowsItemStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<PostV1PartnersVatReviewsListResponseRowsItemResolution>,
    #[serde(rename = "resolutionNote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<PostV1PartnersVatReviewsListResponseRowsItemDetails>,
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

impl PostV1PartnersVatReviewsListResponseRowsItem {
    pub fn builder() -> PostV1PartnersVatReviewsListResponseRowsItemBuilder {
        <PostV1PartnersVatReviewsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersVatReviewsListResponseRowsItemBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    vat_code: Option<String>,
    reason: Option<PostV1PartnersVatReviewsListResponseRowsItemReason>,
    status: Option<PostV1PartnersVatReviewsListResponseRowsItemStatus>,
    resolution: Option<PostV1PartnersVatReviewsListResponseRowsItemResolution>,
    resolution_note: Option<String>,
    details: Option<PostV1PartnersVatReviewsListResponseRowsItemDetails>,
    resolved_at: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl PostV1PartnersVatReviewsListResponseRowsItemBuilder {
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

    pub fn reason(mut self, value: PostV1PartnersVatReviewsListResponseRowsItemReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn status(mut self, value: PostV1PartnersVatReviewsListResponseRowsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn resolution(
        mut self,
        value: PostV1PartnersVatReviewsListResponseRowsItemResolution,
    ) -> Self {
        self.resolution = Some(value);
        self
    }

    pub fn resolution_note(mut self, value: impl Into<String>) -> Self {
        self.resolution_note = Some(value.into());
        self
    }

    pub fn details(mut self, value: PostV1PartnersVatReviewsListResponseRowsItemDetails) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1PartnersVatReviewsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersVatReviewsListResponseRowsItemBuilder::id)
    /// - [`partner_id`](PostV1PartnersVatReviewsListResponseRowsItemBuilder::partner_id)
    /// - [`vat_code`](PostV1PartnersVatReviewsListResponseRowsItemBuilder::vat_code)
    /// - [`reason`](PostV1PartnersVatReviewsListResponseRowsItemBuilder::reason)
    /// - [`status`](PostV1PartnersVatReviewsListResponseRowsItemBuilder::status)
    /// - [`created_at`](PostV1PartnersVatReviewsListResponseRowsItemBuilder::created_at)
    /// - [`updated_at`](PostV1PartnersVatReviewsListResponseRowsItemBuilder::updated_at)
    pub fn build(self) -> Result<PostV1PartnersVatReviewsListResponseRowsItem, BuildError> {
        Ok(PostV1PartnersVatReviewsListResponseRowsItem {
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
