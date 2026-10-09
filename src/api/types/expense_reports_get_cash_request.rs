pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExpenseReportsGetCashRequest {
    #[serde(default)]
    pub id: String,
}

impl ExpenseReportsGetCashRequest {
    pub fn builder() -> ExpenseReportsGetCashRequestBuilder {
        <ExpenseReportsGetCashRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExpenseReportsGetCashRequestBuilder {
    id: Option<String>,
}

impl ExpenseReportsGetCashRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExpenseReportsGetCashRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ExpenseReportsGetCashRequestBuilder::id)
    pub fn build(self) -> Result<ExpenseReportsGetCashRequest, BuildError> {
        Ok(ExpenseReportsGetCashRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
