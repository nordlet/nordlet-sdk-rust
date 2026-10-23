pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecurringInvoicesUpdateSalesRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "templateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<RecurringInvoicesUpdateSalesRequestFrequency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
    #[serde(rename = "nextRunDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_run_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RecurringInvoicesUpdateSalesRequestStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<RecurringInvoicesUpdateSalesRequestLocale>,
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

impl RecurringInvoicesUpdateSalesRequest {
    pub fn builder() -> RecurringInvoicesUpdateSalesRequestBuilder {
        <RecurringInvoicesUpdateSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecurringInvoicesUpdateSalesRequestBuilder {
    id: Option<String>,
    template_id: Option<String>,
    partner_id: Option<String>,
    frequency: Option<RecurringInvoicesUpdateSalesRequestFrequency>,
    interval: Option<i64>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    next_run_date: Option<NaiveDate>,
    status: Option<RecurringInvoicesUpdateSalesRequestStatus>,
    locale: Option<RecurringInvoicesUpdateSalesRequestLocale>,
    auto_issue: Option<bool>,
    auto_send: Option<bool>,
    send_to: Option<String>,
}

impl RecurringInvoicesUpdateSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn template_id(mut self, value: impl Into<String>) -> Self {
        self.template_id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn frequency(mut self, value: RecurringInvoicesUpdateSalesRequestFrequency) -> Self {
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

    pub fn next_run_date(mut self, value: NaiveDate) -> Self {
        self.next_run_date = Some(value);
        self
    }

    pub fn status(mut self, value: RecurringInvoicesUpdateSalesRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn locale(mut self, value: RecurringInvoicesUpdateSalesRequestLocale) -> Self {
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

    /// Consumes the builder and constructs a [`RecurringInvoicesUpdateSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RecurringInvoicesUpdateSalesRequestBuilder::id)
    pub fn build(self) -> Result<RecurringInvoicesUpdateSalesRequest, BuildError> {
        Ok(RecurringInvoicesUpdateSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            template_id: self.template_id,
            partner_id: self.partner_id,
            frequency: self.frequency,
            interval: self.interval,
            start_date: self.start_date,
            end_date: self.end_date,
            next_run_date: self.next_run_date,
            status: self.status,
            locale: self.locale,
            auto_issue: self.auto_issue,
            auto_send: self.auto_send,
            send_to: self.send_to,
        })
    }
}
