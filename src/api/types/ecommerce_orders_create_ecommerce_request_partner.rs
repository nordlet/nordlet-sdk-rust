pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersCreateEcommerceRequestPartner {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl OrdersCreateEcommerceRequestPartner {
    pub fn builder() -> OrdersCreateEcommerceRequestPartnerBuilder {
        <OrdersCreateEcommerceRequestPartnerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersCreateEcommerceRequestPartnerBuilder {
    name: Option<String>,
    email: Option<String>,
    code: Option<String>,
}

impl OrdersCreateEcommerceRequestPartnerBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrdersCreateEcommerceRequestPartner`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](OrdersCreateEcommerceRequestPartnerBuilder::name)
    pub fn build(self) -> Result<OrdersCreateEcommerceRequestPartner, BuildError> {
        Ok(OrdersCreateEcommerceRequestPartner {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            email: self.email,
            code: self.code,
        })
    }
}
