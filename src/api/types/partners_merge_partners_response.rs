pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MergePartnersResponse {
    #[serde(rename = "targetId")]
    #[serde(default)]
    pub target_id: String,
    #[serde(rename = "sourceId")]
    #[serde(default)]
    pub source_id: String,
    #[serde(default)]
    pub moved: Vec<MergePartnersResponseMovedItem>,
}

impl MergePartnersResponse {
    pub fn builder() -> MergePartnersResponseBuilder {
        <MergePartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MergePartnersResponseBuilder {
    target_id: Option<String>,
    source_id: Option<String>,
    moved: Option<Vec<MergePartnersResponseMovedItem>>,
}

impl MergePartnersResponseBuilder {
    pub fn target_id(mut self, value: impl Into<String>) -> Self {
        self.target_id = Some(value.into());
        self
    }

    pub fn source_id(mut self, value: impl Into<String>) -> Self {
        self.source_id = Some(value.into());
        self
    }

    pub fn moved(mut self, value: Vec<MergePartnersResponseMovedItem>) -> Self {
        self.moved = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MergePartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`target_id`](MergePartnersResponseBuilder::target_id)
    /// - [`source_id`](MergePartnersResponseBuilder::source_id)
    /// - [`moved`](MergePartnersResponseBuilder::moved)
    pub fn build(self) -> Result<MergePartnersResponse, BuildError> {
        Ok(MergePartnersResponse {
            target_id: self
                .target_id
                .ok_or_else(|| BuildError::missing_field("target_id"))?,
            source_id: self
                .source_id
                .ok_or_else(|| BuildError::missing_field("source_id"))?,
            moved: self
                .moved
                .ok_or_else(|| BuildError::missing_field("moved"))?,
        })
    }
}
