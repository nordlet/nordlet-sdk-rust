pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OwnersCreateLedgerRequest {
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
    pub shares_type: Option<OwnersCreateLedgerRequestSharesType>,
    #[serde(rename = "sharesAcquisitionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares_acquisition_date: Option<NaiveDate>,
    #[serde(rename = "withholdingTaxPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withholding_tax_percent: Option<String>,
    #[serde(rename = "partnerLiability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_liability: Option<OwnersCreateLedgerRequestPartnerLiability>,
    #[serde(rename = "specialBalanceRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_balance_required: Option<bool>,
    #[serde(rename = "supplementaryBalanceRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplementary_balance_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<OwnersCreateLedgerRequestAddress>,
}

impl OwnersCreateLedgerRequest {
    pub fn builder() -> OwnersCreateLedgerRequestBuilder {
        <OwnersCreateLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OwnersCreateLedgerRequestBuilder {
    name: Option<String>,
    code: Option<String>,
    equity_account_code: Option<String>,
    shares_quantity: Option<String>,
    shares_amount: Option<String>,
    shares_type: Option<OwnersCreateLedgerRequestSharesType>,
    shares_acquisition_date: Option<NaiveDate>,
    withholding_tax_percent: Option<String>,
    partner_liability: Option<OwnersCreateLedgerRequestPartnerLiability>,
    special_balance_required: Option<bool>,
    supplementary_balance_required: Option<bool>,
    address: Option<OwnersCreateLedgerRequestAddress>,
}

impl OwnersCreateLedgerRequestBuilder {
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

    pub fn shares_type(mut self, value: OwnersCreateLedgerRequestSharesType) -> Self {
        self.shares_type = Some(value);
        self
    }

    pub fn shares_acquisition_date(mut self, value: NaiveDate) -> Self {
        self.shares_acquisition_date = Some(value);
        self
    }

    pub fn withholding_tax_percent(mut self, value: impl Into<String>) -> Self {
        self.withholding_tax_percent = Some(value.into());
        self
    }

    pub fn partner_liability(mut self, value: OwnersCreateLedgerRequestPartnerLiability) -> Self {
        self.partner_liability = Some(value);
        self
    }

    pub fn special_balance_required(mut self, value: bool) -> Self {
        self.special_balance_required = Some(value);
        self
    }

    pub fn supplementary_balance_required(mut self, value: bool) -> Self {
        self.supplementary_balance_required = Some(value);
        self
    }

    pub fn address(mut self, value: OwnersCreateLedgerRequestAddress) -> Self {
        self.address = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OwnersCreateLedgerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](OwnersCreateLedgerRequestBuilder::name)
    pub fn build(self) -> Result<OwnersCreateLedgerRequest, BuildError> {
        Ok(OwnersCreateLedgerRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            code: self.code,
            equity_account_code: self.equity_account_code,
            shares_quantity: self.shares_quantity,
            shares_amount: self.shares_amount,
            shares_type: self.shares_type,
            shares_acquisition_date: self.shares_acquisition_date,
            withholding_tax_percent: self.withholding_tax_percent,
            partner_liability: self.partner_liability,
            special_balance_required: self.special_balance_required,
            supplementary_balance_required: self.supplementary_balance_required,
            address: self.address,
        })
    }
}
