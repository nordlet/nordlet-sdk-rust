pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollRunsCancelRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1PayrollRunsCancelRequest {
    pub fn builder() -> PostV1PayrollRunsCancelRequestBuilder {
        <PostV1PayrollRunsCancelRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollRunsCancelRequestBuilder {
    id: Option<String>,
}

impl PostV1PayrollRunsCancelRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollRunsCancelRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PayrollRunsCancelRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PayrollRunsCancelRequest, BuildError> {
        Ok(PostV1PayrollRunsCancelRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
