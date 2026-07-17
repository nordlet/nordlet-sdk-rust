pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerOwnersCreateRequest {
    #[serde(default)]
    pub name: String,
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
    pub shares_type: Option<PostV1LedgerOwnersCreateRequestSharesType>,
    #[serde(rename = "sharesAcquisitionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares_acquisition_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostV1LedgerOwnersCreateRequestAddress>,
}

impl PostV1LedgerOwnersCreateRequest {
    pub fn builder() -> PostV1LedgerOwnersCreateRequestBuilder {
        <PostV1LedgerOwnersCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerOwnersCreateRequestBuilder {
    name: Option<String>,
    code: Option<String>,
    equity_account_code: Option<String>,
    shares_quantity: Option<String>,
    shares_amount: Option<String>,
    shares_type: Option<PostV1LedgerOwnersCreateRequestSharesType>,
    shares_acquisition_date: Option<String>,
    address: Option<PostV1LedgerOwnersCreateRequestAddress>,
}

impl PostV1LedgerOwnersCreateRequestBuilder {
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

    pub fn shares_type(mut self, value: PostV1LedgerOwnersCreateRequestSharesType) -> Self {
        self.shares_type = Some(value);
        self
    }

    pub fn shares_acquisition_date(mut self, value: impl Into<String>) -> Self {
        self.shares_acquisition_date = Some(value.into());
        self
    }

    pub fn address(mut self, value: PostV1LedgerOwnersCreateRequestAddress) -> Self {
        self.address = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerOwnersCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1LedgerOwnersCreateRequestBuilder::name)
    pub fn build(self) -> Result<PostV1LedgerOwnersCreateRequest, BuildError> {
        Ok(PostV1LedgerOwnersCreateRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
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
