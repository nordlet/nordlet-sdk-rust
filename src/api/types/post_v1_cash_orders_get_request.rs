pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CashOrdersGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1CashOrdersGetRequest {
    pub fn builder() -> PostV1CashOrdersGetRequestBuilder {
        <PostV1CashOrdersGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CashOrdersGetRequestBuilder {
    id: Option<String>,
}

impl PostV1CashOrdersGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CashOrdersGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CashOrdersGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1CashOrdersGetRequest, BuildError> {
        Ok(PostV1CashOrdersGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
