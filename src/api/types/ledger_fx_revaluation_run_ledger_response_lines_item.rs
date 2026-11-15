pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FxRevaluationRunLedgerResponseLinesItem {
    pub kind: FxRevaluationRunLedgerResponseLinesItemKind,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(rename = "accountCode")]
    #[serde(default)]
    pub account_code: String,
    /// Unrealised gain (positive) or loss (negative) of the line
    #[serde(default)]
    pub amount: String,
}

impl FxRevaluationRunLedgerResponseLinesItem {
    pub fn builder() -> FxRevaluationRunLedgerResponseLinesItemBuilder {
        <FxRevaluationRunLedgerResponseLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FxRevaluationRunLedgerResponseLinesItemBuilder {
    kind: Option<FxRevaluationRunLedgerResponseLinesItemKind>,
    currency: Option<String>,
    partner_id: Option<String>,
    account_code: Option<String>,
    amount: Option<String>,
}

impl FxRevaluationRunLedgerResponseLinesItemBuilder {
    pub fn kind(mut self, value: FxRevaluationRunLedgerResponseLinesItemKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FxRevaluationRunLedgerResponseLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](FxRevaluationRunLedgerResponseLinesItemBuilder::kind)
    /// - [`currency`](FxRevaluationRunLedgerResponseLinesItemBuilder::currency)
    /// - [`account_code`](FxRevaluationRunLedgerResponseLinesItemBuilder::account_code)
    /// - [`amount`](FxRevaluationRunLedgerResponseLinesItemBuilder::amount)
    pub fn build(self) -> Result<FxRevaluationRunLedgerResponseLinesItem, BuildError> {
        Ok(FxRevaluationRunLedgerResponseLinesItem {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            partner_id: self.partner_id,
            account_code: self
                .account_code
                .ok_or_else(|| BuildError::missing_field("account_code"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
