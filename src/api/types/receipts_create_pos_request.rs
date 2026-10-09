pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceiptsCreatePosRequest {
    #[serde(rename = "shiftId")]
    #[serde(default)]
    pub shift_id: String,
    #[serde(default)]
    pub lines: Vec<ReceiptsCreatePosRequestLinesItem>,
    #[serde(rename = "cashAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_amount: Option<String>,
    #[serde(rename = "cardAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_amount: Option<String>,
}

impl ReceiptsCreatePosRequest {
    pub fn builder() -> ReceiptsCreatePosRequestBuilder {
        <ReceiptsCreatePosRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptsCreatePosRequestBuilder {
    shift_id: Option<String>,
    lines: Option<Vec<ReceiptsCreatePosRequestLinesItem>>,
    cash_amount: Option<String>,
    card_amount: Option<String>,
}

impl ReceiptsCreatePosRequestBuilder {
    pub fn shift_id(mut self, value: impl Into<String>) -> Self {
        self.shift_id = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<ReceiptsCreatePosRequestLinesItem>) -> Self {
        self.lines = Some(value);
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

    /// Consumes the builder and constructs a [`ReceiptsCreatePosRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`shift_id`](ReceiptsCreatePosRequestBuilder::shift_id)
    /// - [`lines`](ReceiptsCreatePosRequestBuilder::lines)
    pub fn build(self) -> Result<ReceiptsCreatePosRequest, BuildError> {
        Ok(ReceiptsCreatePosRequest {
            shift_id: self
                .shift_id
                .ok_or_else(|| BuildError::missing_field("shift_id"))?,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
            cash_amount: self.cash_amount,
            card_amount: self.card_amount,
        })
    }
}
