pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentTypesListPayrollRequest {}

impl PaymentTypesListPayrollRequest {
    pub fn builder() -> PaymentTypesListPayrollRequestBuilder {
        <PaymentTypesListPayrollRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentTypesListPayrollRequestBuilder {}

impl PaymentTypesListPayrollRequestBuilder {
    /// Consumes the builder and constructs a [`PaymentTypesListPayrollRequest`].
    pub fn build(self) -> Result<PaymentTypesListPayrollRequest, BuildError> {
        Ok(PaymentTypesListPayrollRequest {})
    }
}
