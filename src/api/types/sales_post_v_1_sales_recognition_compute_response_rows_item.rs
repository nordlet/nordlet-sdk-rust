pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesRecognitionComputeResponseRowsItem {
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
    pub schedule_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub amount: String,
}

impl PostV1SalesRecognitionComputeResponseRowsItem {
    pub fn builder() -> PostV1SalesRecognitionComputeResponseRowsItemBuilder {
        <PostV1SalesRecognitionComputeResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesRecognitionComputeResponseRowsItemBuilder {
    schedule_id: Option<String>,
    invoice_id: Option<String>,
    invoice_full_number: Option<String>,
    invoice_line_id: Option<String>,
    line_description: Option<String>,
    schedule_date: Option<String>,
    description: Option<String>,
    amount: Option<String>,
}

impl PostV1SalesRecognitionComputeResponseRowsItemBuilder {
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

    pub fn schedule_date(mut self, value: impl Into<String>) -> Self {
        self.schedule_date = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1SalesRecognitionComputeResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`schedule_id`](PostV1SalesRecognitionComputeResponseRowsItemBuilder::schedule_id)
    /// - [`invoice_id`](PostV1SalesRecognitionComputeResponseRowsItemBuilder::invoice_id)
    /// - [`invoice_line_id`](PostV1SalesRecognitionComputeResponseRowsItemBuilder::invoice_line_id)
    /// - [`line_description`](PostV1SalesRecognitionComputeResponseRowsItemBuilder::line_description)
    /// - [`amount`](PostV1SalesRecognitionComputeResponseRowsItemBuilder::amount)
    pub fn build(self) -> Result<PostV1SalesRecognitionComputeResponseRowsItem, BuildError> {
        Ok(PostV1SalesRecognitionComputeResponseRowsItem {
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
