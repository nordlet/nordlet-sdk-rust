pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersDebtRemindersPreviewResponseRowsItemInvoicesItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "fullNumber")]
    #[serde(default)]
    pub full_number: String,
    #[serde(rename = "issueDate")]
    #[serde(default)]
    pub issue_date: String,
    #[serde(rename = "dueDate")]
    #[serde(default)]
    pub due_date: String,
    #[serde(default)]
    pub remaining: String,
    #[serde(rename = "daysLate")]
    #[serde(default)]
    pub days_late: i64,
    #[serde(default)]
    pub interest: String,
}

impl PostV1PartnersDebtRemindersPreviewResponseRowsItemInvoicesItem {
    pub fn builder() -> PostV1PartnersDebtRemindersPreviewResponseRowsItemInvoicesItemBuilder {
        <PostV1PartnersDebtRemindersPreviewResponseRowsItemInvoicesItemBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersDebtRemindersPreviewResponseRowsItemInvoicesItemBuilder {
    id: Option<String>,
    full_number: Option<String>,
    issue_date: Option<String>,
    due_date: Option<String>,
    remaining: Option<String>,
    days_late: Option<i64>,
    interest: Option<String>,
}

impl PostV1PartnersDebtRemindersPreviewResponseRowsItemInvoicesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn full_number(mut self, value: impl Into<String>) -> Self {
        self.full_number = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: impl Into<String>) -> Self {
        self.issue_date = Some(value.into());
        self
    }

    pub fn due_date(mut self, value: impl Into<String>) -> Self {
        self.due_date = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1PartnersDebtRemindersPreviewResponseRowsItemInvoicesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersDebtRemindersPreviewResponseRowsItemInvoicesItemBuilder::id)
    /// - [`full_number`](PostV1PartnersDebtRemindersPreviewResponseRowsItemInvoicesItemBuilder::full_number)
    /// - [`issue_date`](PostV1PartnersDebtRemindersPreviewResponseRowsItemInvoicesItemBuilder::issue_date)
    /// - [`due_date`](PostV1PartnersDebtRemindersPreviewResponseRowsItemInvoicesItemBuilder::due_date)
    /// - [`remaining`](PostV1PartnersDebtRemindersPreviewResponseRowsItemInvoicesItemBuilder::remaining)
    /// - [`days_late`](PostV1PartnersDebtRemindersPreviewResponseRowsItemInvoicesItemBuilder::days_late)
    /// - [`interest`](PostV1PartnersDebtRemindersPreviewResponseRowsItemInvoicesItemBuilder::interest)
    pub fn build(
        self,
    ) -> Result<PostV1PartnersDebtRemindersPreviewResponseRowsItemInvoicesItem, BuildError> {
        Ok(
            PostV1PartnersDebtRemindersPreviewResponseRowsItemInvoicesItem {
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
                remaining: self
                    .remaining
                    .ok_or_else(|| BuildError::missing_field("remaining"))?,
                days_late: self
                    .days_late
                    .ok_or_else(|| BuildError::missing_field("days_late"))?,
                interest: self
                    .interest
                    .ok_or_else(|| BuildError::missing_field("interest"))?,
            },
        )
    }
}
