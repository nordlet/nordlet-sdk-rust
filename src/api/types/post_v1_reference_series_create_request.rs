pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceSeriesCreateRequest {
    #[serde(rename = "documentType")]
    #[serde(default)]
    pub document_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "startAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
}

impl PostV1ReferenceSeriesCreateRequest {
    pub fn builder() -> PostV1ReferenceSeriesCreateRequestBuilder {
        <PostV1ReferenceSeriesCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceSeriesCreateRequestBuilder {
    document_type: Option<String>,
    prefix: Option<String>,
    year: Option<i64>,
    start_at: Option<i64>,
}

impl PostV1ReferenceSeriesCreateRequestBuilder {
    pub fn document_type(mut self, value: impl Into<String>) -> Self {
        self.document_type = Some(value.into());
        self
    }

    pub fn prefix(mut self, value: impl Into<String>) -> Self {
        self.prefix = Some(value.into());
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceSeriesCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_type`](PostV1ReferenceSeriesCreateRequestBuilder::document_type)
    /// - [`year`](PostV1ReferenceSeriesCreateRequestBuilder::year)
    pub fn build(self) -> Result<PostV1ReferenceSeriesCreateRequest, BuildError> {
        Ok(PostV1ReferenceSeriesCreateRequest {
            document_type: self
                .document_type
                .ok_or_else(|| BuildError::missing_field("document_type"))?,
            prefix: self.prefix,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            start_at: self.start_at,
        })
    }
}
