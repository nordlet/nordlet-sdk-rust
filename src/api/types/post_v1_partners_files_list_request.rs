pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersFilesListRequest {
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
}

impl PostV1PartnersFilesListRequest {
    pub fn builder() -> PostV1PartnersFilesListRequestBuilder {
        <PostV1PartnersFilesListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersFilesListRequestBuilder {
    partner_id: Option<String>,
}

impl PostV1PartnersFilesListRequestBuilder {
    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersFilesListRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_id`](PostV1PartnersFilesListRequestBuilder::partner_id)
    pub fn build(self) -> Result<PostV1PartnersFilesListRequest, BuildError> {
        Ok(PostV1PartnersFilesListRequest {
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
        })
    }
}
