pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RecurringInvoicesCreateSalesResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "templateId")]
    #[serde(default)]
    pub template_id: String,
    #[serde(rename = "templateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "partnerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_name: Option<String>,
    pub frequency: RecurringInvoicesCreateSalesResponseFrequency,
    #[serde(default)]
    pub interval: i64,
    #[serde(rename = "startDate")]
    #[serde(default)]
    pub start_date: NaiveDate,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
    #[serde(rename = "nextRunDate")]
    #[serde(default)]
    pub next_run_date: NaiveDate,
    pub status: RecurringInvoicesCreateSalesResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "autoIssue")]
    #[serde(default)]
    pub auto_issue: bool,
    #[serde(rename = "autoSend")]
    #[serde(default)]
    pub auto_send: bool,
    #[serde(rename = "sendTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_to: Option<String>,
    #[serde(default)]
    pub occurrences: i64,
    #[serde(rename = "lastRunAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_run_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "lastInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_invoice_id: Option<String>,
    #[serde(rename = "lastError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl RecurringInvoicesCreateSalesResponse {
    pub fn builder() -> RecurringInvoicesCreateSalesResponseBuilder {
        <RecurringInvoicesCreateSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecurringInvoicesCreateSalesResponseBuilder {
    id: Option<String>,
    template_id: Option<String>,
    template_name: Option<String>,
    partner_id: Option<String>,
    partner_name: Option<String>,
    frequency: Option<RecurringInvoicesCreateSalesResponseFrequency>,
    interval: Option<i64>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    next_run_date: Option<NaiveDate>,
    status: Option<RecurringInvoicesCreateSalesResponseStatus>,
    locale: Option<String>,
    auto_issue: Option<bool>,
    auto_send: Option<bool>,
    send_to: Option<String>,
    occurrences: Option<i64>,
    last_run_at: Option<DateTime<FixedOffset>>,
    last_invoice_id: Option<String>,
    last_error: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl RecurringInvoicesCreateSalesResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn template_id(mut self, value: impl Into<String>) -> Self {
        self.template_id = Some(value.into());
        self
    }

    pub fn template_name(mut self, value: impl Into<String>) -> Self {
        self.template_name = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn partner_name(mut self, value: impl Into<String>) -> Self {
        self.partner_name = Some(value.into());
        self
    }

    pub fn frequency(mut self, value: RecurringInvoicesCreateSalesResponseFrequency) -> Self {
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

    pub fn status(mut self, value: RecurringInvoicesCreateSalesResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = Some(value.into());
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

    pub fn occurrences(mut self, value: i64) -> Self {
        self.occurrences = Some(value);
        self
    }

    pub fn last_run_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_run_at = Some(value);
        self
    }

    pub fn last_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.last_invoice_id = Some(value.into());
        self
    }

    pub fn last_error(mut self, value: impl Into<String>) -> Self {
        self.last_error = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecurringInvoicesCreateSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RecurringInvoicesCreateSalesResponseBuilder::id)
    /// - [`template_id`](RecurringInvoicesCreateSalesResponseBuilder::template_id)
    /// - [`partner_id`](RecurringInvoicesCreateSalesResponseBuilder::partner_id)
    /// - [`frequency`](RecurringInvoicesCreateSalesResponseBuilder::frequency)
    /// - [`interval`](RecurringInvoicesCreateSalesResponseBuilder::interval)
    /// - [`start_date`](RecurringInvoicesCreateSalesResponseBuilder::start_date)
    /// - [`next_run_date`](RecurringInvoicesCreateSalesResponseBuilder::next_run_date)
    /// - [`status`](RecurringInvoicesCreateSalesResponseBuilder::status)
    /// - [`auto_issue`](RecurringInvoicesCreateSalesResponseBuilder::auto_issue)
    /// - [`auto_send`](RecurringInvoicesCreateSalesResponseBuilder::auto_send)
    /// - [`occurrences`](RecurringInvoicesCreateSalesResponseBuilder::occurrences)
    /// - [`created_at`](RecurringInvoicesCreateSalesResponseBuilder::created_at)
    /// - [`updated_at`](RecurringInvoicesCreateSalesResponseBuilder::updated_at)
    pub fn build(self) -> Result<RecurringInvoicesCreateSalesResponse, BuildError> {
        Ok(RecurringInvoicesCreateSalesResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            template_id: self
                .template_id
                .ok_or_else(|| BuildError::missing_field("template_id"))?,
            template_name: self.template_name,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            partner_name: self.partner_name,
            frequency: self
                .frequency
                .ok_or_else(|| BuildError::missing_field("frequency"))?,
            interval: self
                .interval
                .ok_or_else(|| BuildError::missing_field("interval"))?,
            start_date: self
                .start_date
                .ok_or_else(|| BuildError::missing_field("start_date"))?,
            end_date: self.end_date,
            next_run_date: self
                .next_run_date
                .ok_or_else(|| BuildError::missing_field("next_run_date"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            locale: self.locale,
            auto_issue: self
                .auto_issue
                .ok_or_else(|| BuildError::missing_field("auto_issue"))?,
            auto_send: self
                .auto_send
                .ok_or_else(|| BuildError::missing_field("auto_send"))?,
            send_to: self.send_to,
            occurrences: self
                .occurrences
                .ok_or_else(|| BuildError::missing_field("occurrences"))?,
            last_run_at: self.last_run_at,
            last_invoice_id: self.last_invoice_id,
            last_error: self.last_error,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
