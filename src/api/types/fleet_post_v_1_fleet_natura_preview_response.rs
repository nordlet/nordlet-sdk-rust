pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1FleetNaturaPreviewResponse {
    #[serde(default)]
    pub rows: Vec<PostV1FleetNaturaPreviewResponseRowsItem>,
    #[serde(default)]
    pub total: String,
}

impl PostV1FleetNaturaPreviewResponse {
    pub fn builder() -> PostV1FleetNaturaPreviewResponseBuilder {
        <PostV1FleetNaturaPreviewResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FleetNaturaPreviewResponseBuilder {
    rows: Option<Vec<PostV1FleetNaturaPreviewResponseRowsItem>>,
    total: Option<String>,
}

impl PostV1FleetNaturaPreviewResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1FleetNaturaPreviewResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1FleetNaturaPreviewResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1FleetNaturaPreviewResponseBuilder::rows)
    /// - [`total`](PostV1FleetNaturaPreviewResponseBuilder::total)
    pub fn build(self) -> Result<PostV1FleetNaturaPreviewResponse, BuildError> {
        Ok(PostV1FleetNaturaPreviewResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
