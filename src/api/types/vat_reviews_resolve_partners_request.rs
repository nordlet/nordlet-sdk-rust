pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VatReviewsResolvePartnersRequest {
    #[serde(default)]
    pub id: String,
    pub resolution: VatReviewsResolvePartnersRequestResolution,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl VatReviewsResolvePartnersRequest {
    pub fn builder() -> VatReviewsResolvePartnersRequestBuilder {
        <VatReviewsResolvePartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatReviewsResolvePartnersRequestBuilder {
    id: Option<String>,
    resolution: Option<VatReviewsResolvePartnersRequestResolution>,
    note: Option<String>,
}

impl VatReviewsResolvePartnersRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn resolution(mut self, value: VatReviewsResolvePartnersRequestResolution) -> Self {
        self.resolution = Some(value);
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VatReviewsResolvePartnersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](VatReviewsResolvePartnersRequestBuilder::id)
    /// - [`resolution`](VatReviewsResolvePartnersRequestBuilder::resolution)
    pub fn build(self) -> Result<VatReviewsResolvePartnersRequest, BuildError> {
        Ok(VatReviewsResolvePartnersRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            resolution: self
                .resolution
                .ok_or_else(|| BuildError::missing_field("resolution"))?,
            note: self.note,
        })
    }
}
