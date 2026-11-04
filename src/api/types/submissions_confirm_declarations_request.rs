pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmissionsConfirmDeclarationsRequest {
    #[serde(default)]
    pub id: String,
}

impl SubmissionsConfirmDeclarationsRequest {
    pub fn builder() -> SubmissionsConfirmDeclarationsRequestBuilder {
        <SubmissionsConfirmDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmissionsConfirmDeclarationsRequestBuilder {
    id: Option<String>,
}

impl SubmissionsConfirmDeclarationsRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubmissionsConfirmDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SubmissionsConfirmDeclarationsRequestBuilder::id)
    pub fn build(self) -> Result<SubmissionsConfirmDeclarationsRequest, BuildError> {
        Ok(SubmissionsConfirmDeclarationsRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
