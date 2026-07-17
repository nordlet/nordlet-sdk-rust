pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesInvoicesRegisterRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "registrationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<String>,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
}

impl PostV1PurchasesInvoicesRegisterRequest {
    pub fn builder() -> PostV1PurchasesInvoicesRegisterRequestBuilder {
        <PostV1PurchasesInvoicesRegisterRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesInvoicesRegisterRequestBuilder {
    id: Option<String>,
    registration_date: Option<String>,
    warehouse_id: Option<String>,
}

impl PostV1PurchasesInvoicesRegisterRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn registration_date(mut self, value: impl Into<String>) -> Self {
        self.registration_date = Some(value.into());
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesInvoicesRegisterRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PurchasesInvoicesRegisterRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PurchasesInvoicesRegisterRequest, BuildError> {
        Ok(PostV1PurchasesInvoicesRegisterRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            registration_date: self.registration_date,
            warehouse_id: self.warehouse_id,
        })
    }
}
