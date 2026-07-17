pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1SalesInvoicesDeleteResponse {
    pub fn builder() -> PostV1SalesInvoicesDeleteResponseBuilder {
        <PostV1SalesInvoicesDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1SalesInvoicesDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesInvoicesDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1SalesInvoicesDeleteResponse, BuildError> {
        Ok(PostV1SalesInvoicesDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
