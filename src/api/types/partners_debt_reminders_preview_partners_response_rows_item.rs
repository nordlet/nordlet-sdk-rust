pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DebtRemindersPreviewPartnersResponseRowsItem {
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "partnerName")]
    #[serde(default)]
    pub partner_name: String,
    #[serde(default)]
    pub email: String,
    pub locale: DebtRemindersPreviewPartnersResponseRowsItemLocale,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub invoices: Vec<DebtRemindersPreviewPartnersResponseRowsItemInvoicesItem>,
    #[serde(rename = "totalDue")]
    #[serde(default)]
    pub total_due: String,
    #[serde(rename = "interestDue")]
    #[serde(default)]
    pub interest_due: String,
}

impl DebtRemindersPreviewPartnersResponseRowsItem {
    pub fn builder() -> DebtRemindersPreviewPartnersResponseRowsItemBuilder {
        <DebtRemindersPreviewPartnersResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DebtRemindersPreviewPartnersResponseRowsItemBuilder {
    partner_id: Option<String>,
    partner_name: Option<String>,
    email: Option<String>,
    locale: Option<DebtRemindersPreviewPartnersResponseRowsItemLocale>,
    currency: Option<String>,
    invoices: Option<Vec<DebtRemindersPreviewPartnersResponseRowsItemInvoicesItem>>,
    total_due: Option<String>,
    interest_due: Option<String>,
}

impl DebtRemindersPreviewPartnersResponseRowsItemBuilder {
    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn partner_name(mut self, value: impl Into<String>) -> Self {
        self.partner_name = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn locale(mut self, value: DebtRemindersPreviewPartnersResponseRowsItemLocale) -> Self {
        self.locale = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn invoices(
        mut self,
        value: Vec<DebtRemindersPreviewPartnersResponseRowsItemInvoicesItem>,
    ) -> Self {
        self.invoices = Some(value);
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

    /// Consumes the builder and constructs a [`DebtRemindersPreviewPartnersResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_id`](DebtRemindersPreviewPartnersResponseRowsItemBuilder::partner_id)
    /// - [`partner_name`](DebtRemindersPreviewPartnersResponseRowsItemBuilder::partner_name)
    /// - [`email`](DebtRemindersPreviewPartnersResponseRowsItemBuilder::email)
    /// - [`locale`](DebtRemindersPreviewPartnersResponseRowsItemBuilder::locale)
    /// - [`currency`](DebtRemindersPreviewPartnersResponseRowsItemBuilder::currency)
    /// - [`invoices`](DebtRemindersPreviewPartnersResponseRowsItemBuilder::invoices)
    /// - [`total_due`](DebtRemindersPreviewPartnersResponseRowsItemBuilder::total_due)
    /// - [`interest_due`](DebtRemindersPreviewPartnersResponseRowsItemBuilder::interest_due)
    pub fn build(self) -> Result<DebtRemindersPreviewPartnersResponseRowsItem, BuildError> {
        Ok(DebtRemindersPreviewPartnersResponseRowsItem {
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            partner_name: self
                .partner_name
                .ok_or_else(|| BuildError::missing_field("partner_name"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            locale: self
                .locale
                .ok_or_else(|| BuildError::missing_field("locale"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            invoices: self
                .invoices
                .ok_or_else(|| BuildError::missing_field("invoices"))?,
            total_due: self
                .total_due
                .ok_or_else(|| BuildError::missing_field("total_due"))?,
            interest_due: self
                .interest_due
                .ok_or_else(|| BuildError::missing_field("interest_due"))?,
        })
    }
}
