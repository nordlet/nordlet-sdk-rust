pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DebtRemindersPreviewPartnersResponseRowsItemInvoicesItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "fullNumber")]
    #[serde(default)]
    pub full_number: String,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: NaiveDate,
    #[serde(rename = "dueDate")]
    #[serde(default)]
    pub due_date: NaiveDate,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub remaining: String,
    #[serde(rename = "daysLate")]
    #[serde(default)]
    pub days_late: i64,
    #[serde(default)]
    pub interest: String,
}

impl DebtRemindersPreviewPartnersResponseRowsItemInvoicesItem {
    pub fn builder() -> DebtRemindersPreviewPartnersResponseRowsItemInvoicesItemBuilder {
        <DebtRemindersPreviewPartnersResponseRowsItemInvoicesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DebtRemindersPreviewPartnersResponseRowsItemInvoicesItemBuilder {
    id: Option<String>,
    full_number: Option<String>,
    issue_date: Option<NaiveDate>,
    due_date: Option<NaiveDate>,
    currency: Option<String>,
    remaining: Option<String>,
    days_late: Option<i64>,
    interest: Option<String>,
}

impl DebtRemindersPreviewPartnersResponseRowsItemInvoicesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn full_number(mut self, value: impl Into<String>) -> Self {
        self.full_number = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: NaiveDate) -> Self {
        self.issue_date = Some(value);
        self
    }

    pub fn due_date(mut self, value: NaiveDate) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn remaining(mut self, value: impl Into<String>) -> Self {
        self.remaining = Some(value.into());
        self
    }

    pub fn days_late(mut self, value: i64) -> Self {
        self.days_late = Some(value);
        self
    }

    pub fn interest(mut self, value: impl Into<String>) -> Self {
        self.interest = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DebtRemindersPreviewPartnersResponseRowsItemInvoicesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DebtRemindersPreviewPartnersResponseRowsItemInvoicesItemBuilder::id)
    /// - [`full_number`](DebtRemindersPreviewPartnersResponseRowsItemInvoicesItemBuilder::full_number)
    /// - [`issue_date`](DebtRemindersPreviewPartnersResponseRowsItemInvoicesItemBuilder::issue_date)
    /// - [`due_date`](DebtRemindersPreviewPartnersResponseRowsItemInvoicesItemBuilder::due_date)
    /// - [`currency`](DebtRemindersPreviewPartnersResponseRowsItemInvoicesItemBuilder::currency)
    /// - [`remaining`](DebtRemindersPreviewPartnersResponseRowsItemInvoicesItemBuilder::remaining)
    /// - [`days_late`](DebtRemindersPreviewPartnersResponseRowsItemInvoicesItemBuilder::days_late)
    /// - [`interest`](DebtRemindersPreviewPartnersResponseRowsItemInvoicesItemBuilder::interest)
    pub fn build(
        self,
    ) -> Result<DebtRemindersPreviewPartnersResponseRowsItemInvoicesItem, BuildError> {
        Ok(DebtRemindersPreviewPartnersResponseRowsItemInvoicesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            full_number: self
                .full_number
                .ok_or_else(|| BuildError::missing_field("full_number"))?,
            issue_date: self
                .issue_date
                .ok_or_else(|| BuildError::missing_field("issue_date"))?,
            due_date: self
                .due_date
                .ok_or_else(|| BuildError::missing_field("due_date"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            remaining: self
                .remaining
                .ok_or_else(|| BuildError::missing_field("remaining"))?,
            days_late: self
                .days_late
                .ok_or_else(|| BuildError::missing_field("days_late"))?,
            interest: self
                .interest
                .ok_or_else(|| BuildError::missing_field("interest"))?,
        })
    }
}
