pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecognitionComputeSalesResponseRowsItem {
    #[serde(rename = "scheduleId")]
    #[serde(default)]
    pub schedule_id: String,
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    #[serde(rename = "invoiceFullNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_full_number: Option<String>,
    #[serde(rename = "invoiceLineId")]
    #[serde(default)]
    pub invoice_line_id: String,
    #[serde(rename = "lineDescription")]
    #[serde(default)]
    pub line_description: String,
    #[serde(rename = "scheduleDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub amount: String,
}

impl RecognitionComputeSalesResponseRowsItem {
    pub fn builder() -> RecognitionComputeSalesResponseRowsItemBuilder {
        <RecognitionComputeSalesResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecognitionComputeSalesResponseRowsItemBuilder {
    schedule_id: Option<String>,
    invoice_id: Option<String>,
    invoice_full_number: Option<String>,
    invoice_line_id: Option<String>,
    line_description: Option<String>,
    schedule_date: Option<NaiveDate>,
    description: Option<String>,
    amount: Option<String>,
}

impl RecognitionComputeSalesResponseRowsItemBuilder {
    pub fn schedule_id(mut self, value: impl Into<String>) -> Self {
        self.schedule_id = Some(value.into());
        self
    }

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn invoice_full_number(mut self, value: impl Into<String>) -> Self {
        self.invoice_full_number = Some(value.into());
        self
    }

    pub fn invoice_line_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_line_id = Some(value.into());
        self
    }

    pub fn line_description(mut self, value: impl Into<String>) -> Self {
        self.line_description = Some(value.into());
        self
    }

    pub fn schedule_date(mut self, value: NaiveDate) -> Self {
        self.schedule_date = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RecognitionComputeSalesResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`schedule_id`](RecognitionComputeSalesResponseRowsItemBuilder::schedule_id)
    /// - [`invoice_id`](RecognitionComputeSalesResponseRowsItemBuilder::invoice_id)
    /// - [`invoice_line_id`](RecognitionComputeSalesResponseRowsItemBuilder::invoice_line_id)
    /// - [`line_description`](RecognitionComputeSalesResponseRowsItemBuilder::line_description)
    /// - [`amount`](RecognitionComputeSalesResponseRowsItemBuilder::amount)
    pub fn build(self) -> Result<RecognitionComputeSalesResponseRowsItem, BuildError> {
        Ok(RecognitionComputeSalesResponseRowsItem {
            schedule_id: self
                .schedule_id
                .ok_or_else(|| BuildError::missing_field("schedule_id"))?,
            invoice_id: self
                .invoice_id
                .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
            invoice_full_number: self.invoice_full_number,
            invoice_line_id: self
                .invoice_line_id
                .ok_or_else(|| BuildError::missing_field("invoice_line_id"))?,
            line_description: self
                .line_description
                .ok_or_else(|| BuildError::missing_field("line_description"))?,
            schedule_date: self.schedule_date,
            description: self.description,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
