pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FilesListLeadsRequest {
    #[serde(rename = "leadId")]
    #[serde(default)]
    pub lead_id: String,
}

impl FilesListLeadsRequest {
    pub fn builder() -> FilesListLeadsRequestBuilder {
        <FilesListLeadsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FilesListLeadsRequestBuilder {
    lead_id: Option<String>,
}

impl FilesListLeadsRequestBuilder {
    pub fn lead_id(mut self, value: impl Into<String>) -> Self {
        self.lead_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FilesListLeadsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`lead_id`](FilesListLeadsRequestBuilder::lead_id)
    pub fn build(self) -> Result<FilesListLeadsRequest, BuildError> {
        Ok(FilesListLeadsRequest {
            lead_id: self
                .lead_id
                .ok_or_else(|| BuildError::missing_field("lead_id"))?,
        })
    }
}
