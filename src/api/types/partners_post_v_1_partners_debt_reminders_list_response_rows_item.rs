pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersDebtRemindersListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "sentTo")]
    #[serde(default)]
    pub sent_to: String,
    #[serde(rename = "invoiceCount")]
    #[serde(default)]
    pub invoice_count: i64,
    #[serde(rename = "totalDue")]
    #[serde(default)]
    pub total_due: String,
    #[serde(rename = "interestDue")]
    #[serde(default)]
    pub interest_due: String,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "invoiceIds")]
    #[serde(default)]
    pub invoice_ids: Vec<String>,
    #[serde(rename = "sentAt")]
    #[serde(default)]
    pub sent_at: String,
}

impl PostV1PartnersDebtRemindersListResponseRowsItem {
    pub fn builder() -> PostV1PartnersDebtRemindersListResponseRowsItemBuilder {
        <PostV1PartnersDebtRemindersListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersDebtRemindersListResponseRowsItemBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    sent_to: Option<String>,
    invoice_count: Option<i64>,
    total_due: Option<String>,
    interest_due: Option<String>,
    currency: Option<String>,
    invoice_ids: Option<Vec<String>>,
    sent_at: Option<String>,
}

impl PostV1PartnersDebtRemindersListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn sent_to(mut self, value: impl Into<String>) -> Self {
        self.sent_to = Some(value.into());
        self
    }

    pub fn invoice_count(mut self, value: i64) -> Self {
        self.invoice_count = Some(value);
        self
    }

    pub fn total_due(mut self, value: impl Into<String>) -> Self {
        self.total_due = Some(value.into());
        self
    }

    pub fn interest_due(mut self, value: impl Into<String>) -> Self {
        self.interest_due = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn invoice_ids(mut self, value: Vec<String>) -> Self {
        self.invoice_ids = Some(value);
        self
    }

    pub fn sent_at(mut self, value: impl Into<String>) -> Self {
        self.sent_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersDebtRemindersListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersDebtRemindersListResponseRowsItemBuilder::id)
    /// - [`partner_id`](PostV1PartnersDebtRemindersListResponseRowsItemBuilder::partner_id)
    /// - [`sent_to`](PostV1PartnersDebtRemindersListResponseRowsItemBuilder::sent_to)
    /// - [`invoice_count`](PostV1PartnersDebtRemindersListResponseRowsItemBuilder::invoice_count)
    /// - [`total_due`](PostV1PartnersDebtRemindersListResponseRowsItemBuilder::total_due)
    /// - [`interest_due`](PostV1PartnersDebtRemindersListResponseRowsItemBuilder::interest_due)
    /// - [`currency`](PostV1PartnersDebtRemindersListResponseRowsItemBuilder::currency)
    /// - [`invoice_ids`](PostV1PartnersDebtRemindersListResponseRowsItemBuilder::invoice_ids)
    /// - [`sent_at`](PostV1PartnersDebtRemindersListResponseRowsItemBuilder::sent_at)
    pub fn build(self) -> Result<PostV1PartnersDebtRemindersListResponseRowsItem, BuildError> {
        Ok(PostV1PartnersDebtRemindersListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            sent_to: self
                .sent_to
                .ok_or_else(|| BuildError::missing_field("sent_to"))?,
            invoice_count: self
                .invoice_count
                .ok_or_else(|| BuildError::missing_field("invoice_count"))?,
            total_due: self
                .total_due
                .ok_or_else(|| BuildError::missing_field("total_due"))?,
            interest_due: self
                .interest_due
                .ok_or_else(|| BuildError::missing_field("interest_due"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            invoice_ids: self
                .invoice_ids
                .ok_or_else(|| BuildError::missing_field("invoice_ids"))?,
            sent_at: self
                .sent_at
                .ok_or_else(|| BuildError::missing_field("sent_at"))?,
        })
    }
}
