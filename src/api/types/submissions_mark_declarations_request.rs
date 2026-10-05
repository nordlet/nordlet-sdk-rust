pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubmissionsMarkDeclarationsRequest {
    #[serde(default)]
    pub id: String,
    pub status: SubmissionsMarkDeclarationsRequestStatus,
    #[serde(rename = "externalRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl SubmissionsMarkDeclarationsRequest {
    pub fn builder() -> SubmissionsMarkDeclarationsRequestBuilder {
        <SubmissionsMarkDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmissionsMarkDeclarationsRequestBuilder {
    id: Option<String>,
    status: Option<SubmissionsMarkDeclarationsRequestStatus>,
    external_ref: Option<String>,
    message: Option<String>,
}

impl SubmissionsMarkDeclarationsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: SubmissionsMarkDeclarationsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn external_ref(mut self, value: impl Into<String>) -> Self {
        self.external_ref = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubmissionsMarkDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SubmissionsMarkDeclarationsRequestBuilder::id)
    /// - [`status`](SubmissionsMarkDeclarationsRequestBuilder::status)
    pub fn build(self) -> Result<SubmissionsMarkDeclarationsRequest, BuildError> {
        Ok(SubmissionsMarkDeclarationsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            external_ref: self.external_ref,
            message: self.message,
        })
    }
}
