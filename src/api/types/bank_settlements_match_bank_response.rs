pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SettlementsMatchBankResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "externalId")]
    #[serde(default)]
    pub external_id: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(default)]
    pub gross: String,
    #[serde(default)]
    pub fee: String,
    #[serde(default)]
    pub net: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "sourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(rename = "chargeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_id: Option<String>,
    #[serde(rename = "commissionPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_percent: Option<String>,
    #[serde(rename = "commissionAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(rename = "matchedInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_invoice_id: Option<String>,
    #[serde(rename = "matchStatus")]
    pub match_status: SettlementsMatchBankResponseMatchStatus,
    #[serde(rename = "clearingBankAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearing_bank_account_id: Option<String>,
    #[serde(rename = "clearingBooked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearing_booked: Option<String>,
    #[serde(rename = "clearingDifference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearing_difference: Option<String>,
    #[serde(rename = "clearingUnposted")]
    #[serde(default)]
    pub clearing_unposted: bool,
}

impl SettlementsMatchBankResponse {
    pub fn builder() -> SettlementsMatchBankResponseBuilder {
        <SettlementsMatchBankResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettlementsMatchBankResponseBuilder {
    id: Option<String>,
    external_id: Option<String>,
    category: Option<String>,
    date: Option<NaiveDate>,
    gross: Option<String>,
    fee: Option<String>,
    net: Option<String>,
    description: Option<String>,
    source_id: Option<String>,
    charge_id: Option<String>,
    commission_percent: Option<String>,
    commission_amount: Option<String>,
    reference: Option<String>,
    matched_invoice_id: Option<String>,
    match_status: Option<SettlementsMatchBankResponseMatchStatus>,
    clearing_bank_account_id: Option<String>,
    clearing_booked: Option<String>,
    clearing_difference: Option<String>,
    clearing_unposted: Option<bool>,
}

impl SettlementsMatchBankResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn gross(mut self, value: impl Into<String>) -> Self {
        self.gross = Some(value.into());
        self
    }

    pub fn fee(mut self, value: impl Into<String>) -> Self {
        self.fee = Some(value.into());
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn source_id(mut self, value: impl Into<String>) -> Self {
        self.source_id = Some(value.into());
        self
    }

    pub fn charge_id(mut self, value: impl Into<String>) -> Self {
        self.charge_id = Some(value.into());
        self
    }

    pub fn commission_percent(mut self, value: impl Into<String>) -> Self {
        self.commission_percent = Some(value.into());
        self
    }

    pub fn commission_amount(mut self, value: impl Into<String>) -> Self {
        self.commission_amount = Some(value.into());
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn matched_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.matched_invoice_id = Some(value.into());
        self
    }

    pub fn match_status(mut self, value: SettlementsMatchBankResponseMatchStatus) -> Self {
        self.match_status = Some(value);
        self
    }

    pub fn clearing_bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.clearing_bank_account_id = Some(value.into());
        self
    }

    pub fn clearing_booked(mut self, value: impl Into<String>) -> Self {
        self.clearing_booked = Some(value.into());
        self
    }

    pub fn clearing_difference(mut self, value: impl Into<String>) -> Self {
        self.clearing_difference = Some(value.into());
        self
    }

    pub fn clearing_unposted(mut self, value: bool) -> Self {
        self.clearing_unposted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SettlementsMatchBankResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SettlementsMatchBankResponseBuilder::id)
    /// - [`external_id`](SettlementsMatchBankResponseBuilder::external_id)
    /// - [`category`](SettlementsMatchBankResponseBuilder::category)
    /// - [`date`](SettlementsMatchBankResponseBuilder::date)
    /// - [`gross`](SettlementsMatchBankResponseBuilder::gross)
    /// - [`fee`](SettlementsMatchBankResponseBuilder::fee)
    /// - [`net`](SettlementsMatchBankResponseBuilder::net)
    /// - [`match_status`](SettlementsMatchBankResponseBuilder::match_status)
    /// - [`clearing_unposted`](SettlementsMatchBankResponseBuilder::clearing_unposted)
    pub fn build(self) -> Result<SettlementsMatchBankResponse, BuildError> {
        Ok(SettlementsMatchBankResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            external_id: self
                .external_id
                .ok_or_else(|| BuildError::missing_field("external_id"))?,
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            gross: self
                .gross
                .ok_or_else(|| BuildError::missing_field("gross"))?,
            fee: self.fee.ok_or_else(|| BuildError::missing_field("fee"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            description: self.description,
            source_id: self.source_id,
            charge_id: self.charge_id,
            commission_percent: self.commission_percent,
            commission_amount: self.commission_amount,
            reference: self.reference,
            matched_invoice_id: self.matched_invoice_id,
            match_status: self
                .match_status
                .ok_or_else(|| BuildError::missing_field("match_status"))?,
            clearing_bank_account_id: self.clearing_bank_account_id,
            clearing_booked: self.clearing_booked,
            clearing_difference: self.clearing_difference,
            clearing_unposted: self
                .clearing_unposted
                .ok_or_else(|| BuildError::missing_field("clearing_unposted"))?,
        })
    }
}
