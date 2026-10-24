pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DebtRemindersUpcomingPartnersResponseRowsItem {
    #[serde(default)]
    pub date: NaiveDate,
    pub kind: DebtRemindersUpcomingPartnersResponseRowsItemKind,
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    #[serde(rename = "fullNumber")]
    #[serde(default)]
    pub full_number: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "partnerName")]
    #[serde(default)]
    pub partner_name: String,
    #[serde(rename = "dueDate")]
    #[serde(default)]
    pub due_date: NaiveDate,
    #[serde(default)]
    pub remaining: String,
    #[serde(default)]
    pub currency: String,
}

impl DebtRemindersUpcomingPartnersResponseRowsItem {
    pub fn builder() -> DebtRemindersUpcomingPartnersResponseRowsItemBuilder {
        <DebtRemindersUpcomingPartnersResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DebtRemindersUpcomingPartnersResponseRowsItemBuilder {
    date: Option<NaiveDate>,
    kind: Option<DebtRemindersUpcomingPartnersResponseRowsItemKind>,
    invoice_id: Option<String>,
    full_number: Option<String>,
    partner_id: Option<String>,
    partner_name: Option<String>,
    due_date: Option<NaiveDate>,
    remaining: Option<String>,
    currency: Option<String>,
}

impl DebtRemindersUpcomingPartnersResponseRowsItemBuilder {
    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn kind(mut self, value: DebtRemindersUpcomingPartnersResponseRowsItemKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn full_number(mut self, value: impl Into<String>) -> Self {
        self.full_number = Some(value.into());
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

    pub fn due_date(mut self, value: NaiveDate) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn remaining(mut self, value: impl Into<String>) -> Self {
        self.remaining = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DebtRemindersUpcomingPartnersResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](DebtRemindersUpcomingPartnersResponseRowsItemBuilder::date)
    /// - [`kind`](DebtRemindersUpcomingPartnersResponseRowsItemBuilder::kind)
    /// - [`invoice_id`](DebtRemindersUpcomingPartnersResponseRowsItemBuilder::invoice_id)
    /// - [`full_number`](DebtRemindersUpcomingPartnersResponseRowsItemBuilder::full_number)
    /// - [`partner_id`](DebtRemindersUpcomingPartnersResponseRowsItemBuilder::partner_id)
    /// - [`partner_name`](DebtRemindersUpcomingPartnersResponseRowsItemBuilder::partner_name)
    /// - [`due_date`](DebtRemindersUpcomingPartnersResponseRowsItemBuilder::due_date)
    /// - [`remaining`](DebtRemindersUpcomingPartnersResponseRowsItemBuilder::remaining)
    /// - [`currency`](DebtRemindersUpcomingPartnersResponseRowsItemBuilder::currency)
    pub fn build(self) -> Result<DebtRemindersUpcomingPartnersResponseRowsItem, BuildError> {
        Ok(DebtRemindersUpcomingPartnersResponseRowsItem {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            invoice_id: self
                .invoice_id
                .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
            full_number: self
                .full_number
                .ok_or_else(|| BuildError::missing_field("full_number"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            partner_name: self
                .partner_name
                .ok_or_else(|| BuildError::missing_field("partner_name"))?,
            due_date: self
                .due_date
                .ok_or_else(|| BuildError::missing_field("due_date"))?,
            remaining: self
                .remaining
                .ok_or_else(|| BuildError::missing_field("remaining"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
        })
    }
}
