pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersAnonymizeResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub anonymized: bool,
}

impl PostV1PartnersAnonymizeResponse {
    pub fn builder() -> PostV1PartnersAnonymizeResponseBuilder {
        <PostV1PartnersAnonymizeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersAnonymizeResponseBuilder {
    id: Option<String>,
    anonymized: Option<bool>,
}

impl PostV1PartnersAnonymizeResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn anonymized(mut self, value: bool) -> Self {
        self.anonymized = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersAnonymizeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersAnonymizeResponseBuilder::id)
    /// - [`anonymized`](PostV1PartnersAnonymizeResponseBuilder::anonymized)
    pub fn build(self) -> Result<PostV1PartnersAnonymizeResponse, BuildError> {
        Ok(PostV1PartnersAnonymizeResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            anonymized: self
                .anonymized
                .ok_or_else(|| BuildError::missing_field("anonymized"))?,
        })
    }
}
