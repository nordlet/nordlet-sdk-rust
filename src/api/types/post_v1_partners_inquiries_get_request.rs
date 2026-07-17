pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersInquiriesGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1PartnersInquiriesGetRequest {
    pub fn builder() -> PostV1PartnersInquiriesGetRequestBuilder {
        <PostV1PartnersInquiriesGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersInquiriesGetRequestBuilder {
    id: Option<String>,
}

impl PostV1PartnersInquiriesGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersInquiriesGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersInquiriesGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PartnersInquiriesGetRequest, BuildError> {
        Ok(PostV1PartnersInquiriesGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
