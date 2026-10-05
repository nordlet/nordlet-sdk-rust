pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuSmeThresholdGetDeclarationsResponseTurnover {
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub documents: i64,
}

impl EuSmeThresholdGetDeclarationsResponseTurnover {
    pub fn builder() -> EuSmeThresholdGetDeclarationsResponseTurnoverBuilder {
        <EuSmeThresholdGetDeclarationsResponseTurnoverBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuSmeThresholdGetDeclarationsResponseTurnoverBuilder {
    amount: Option<String>,
    currency: Option<String>,
    documents: Option<i64>,
}

impl EuSmeThresholdGetDeclarationsResponseTurnoverBuilder {
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

    /// Consumes the builder and constructs a [`EuSmeThresholdGetDeclarationsResponseTurnover`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](EuSmeThresholdGetDeclarationsResponseTurnoverBuilder::amount)
    /// - [`currency`](EuSmeThresholdGetDeclarationsResponseTurnoverBuilder::currency)
    /// - [`documents`](EuSmeThresholdGetDeclarationsResponseTurnoverBuilder::documents)
    pub fn build(self) -> Result<EuSmeThresholdGetDeclarationsResponseTurnover, BuildError> {
        Ok(EuSmeThresholdGetDeclarationsResponseTurnover {
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
