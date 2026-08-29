pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesReceiptsGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1PurchasesReceiptsGetRequest {
    pub fn builder() -> PostV1PurchasesReceiptsGetRequestBuilder {
        <PostV1PurchasesReceiptsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesReceiptsGetRequestBuilder {
    id: Option<String>,
}

impl PostV1PurchasesReceiptsGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesReceiptsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PurchasesReceiptsGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PurchasesReceiptsGetRequest, BuildError> {
        Ok(PostV1PurchasesReceiptsGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
