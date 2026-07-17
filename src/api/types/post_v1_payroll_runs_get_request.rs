pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollRunsGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1PayrollRunsGetRequest {
    pub fn builder() -> PostV1PayrollRunsGetRequestBuilder {
        <PostV1PayrollRunsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollRunsGetRequestBuilder {
    id: Option<String>,
}

impl PostV1PayrollRunsGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollRunsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PayrollRunsGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PayrollRunsGetRequest, BuildError> {
        Ok(PostV1PayrollRunsGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
