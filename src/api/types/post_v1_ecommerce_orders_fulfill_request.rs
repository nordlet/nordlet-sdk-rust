pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1EcommerceOrdersFulfillRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "cogsAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cogs_account_code: Option<String>,
    #[serde(rename = "inventoryAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_account_code: Option<String>,
}

impl PostV1EcommerceOrdersFulfillRequest {
    pub fn builder() -> PostV1EcommerceOrdersFulfillRequestBuilder {
        <PostV1EcommerceOrdersFulfillRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1EcommerceOrdersFulfillRequestBuilder {
    id: Option<String>,
    date: Option<String>,
    cogs_account_code: Option<String>,
    inventory_account_code: Option<String>,
}

impl PostV1EcommerceOrdersFulfillRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1EcommerceOrdersFulfillRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1EcommerceOrdersFulfillRequestBuilder::id)
    pub fn build(self) -> Result<PostV1EcommerceOrdersFulfillRequest, BuildError> {
        Ok(PostV1EcommerceOrdersFulfillRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            date: self.date,
            cogs_account_code: self.cogs_account_code,
            inventory_account_code: self.inventory_account_code,
        })
    }
}
