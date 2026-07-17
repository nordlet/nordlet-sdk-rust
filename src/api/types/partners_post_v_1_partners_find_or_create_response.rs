pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1PartnersFindOrCreateResponse {
    #[serde(default)]
    pub created: bool,
    pub partner: PostV1PartnersFindOrCreateResponsePartner,
}

impl PostV1PartnersFindOrCreateResponse {
    pub fn builder() -> PostV1PartnersFindOrCreateResponseBuilder {
        <PostV1PartnersFindOrCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersFindOrCreateResponseBuilder {
    created: Option<bool>,
    partner: Option<PostV1PartnersFindOrCreateResponsePartner>,
}

impl PostV1PartnersFindOrCreateResponseBuilder {
    pub fn created(mut self, value: bool) -> Self {
        self.created = Some(value);
        self
    }

    pub fn partner(mut self, value: PostV1PartnersFindOrCreateResponsePartner) -> Self {
        self.partner = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersFindOrCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](PostV1PartnersFindOrCreateResponseBuilder::created)
    /// - [`partner`](PostV1PartnersFindOrCreateResponseBuilder::partner)
    pub fn build(self) -> Result<PostV1PartnersFindOrCreateResponse, BuildError> {
        Ok(PostV1PartnersFindOrCreateResponse {
            created: self
                .created
                .ok_or_else(|| BuildError::missing_field("created"))?,
            partner: self
                .partner
                .ok_or_else(|| BuildError::missing_field("partner"))?,
        })
    }
}
