pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MergePartnersRequest {
    #[serde(rename = "sourceId")]
    #[serde(default)]
    pub source_id: String,
    #[serde(rename = "targetId")]
    #[serde(default)]
    pub target_id: String,
}

impl MergePartnersRequest {
    pub fn builder() -> MergePartnersRequestBuilder {
        <MergePartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MergePartnersRequestBuilder {
    source_id: Option<String>,
    target_id: Option<String>,
}

impl MergePartnersRequestBuilder {
    pub fn source_id(mut self, value: impl Into<String>) -> Self {
        self.source_id = Some(value.into());
        self
    }

    pub fn target_id(mut self, value: impl Into<String>) -> Self {
        self.target_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MergePartnersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source_id`](MergePartnersRequestBuilder::source_id)
    /// - [`target_id`](MergePartnersRequestBuilder::target_id)
    pub fn build(self) -> Result<MergePartnersRequest, BuildError> {
        Ok(MergePartnersRequest {
            source_id: self
                .source_id
                .ok_or_else(|| BuildError::missing_field("source_id"))?,
            target_id: self
                .target_id
                .ok_or_else(|| BuildError::missing_field("target_id"))?,
        })
    }
}
