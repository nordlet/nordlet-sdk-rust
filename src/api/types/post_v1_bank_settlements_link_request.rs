pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankSettlementsLinkRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "bankTransactionId")]
    #[serde(default)]
    pub bank_transaction_id: String,
}

impl PostV1BankSettlementsLinkRequest {
    pub fn builder() -> PostV1BankSettlementsLinkRequestBuilder {
        <PostV1BankSettlementsLinkRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankSettlementsLinkRequestBuilder {
    id: Option<String>,
    bank_transaction_id: Option<String>,
}

impl PostV1BankSettlementsLinkRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn bank_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.bank_transaction_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankSettlementsLinkRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankSettlementsLinkRequestBuilder::id)
    /// - [`bank_transaction_id`](PostV1BankSettlementsLinkRequestBuilder::bank_transaction_id)
    pub fn build(self) -> Result<PostV1BankSettlementsLinkRequest, BuildError> {
        Ok(PostV1BankSettlementsLinkRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            bank_transaction_id: self
                .bank_transaction_id
                .ok_or_else(|| BuildError::missing_field("bank_transaction_id"))?,
        })
    }
}
