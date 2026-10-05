pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VatReviewsListPartnersResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "vatCode")]
    #[serde(default)]
    pub vat_code: String,
    pub reason: VatReviewsListPartnersResponseRowsItemReason,
    pub status: VatReviewsListPartnersResponseRowsItemStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<VatReviewsListPartnersResponseRowsItemResolution>,
    #[serde(rename = "resolutionNote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<VatReviewsListPartnersResponseRowsItemDetails>,
    #[serde(rename = "resolvedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub resolved_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl VatReviewsListPartnersResponseRowsItem {
    pub fn builder() -> VatReviewsListPartnersResponseRowsItemBuilder {
        <VatReviewsListPartnersResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatReviewsListPartnersResponseRowsItemBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    vat_code: Option<String>,
    reason: Option<VatReviewsListPartnersResponseRowsItemReason>,
    status: Option<VatReviewsListPartnersResponseRowsItemStatus>,
    resolution: Option<VatReviewsListPartnersResponseRowsItemResolution>,
    resolution_note: Option<String>,
    details: Option<VatReviewsListPartnersResponseRowsItemDetails>,
    resolved_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl VatReviewsListPartnersResponseRowsItemBuilder {
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

    pub fn reason(mut self, value: VatReviewsListPartnersResponseRowsItemReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn status(mut self, value: VatReviewsListPartnersResponseRowsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn resolution(mut self, value: VatReviewsListPartnersResponseRowsItemResolution) -> Self {
        self.resolution = Some(value);
        self
    }

    pub fn resolution_note(mut self, value: impl Into<String>) -> Self {
        self.resolution_note = Some(value.into());
        self
    }

    pub fn details(mut self, value: VatReviewsListPartnersResponseRowsItemDetails) -> Self {
        self.details = Some(value);
        self
    }

    pub fn resolved_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.resolved_at = Some(value);
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

    /// Consumes the builder and constructs a [`VatReviewsListPartnersResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](VatReviewsListPartnersResponseRowsItemBuilder::id)
    /// - [`partner_id`](VatReviewsListPartnersResponseRowsItemBuilder::partner_id)
    /// - [`vat_code`](VatReviewsListPartnersResponseRowsItemBuilder::vat_code)
    /// - [`reason`](VatReviewsListPartnersResponseRowsItemBuilder::reason)
    /// - [`status`](VatReviewsListPartnersResponseRowsItemBuilder::status)
    /// - [`created_at`](VatReviewsListPartnersResponseRowsItemBuilder::created_at)
    /// - [`updated_at`](VatReviewsListPartnersResponseRowsItemBuilder::updated_at)
    pub fn build(self) -> Result<VatReviewsListPartnersResponseRowsItem, BuildError> {
        Ok(VatReviewsListPartnersResponseRowsItem {
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
