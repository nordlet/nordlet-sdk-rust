pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesGetPublicResponseBuyer {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "vatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<String>,
    #[serde(default)]
    pub address: InvoicesGetPublicResponseBuyerAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl InvoicesGetPublicResponseBuyer {
    pub fn builder() -> InvoicesGetPublicResponseBuyerBuilder {
        <InvoicesGetPublicResponseBuyerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesGetPublicResponseBuyerBuilder {
    name: Option<String>,
    vat_code: Option<String>,
    address: Option<InvoicesGetPublicResponseBuyerAddress>,
    email: Option<String>,
}

impl InvoicesGetPublicResponseBuyerBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn vat_code(mut self, value: impl Into<String>) -> Self {
        self.vat_code = Some(value.into());
        self
    }

    pub fn address(mut self, value: InvoicesGetPublicResponseBuyerAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesGetPublicResponseBuyer`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](InvoicesGetPublicResponseBuyerBuilder::name)
    /// - [`address`](InvoicesGetPublicResponseBuyerBuilder::address)
    pub fn build(self) -> Result<InvoicesGetPublicResponseBuyer, BuildError> {
        Ok(InvoicesGetPublicResponseBuyer {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            vat_code: self.vat_code,
            address: self
                .address
                .ok_or_else(|| BuildError::missing_field("address"))?,
            email: self.email,
        })
    }
}
