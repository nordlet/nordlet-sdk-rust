pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersVatReviewsListResponseRowsItemDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "partnerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_name: Option<String>,
    #[serde(rename = "viesName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vies_name: Option<String>,
    #[serde(rename = "viesAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vies_address: Option<String>,
    #[serde(rename = "requestIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_identifier: Option<String>,
}

impl PostV1PartnersVatReviewsListResponseRowsItemDetails {
    pub fn builder() -> PostV1PartnersVatReviewsListResponseRowsItemDetailsBuilder {
        <PostV1PartnersVatReviewsListResponseRowsItemDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersVatReviewsListResponseRowsItemDetailsBuilder {
    message: Option<String>,
    partner_name: Option<String>,
    vies_name: Option<String>,
    vies_address: Option<String>,
    request_identifier: Option<String>,
}

impl PostV1PartnersVatReviewsListResponseRowsItemDetailsBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn partner_name(mut self, value: impl Into<String>) -> Self {
        self.partner_name = Some(value.into());
        self
    }

    pub fn vies_name(mut self, value: impl Into<String>) -> Self {
        self.vies_name = Some(value.into());
        self
    }

    pub fn vies_address(mut self, value: impl Into<String>) -> Self {
        self.vies_address = Some(value.into());
        self
    }

    pub fn request_identifier(mut self, value: impl Into<String>) -> Self {
        self.request_identifier = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersVatReviewsListResponseRowsItemDetails`].
    pub fn build(self) -> Result<PostV1PartnersVatReviewsListResponseRowsItemDetails, BuildError> {
        Ok(PostV1PartnersVatReviewsListResponseRowsItemDetails {
            message: self.message,
            partner_name: self.partner_name,
            vies_name: self.vies_name,
            vies_address: self.vies_address,
            request_identifier: self.request_identifier,
        })
    }
}
