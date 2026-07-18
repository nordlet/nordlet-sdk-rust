pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuSmeThresholdGetResponseTurnover {
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub documents: i64,
}

impl PostV1DeclarationsEuSmeThresholdGetResponseTurnover {
    pub fn builder() -> PostV1DeclarationsEuSmeThresholdGetResponseTurnoverBuilder {
        <PostV1DeclarationsEuSmeThresholdGetResponseTurnoverBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuSmeThresholdGetResponseTurnoverBuilder {
    amount: Option<String>,
    currency: Option<String>,
    documents: Option<i64>,
}

impl PostV1DeclarationsEuSmeThresholdGetResponseTurnoverBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn documents(mut self, value: i64) -> Self {
        self.documents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuSmeThresholdGetResponseTurnover`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](PostV1DeclarationsEuSmeThresholdGetResponseTurnoverBuilder::amount)
    /// - [`currency`](PostV1DeclarationsEuSmeThresholdGetResponseTurnoverBuilder::currency)
    /// - [`documents`](PostV1DeclarationsEuSmeThresholdGetResponseTurnoverBuilder::documents)
    pub fn build(self) -> Result<PostV1DeclarationsEuSmeThresholdGetResponseTurnover, BuildError> {
        Ok(PostV1DeclarationsEuSmeThresholdGetResponseTurnover {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            documents: self
                .documents
                .ok_or_else(|| BuildError::missing_field("documents"))?,
        })
    }
}
