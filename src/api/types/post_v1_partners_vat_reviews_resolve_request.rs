pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1PartnersVatReviewsResolveRequest {
    #[serde(default)]
    pub id: String,
    pub resolution: PostV1PartnersVatReviewsResolveRequestResolution,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl PostV1PartnersVatReviewsResolveRequest {
    pub fn builder() -> PostV1PartnersVatReviewsResolveRequestBuilder {
        <PostV1PartnersVatReviewsResolveRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersVatReviewsResolveRequestBuilder {
    id: Option<String>,
    resolution: Option<PostV1PartnersVatReviewsResolveRequestResolution>,
    note: Option<String>,
}

impl PostV1PartnersVatReviewsResolveRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn resolution(mut self, value: PostV1PartnersVatReviewsResolveRequestResolution) -> Self {
        self.resolution = Some(value);
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersVatReviewsResolveRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersVatReviewsResolveRequestBuilder::id)
    /// - [`resolution`](PostV1PartnersVatReviewsResolveRequestBuilder::resolution)
    pub fn build(self) -> Result<PostV1PartnersVatReviewsResolveRequest, BuildError> {
        Ok(PostV1PartnersVatReviewsResolveRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            resolution: self
                .resolution
                .ok_or_else(|| BuildError::missing_field("resolution"))?,
            note: self.note,
        })
    }
}
