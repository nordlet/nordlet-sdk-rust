pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersFulfillEcommerceRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
    #[serde(rename = "cogsAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cogs_account_code: Option<String>,
    #[serde(rename = "inventoryAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_account_code: Option<String>,
}

impl OrdersFulfillEcommerceRequest {
    pub fn builder() -> OrdersFulfillEcommerceRequestBuilder {
        <OrdersFulfillEcommerceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersFulfillEcommerceRequestBuilder {
    id: Option<String>,
    date: Option<NaiveDate>,
    cogs_account_code: Option<String>,
    inventory_account_code: Option<String>,
}

impl OrdersFulfillEcommerceRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn cogs_account_code(mut self, value: impl Into<String>) -> Self {
        self.cogs_account_code = Some(value.into());
        self
    }

    pub fn inventory_account_code(mut self, value: impl Into<String>) -> Self {
        self.inventory_account_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrdersFulfillEcommerceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OrdersFulfillEcommerceRequestBuilder::id)
    pub fn build(self) -> Result<OrdersFulfillEcommerceRequest, BuildError> {
        Ok(OrdersFulfillEcommerceRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            date: self.date,
            cogs_account_code: self.cogs_account_code,
            inventory_account_code: self.inventory_account_code,
        })
    }
}
