pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RefundsCreatePosRequest {
    #[serde(rename = "shiftId")]
    #[serde(default)]
    pub shift_id: String,
    #[serde(rename = "receiptId")]
    #[serde(default)]
    pub receipt_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
    #[serde(default)]
    pub lines: Vec<RefundsCreatePosRequestLinesItem>,
}

impl RefundsCreatePosRequest {
    pub fn builder() -> RefundsCreatePosRequestBuilder {
        <RefundsCreatePosRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundsCreatePosRequestBuilder {
    shift_id: Option<String>,
    receipt_id: Option<String>,
    date: Option<NaiveDate>,
    lines: Option<Vec<RefundsCreatePosRequestLinesItem>>,
}

impl RefundsCreatePosRequestBuilder {
    pub fn shift_id(mut self, value: impl Into<String>) -> Self {
        self.shift_id = Some(value.into());
        self
    }

    pub fn receipt_id(mut self, value: impl Into<String>) -> Self {
        self.receipt_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn lines(mut self, value: Vec<RefundsCreatePosRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefundsCreatePosRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`shift_id`](RefundsCreatePosRequestBuilder::shift_id)
    /// - [`receipt_id`](RefundsCreatePosRequestBuilder::receipt_id)
    /// - [`lines`](RefundsCreatePosRequestBuilder::lines)
    pub fn build(self) -> Result<RefundsCreatePosRequest, BuildError> {
        Ok(RefundsCreatePosRequest {
            shift_id: self
                .shift_id
                .ok_or_else(|| BuildError::missing_field("shift_id"))?,
            receipt_id: self
                .receipt_id
                .ok_or_else(|| BuildError::missing_field("receipt_id"))?,
            date: self.date,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
