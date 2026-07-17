pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerOwnersUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "equityAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equity_account_code: Option<String>,
    #[serde(rename = "sharesQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares_quantity: Option<String>,
    #[serde(rename = "sharesAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares_amount: Option<String>,
    #[serde(rename = "sharesType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares_type: Option<PostV1LedgerOwnersUpdateRequestSharesType>,
    #[serde(rename = "sharesAcquisitionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares_acquisition_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostV1LedgerOwnersUpdateRequestAddress>,
}

impl PostV1LedgerOwnersUpdateRequest {
    pub fn builder() -> PostV1LedgerOwnersUpdateRequestBuilder {
        <PostV1LedgerOwnersUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerOwnersUpdateRequestBuilder {
    id: Option<String>,
    name: Option<String>,
    code: Option<String>,
    equity_account_code: Option<String>,
    shares_quantity: Option<String>,
    shares_amount: Option<String>,
    shares_type: Option<PostV1LedgerOwnersUpdateRequestSharesType>,
    shares_acquisition_date: Option<String>,
    address: Option<PostV1LedgerOwnersUpdateRequestAddress>,
}

impl PostV1LedgerOwnersUpdateRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn equity_account_code(mut self, value: impl Into<String>) -> Self {
        self.equity_account_code = Some(value.into());
        self
    }

    pub fn shares_quantity(mut self, value: impl Into<String>) -> Self {
        self.shares_quantity = Some(value.into());
        self
    }

    pub fn shares_amount(mut self, value: impl Into<String>) -> Self {
        self.shares_amount = Some(value.into());
        self
    }

    pub fn shares_type(mut self, value: PostV1LedgerOwnersUpdateRequestSharesType) -> Self {
        self.shares_type = Some(value);
        self
    }

    pub fn shares_acquisition_date(mut self, value: impl Into<String>) -> Self {
        self.shares_acquisition_date = Some(value.into());
        self
    }

    pub fn address(mut self, value: PostV1LedgerOwnersUpdateRequestAddress) -> Self {
        self.address = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerOwnersUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LedgerOwnersUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1LedgerOwnersUpdateRequest, BuildError> {
        Ok(PostV1LedgerOwnersUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            code: self.code,
            equity_account_code: self.equity_account_code,
            shares_quantity: self.shares_quantity,
            shares_amount: self.shares_amount,
            shares_type: self.shares_type,
            shares_acquisition_date: self.shares_acquisition_date,
            address: self.address,
        })
    }
}
