pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlJpkMagGenerateRequest {
    #[serde(rename = "dateFrom")]
    #[serde(default)]
    pub date_from: String,
    #[serde(rename = "dateTo")]
    #[serde(default)]
    pub date_to: String,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
}

impl PostV1DeclarationsPlJpkMagGenerateRequest {
    pub fn builder() -> PostV1DeclarationsPlJpkMagGenerateRequestBuilder {
        <PostV1DeclarationsPlJpkMagGenerateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlJpkMagGenerateRequestBuilder {
    date_from: Option<String>,
    date_to: Option<String>,
    warehouse_id: Option<String>,
}

impl PostV1DeclarationsPlJpkMagGenerateRequestBuilder {
    pub fn date_from(mut self, value: impl Into<String>) -> Self {
        self.date_from = Some(value.into());
        self
    }

    pub fn date_to(mut self, value: impl Into<String>) -> Self {
        self.date_to = Some(value.into());
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlJpkMagGenerateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date_from`](PostV1DeclarationsPlJpkMagGenerateRequestBuilder::date_from)
    /// - [`date_to`](PostV1DeclarationsPlJpkMagGenerateRequestBuilder::date_to)
    pub fn build(self) -> Result<PostV1DeclarationsPlJpkMagGenerateRequest, BuildError> {
        Ok(PostV1DeclarationsPlJpkMagGenerateRequest {
            date_from: self
                .date_from
                .ok_or_else(|| BuildError::missing_field("date_from"))?,
            date_to: self
                .date_to
                .ok_or_else(|| BuildError::missing_field("date_to"))?,
            warehouse_id: self.warehouse_id,
        })
    }
}
