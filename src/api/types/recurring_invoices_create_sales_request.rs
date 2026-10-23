pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecurringInvoicesCreateSalesRequest {
    #[serde(rename = "templateId")]
    #[serde(default)]
    pub template_id: String,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<RecurringInvoicesCreateSalesRequestFrequency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[serde(rename = "startDate")]
    #[serde(default)]
    pub start_date: NaiveDate,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<RecurringInvoicesCreateSalesRequestLocale>,
    #[serde(rename = "autoIssue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_issue: Option<bool>,
    #[serde(rename = "autoSend")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_send: Option<bool>,
    #[serde(rename = "sendTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_to: Option<String>,
}

impl RecurringInvoicesCreateSalesRequest {
    pub fn builder() -> RecurringInvoicesCreateSalesRequestBuilder {
        <RecurringInvoicesCreateSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecurringInvoicesCreateSalesRequestBuilder {
    template_id: Option<String>,
    partner_id: Option<String>,
    frequency: Option<RecurringInvoicesCreateSalesRequestFrequency>,
    interval: Option<i64>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    locale: Option<RecurringInvoicesCreateSalesRequestLocale>,
    auto_issue: Option<bool>,
    auto_send: Option<bool>,
    send_to: Option<String>,
}

impl RecurringInvoicesCreateSalesRequestBuilder {
    pub fn template_id(mut self, value: impl Into<String>) -> Self {
        self.template_id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn frequency(mut self, value: RecurringInvoicesCreateSalesRequestFrequency) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn interval(mut self, value: i64) -> Self {
        self.interval = Some(value);
        self
    }

    pub fn start_date(mut self, value: NaiveDate) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn end_date(mut self, value: NaiveDate) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn locale(mut self, value: RecurringInvoicesCreateSalesRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    pub fn auto_issue(mut self, value: bool) -> Self {
        self.auto_issue = Some(value);
        self
    }

    pub fn auto_send(mut self, value: bool) -> Self {
        self.auto_send = Some(value);
        self
    }

    pub fn send_to(mut self, value: impl Into<String>) -> Self {
        self.send_to = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RecurringInvoicesCreateSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`template_id`](RecurringInvoicesCreateSalesRequestBuilder::template_id)
    /// - [`start_date`](RecurringInvoicesCreateSalesRequestBuilder::start_date)
    pub fn build(self) -> Result<RecurringInvoicesCreateSalesRequest, BuildError> {
        Ok(RecurringInvoicesCreateSalesRequest {
            template_id: self
                .template_id
                .ok_or_else(|| BuildError::missing_field("template_id"))?,
            partner_id: self.partner_id,
            frequency: self.frequency,
            interval: self.interval,
            start_date: self
                .start_date
                .ok_or_else(|| BuildError::missing_field("start_date"))?,
            end_date: self.end_date,
            locale: self.locale,
            auto_issue: self.auto_issue,
            auto_send: self.auto_send,
            send_to: self.send_to,
        })
    }
}
