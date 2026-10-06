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
    pub invoices: Vec<DebtRemindersPreviewPartnersResponseRowsItemInvoicesItem>,
    #[serde(default)]
    pub totals: Vec<DebtRemindersPreviewPartnersResponseRowsItemTotalsItem>,
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
    invoices: Option<Vec<DebtRemindersPreviewPartnersResponseRowsItemInvoicesItem>>,
    totals: Option<Vec<DebtRemindersPreviewPartnersResponseRowsItemTotalsItem>>,
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

    pub fn invoices(
        mut self,
        value: Vec<DebtRemindersPreviewPartnersResponseRowsItemInvoicesItem>,
    ) -> Self {
        self.invoices = Some(value);
        self
    }

    pub fn totals(
        mut self,
        value: Vec<DebtRemindersPreviewPartnersResponseRowsItemTotalsItem>,
    ) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DebtRemindersPreviewPartnersResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_id`](DebtRemindersPreviewPartnersResponseRowsItemBuilder::partner_id)
    /// - [`partner_name`](DebtRemindersPreviewPartnersResponseRowsItemBuilder::partner_name)
    /// - [`email`](DebtRemindersPreviewPartnersResponseRowsItemBuilder::email)
    /// - [`locale`](DebtRemindersPreviewPartnersResponseRowsItemBuilder::locale)
    /// - [`invoices`](DebtRemindersPreviewPartnersResponseRowsItemBuilder::invoices)
    /// - [`totals`](DebtRemindersPreviewPartnersResponseRowsItemBuilder::totals)
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
            invoices: self
                .invoices
                .ok_or_else(|| BuildError::missing_field("invoices"))?,
            totals: self
                .totals
                .ok_or_else(|| BuildError::missing_field("totals"))?,
        })
    }
}
