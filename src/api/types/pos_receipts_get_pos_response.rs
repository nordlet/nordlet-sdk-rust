pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceiptsGetPosResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "shiftId")]
    #[serde(default)]
    pub shift_id: String,
    #[serde(default)]
    pub number: i64,
    #[serde(rename = "netTotal")]
    #[serde(default)]
    pub net_total: String,
    #[serde(rename = "vatTotal")]
    #[serde(default)]
    pub vat_total: String,
    #[serde(rename = "grossTotal")]
    #[serde(default)]
    pub gross_total: String,
    #[serde(rename = "cashAmount")]
    #[serde(default)]
    pub cash_amount: String,
    #[serde(rename = "cardAmount")]
    #[serde(default)]
    pub card_amount: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub lines: Vec<ReceiptsGetPosResponseLinesItem>,
}

impl ReceiptsGetPosResponse {
    pub fn builder() -> ReceiptsGetPosResponseBuilder {
        <ReceiptsGetPosResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptsGetPosResponseBuilder {
    id: Option<String>,
    shift_id: Option<String>,
    number: Option<i64>,
    net_total: Option<String>,
    vat_total: Option<String>,
    gross_total: Option<String>,
    cash_amount: Option<String>,
    card_amount: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    lines: Option<Vec<ReceiptsGetPosResponseLinesItem>>,
}

impl ReceiptsGetPosResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn shift_id(mut self, value: impl Into<String>) -> Self {
        self.shift_id = Some(value.into());
        self
    }

    pub fn number(mut self, value: i64) -> Self {
        self.number = Some(value);
        self
    }

    pub fn net_total(mut self, value: impl Into<String>) -> Self {
        self.net_total = Some(value.into());
        self
    }

    pub fn vat_total(mut self, value: impl Into<String>) -> Self {
        self.vat_total = Some(value.into());
        self
    }

    pub fn gross_total(mut self, value: impl Into<String>) -> Self {
        self.gross_total = Some(value.into());
        self
    }

    pub fn cash_amount(mut self, value: impl Into<String>) -> Self {
        self.cash_amount = Some(value.into());
        self
    }

    pub fn card_amount(mut self, value: impl Into<String>) -> Self {
        self.card_amount = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn lines(mut self, value: Vec<ReceiptsGetPosResponseLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceiptsGetPosResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReceiptsGetPosResponseBuilder::id)
    /// - [`shift_id`](ReceiptsGetPosResponseBuilder::shift_id)
    /// - [`number`](ReceiptsGetPosResponseBuilder::number)
    /// - [`net_total`](ReceiptsGetPosResponseBuilder::net_total)
    /// - [`vat_total`](ReceiptsGetPosResponseBuilder::vat_total)
    /// - [`gross_total`](ReceiptsGetPosResponseBuilder::gross_total)
    /// - [`cash_amount`](ReceiptsGetPosResponseBuilder::cash_amount)
    /// - [`card_amount`](ReceiptsGetPosResponseBuilder::card_amount)
    /// - [`created_at`](ReceiptsGetPosResponseBuilder::created_at)
    /// - [`lines`](ReceiptsGetPosResponseBuilder::lines)
    pub fn build(self) -> Result<ReceiptsGetPosResponse, BuildError> {
        Ok(ReceiptsGetPosResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            shift_id: self
                .shift_id
                .ok_or_else(|| BuildError::missing_field("shift_id"))?,
            number: self
                .number
                .ok_or_else(|| BuildError::missing_field("number"))?,
            net_total: self
                .net_total
                .ok_or_else(|| BuildError::missing_field("net_total"))?,
            vat_total: self
                .vat_total
                .ok_or_else(|| BuildError::missing_field("vat_total"))?,
            gross_total: self
                .gross_total
                .ok_or_else(|| BuildError::missing_field("gross_total"))?,
            cash_amount: self
                .cash_amount
                .ok_or_else(|| BuildError::missing_field("cash_amount"))?,
            card_amount: self
                .card_amount
                .ok_or_else(|| BuildError::missing_field("card_amount"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
