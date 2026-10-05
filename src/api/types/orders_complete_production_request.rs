pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersCompleteProductionRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "scrappedQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrapped_quantity: Option<String>,
    #[serde(rename = "componentsAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components_account_code: Option<String>,
    #[serde(rename = "finishedAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_account_code: Option<String>,
}

impl OrdersCompleteProductionRequest {
    pub fn builder() -> OrdersCompleteProductionRequestBuilder {
        <OrdersCompleteProductionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersCompleteProductionRequestBuilder {
    id: Option<String>,
    scrapped_quantity: Option<String>,
    components_account_code: Option<String>,
    finished_account_code: Option<String>,
}

impl OrdersCompleteProductionRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn scrapped_quantity(mut self, value: impl Into<String>) -> Self {
        self.scrapped_quantity = Some(value.into());
        self
    }

    pub fn components_account_code(mut self, value: impl Into<String>) -> Self {
        self.components_account_code = Some(value.into());
        self
    }

    pub fn finished_account_code(mut self, value: impl Into<String>) -> Self {
        self.finished_account_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrdersCompleteProductionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OrdersCompleteProductionRequestBuilder::id)
    pub fn build(self) -> Result<OrdersCompleteProductionRequest, BuildError> {
        Ok(OrdersCompleteProductionRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            scrapped_quantity: self.scrapped_quantity,
            components_account_code: self.components_account_code,
            finished_account_code: self.finished_account_code,
        })
    }
}
