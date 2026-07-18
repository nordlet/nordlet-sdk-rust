pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsAgreementsUpdateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "typeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(default)]
    pub number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "startDate")]
    #[serde(default)]
    pub start_date: String,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "autoRenew")]
    #[serde(default)]
    pub auto_renew: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "billingPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<PostV1AgreementsAgreementsUpdateResponseBillingPeriod>,
    #[serde(default)]
    pub currency: String,
    pub status: PostV1AgreementsAgreementsUpdateResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub items: Vec<PostV1AgreementsAgreementsUpdateResponseItemsItem>,
}

impl PostV1AgreementsAgreementsUpdateResponse {
    pub fn builder() -> PostV1AgreementsAgreementsUpdateResponseBuilder {
        <PostV1AgreementsAgreementsUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsAgreementsUpdateResponseBuilder {
    id: Option<String>,
    type_id: Option<String>,
    partner_id: Option<String>,
    number: Option<String>,
    name: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    auto_renew: Option<bool>,
    value: Option<String>,
    billing_period: Option<PostV1AgreementsAgreementsUpdateResponseBillingPeriod>,
    currency: Option<String>,
    status: Option<PostV1AgreementsAgreementsUpdateResponseStatus>,
    notes: Option<String>,
    created_at: Option<String>,
    items: Option<Vec<PostV1AgreementsAgreementsUpdateResponseItemsItem>>,
}

impl PostV1AgreementsAgreementsUpdateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn type_id(mut self, value: impl Into<String>) -> Self {
        self.type_id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn number(mut self, value: impl Into<String>) -> Self {
        self.number = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    pub fn auto_renew(mut self, value: bool) -> Self {
        self.auto_renew = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn billing_period(
        mut self,
        value: PostV1AgreementsAgreementsUpdateResponseBillingPeriod,
    ) -> Self {
        self.billing_period = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostV1AgreementsAgreementsUpdateResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn items(mut self, value: Vec<PostV1AgreementsAgreementsUpdateResponseItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsAgreementsUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AgreementsAgreementsUpdateResponseBuilder::id)
    /// - [`partner_id`](PostV1AgreementsAgreementsUpdateResponseBuilder::partner_id)
    /// - [`number`](PostV1AgreementsAgreementsUpdateResponseBuilder::number)
    /// - [`start_date`](PostV1AgreementsAgreementsUpdateResponseBuilder::start_date)
    /// - [`auto_renew`](PostV1AgreementsAgreementsUpdateResponseBuilder::auto_renew)
    /// - [`currency`](PostV1AgreementsAgreementsUpdateResponseBuilder::currency)
    /// - [`status`](PostV1AgreementsAgreementsUpdateResponseBuilder::status)
    /// - [`created_at`](PostV1AgreementsAgreementsUpdateResponseBuilder::created_at)
    /// - [`items`](PostV1AgreementsAgreementsUpdateResponseBuilder::items)
    pub fn build(self) -> Result<PostV1AgreementsAgreementsUpdateResponse, BuildError> {
        Ok(PostV1AgreementsAgreementsUpdateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            type_id: self.type_id,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            number: self
                .number
                .ok_or_else(|| BuildError::missing_field("number"))?,
            name: self.name,
            start_date: self
                .start_date
                .ok_or_else(|| BuildError::missing_field("start_date"))?,
            end_date: self.end_date,
            auto_renew: self
                .auto_renew
                .ok_or_else(|| BuildError::missing_field("auto_renew"))?,
            value: self.value,
            billing_period: self.billing_period,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
