pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankMandatesCancelRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1BankMandatesCancelRequest {
    pub fn builder() -> PostV1BankMandatesCancelRequestBuilder {
        <PostV1BankMandatesCancelRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankMandatesCancelRequestBuilder {
    id: Option<String>,
}

impl PostV1BankMandatesCancelRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankMandatesCancelRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankMandatesCancelRequestBuilder::id)
    pub fn build(self) -> Result<PostV1BankMandatesCancelRequest, BuildError> {
        Ok(PostV1BankMandatesCancelRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
