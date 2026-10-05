pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IntercompanyCandidatesConsolidationRequest {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
}

impl IntercompanyCandidatesConsolidationRequest {
    pub fn builder() -> IntercompanyCandidatesConsolidationRequestBuilder {
        <IntercompanyCandidatesConsolidationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntercompanyCandidatesConsolidationRequestBuilder {
    group_id: Option<String>,
}

impl IntercompanyCandidatesConsolidationRequestBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IntercompanyCandidatesConsolidationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](IntercompanyCandidatesConsolidationRequestBuilder::group_id)
    pub fn build(self) -> Result<IntercompanyCandidatesConsolidationRequest, BuildError> {
        Ok(IntercompanyCandidatesConsolidationRequest {
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
        })
    }
}
