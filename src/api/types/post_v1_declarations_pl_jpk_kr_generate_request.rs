pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlJpkKrGenerateRequest {
    #[serde(rename = "dateFrom")]
    #[serde(default)]
    pub date_from: String,
    #[serde(rename = "dateTo")]
    #[serde(default)]
    pub date_to: String,
}

impl PostV1DeclarationsPlJpkKrGenerateRequest {
    pub fn builder() -> PostV1DeclarationsPlJpkKrGenerateRequestBuilder {
        <PostV1DeclarationsPlJpkKrGenerateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlJpkKrGenerateRequestBuilder {
    date_from: Option<String>,
    date_to: Option<String>,
}

impl PostV1DeclarationsPlJpkKrGenerateRequestBuilder {
    pub fn date_from(mut self, value: impl Into<String>) -> Self {
        self.date_from = Some(value.into());
        self
    }

    pub fn date_to(mut self, value: impl Into<String>) -> Self {
        self.date_to = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlJpkKrGenerateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date_from`](PostV1DeclarationsPlJpkKrGenerateRequestBuilder::date_from)
    /// - [`date_to`](PostV1DeclarationsPlJpkKrGenerateRequestBuilder::date_to)
    pub fn build(self) -> Result<PostV1DeclarationsPlJpkKrGenerateRequest, BuildError> {
        Ok(PostV1DeclarationsPlJpkKrGenerateRequest {
            date_from: self
                .date_from
                .ok_or_else(|| BuildError::missing_field("date_from"))?,
            date_to: self
                .date_to
                .ok_or_else(|| BuildError::missing_field("date_to"))?,
        })
    }
}
