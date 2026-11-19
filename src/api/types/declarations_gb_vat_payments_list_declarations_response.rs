pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GbVatPaymentsListDeclarationsResponse {
    #[serde(default)]
    pub payments: Vec<GbVatPaymentsListDeclarationsResponsePaymentsItem>,
}

impl GbVatPaymentsListDeclarationsResponse {
    pub fn builder() -> GbVatPaymentsListDeclarationsResponseBuilder {
        <GbVatPaymentsListDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatPaymentsListDeclarationsResponseBuilder {
    payments: Option<Vec<GbVatPaymentsListDeclarationsResponsePaymentsItem>>,
}

impl GbVatPaymentsListDeclarationsResponseBuilder {
    pub fn payments(
        mut self,
        value: Vec<GbVatPaymentsListDeclarationsResponsePaymentsItem>,
    ) -> Self {
        self.payments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GbVatPaymentsListDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payments`](GbVatPaymentsListDeclarationsResponseBuilder::payments)
    pub fn build(self) -> Result<GbVatPaymentsListDeclarationsResponse, BuildError> {
        Ok(GbVatPaymentsListDeclarationsResponse {
            payments: self
                .payments
                .ok_or_else(|| BuildError::missing_field("payments"))?,
        })
    }
}
