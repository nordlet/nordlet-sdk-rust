pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OwnersUpdateLedgerResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "equityAccountCode")]
    #[serde(default)]
    pub equity_account_code: String,
    #[serde(rename = "sharesQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares_quantity: Option<String>,
    #[serde(rename = "sharesAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares_amount: Option<String>,
    #[serde(rename = "sharesType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares_type: Option<String>,
    #[serde(rename = "sharesAcquisitionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares_acquisition_date: Option<NaiveDate>,
    #[serde(rename = "withholdingTaxPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withholding_tax_percent: Option<String>,
    #[serde(rename = "partnerLiability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_liability: Option<OwnersUpdateLedgerResponsePartnerLiability>,
    #[serde(rename = "specialBalanceRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_balance_required: Option<bool>,
    #[serde(rename = "supplementaryBalanceRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplementary_balance_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<OwnersUpdateLedgerResponseAddress>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl OwnersUpdateLedgerResponse {
    pub fn builder() -> OwnersUpdateLedgerResponseBuilder {
        <OwnersUpdateLedgerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OwnersUpdateLedgerResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    code: Option<String>,
    equity_account_code: Option<String>,
    shares_quantity: Option<String>,
    shares_amount: Option<String>,
    shares_type: Option<String>,
    shares_acquisition_date: Option<NaiveDate>,
    withholding_tax_percent: Option<String>,
    partner_liability: Option<OwnersUpdateLedgerResponsePartnerLiability>,
    special_balance_required: Option<bool>,
    supplementary_balance_required: Option<bool>,
    address: Option<OwnersUpdateLedgerResponseAddress>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl OwnersUpdateLedgerResponseBuilder {
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

    pub fn shares_type(mut self, value: impl Into<String>) -> Self {
        self.shares_type = Some(value.into());
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

    pub fn partner_liability(mut self, value: OwnersUpdateLedgerResponsePartnerLiability) -> Self {
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

    pub fn address(mut self, value: OwnersUpdateLedgerResponseAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OwnersUpdateLedgerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OwnersUpdateLedgerResponseBuilder::id)
    /// - [`name`](OwnersUpdateLedgerResponseBuilder::name)
    /// - [`equity_account_code`](OwnersUpdateLedgerResponseBuilder::equity_account_code)
    /// - [`created_at`](OwnersUpdateLedgerResponseBuilder::created_at)
    pub fn build(self) -> Result<OwnersUpdateLedgerResponse, BuildError> {
        Ok(OwnersUpdateLedgerResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            code: self.code,
            equity_account_code: self
                .equity_account_code
                .ok_or_else(|| BuildError::missing_field("equity_account_code"))?,
            shares_quantity: self.shares_quantity,
            shares_amount: self.shares_amount,
            shares_type: self.shares_type,
            shares_acquisition_date: self.shares_acquisition_date,
            withholding_tax_percent: self.withholding_tax_percent,
            partner_liability: self.partner_liability,
            special_balance_required: self.special_balance_required,
            supplementary_balance_required: self.supplementary_balance_required,
            address: self.address,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
