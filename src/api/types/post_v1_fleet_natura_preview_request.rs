pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1FleetNaturaPreviewRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl PostV1FleetNaturaPreviewRequest {
    pub fn builder() -> PostV1FleetNaturaPreviewRequestBuilder {
        <PostV1FleetNaturaPreviewRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FleetNaturaPreviewRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl PostV1FleetNaturaPreviewRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1FleetNaturaPreviewRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1FleetNaturaPreviewRequestBuilder::year)
    /// - [`month`](PostV1FleetNaturaPreviewRequestBuilder::month)
    pub fn build(self) -> Result<PostV1FleetNaturaPreviewRequest, BuildError> {
        Ok(PostV1FleetNaturaPreviewRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
