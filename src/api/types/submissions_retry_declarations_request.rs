pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmissionsRetryDeclarationsRequest {
    #[serde(default)]
    pub id: String,
}

impl SubmissionsRetryDeclarationsRequest {
    pub fn builder() -> SubmissionsRetryDeclarationsRequestBuilder {
        <SubmissionsRetryDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmissionsRetryDeclarationsRequestBuilder {
    id: Option<String>,
}

impl SubmissionsRetryDeclarationsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubmissionsRetryDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SubmissionsRetryDeclarationsRequestBuilder::id)
    pub fn build(self) -> Result<SubmissionsRetryDeclarationsRequest, BuildError> {
        Ok(SubmissionsRetryDeclarationsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
