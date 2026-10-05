pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesRegisterPurchasesRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "registrationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<NaiveDate>,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
}

impl InvoicesRegisterPurchasesRequest {
    pub fn builder() -> InvoicesRegisterPurchasesRequestBuilder {
        <InvoicesRegisterPurchasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesRegisterPurchasesRequestBuilder {
    id: Option<String>,
    registration_date: Option<NaiveDate>,
    warehouse_id: Option<String>,
}

impl InvoicesRegisterPurchasesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn registration_date(mut self, value: NaiveDate) -> Self {
        self.registration_date = Some(value);
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesRegisterPurchasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesRegisterPurchasesRequestBuilder::id)
    pub fn build(self) -> Result<InvoicesRegisterPurchasesRequest, BuildError> {
        Ok(InvoicesRegisterPurchasesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            registration_date: self.registration_date,
            warehouse_id: self.warehouse_id,
        })
    }
}
