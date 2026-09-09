pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsFilesListRequest {
    #[serde(rename = "leadId")]
    #[serde(default)]
    pub lead_id: String,
}

impl PostV1LeadsFilesListRequest {
    pub fn builder() -> PostV1LeadsFilesListRequestBuilder {
        <PostV1LeadsFilesListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsFilesListRequestBuilder {
    lead_id: Option<String>,
}

impl PostV1LeadsFilesListRequestBuilder {
    pub fn lead_id(mut self, value: impl Into<String>) -> Self {
        self.lead_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsFilesListRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`lead_id`](PostV1LeadsFilesListRequestBuilder::lead_id)
    pub fn build(self) -> Result<PostV1LeadsFilesListRequest, BuildError> {
        Ok(PostV1LeadsFilesListRequest {
            lead_id: self
                .lead_id
                .ok_or_else(|| BuildError::missing_field("lead_id"))?,
        })
    }
}
