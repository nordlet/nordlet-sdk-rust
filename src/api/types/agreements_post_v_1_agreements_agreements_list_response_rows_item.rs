pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsAgreementsListResponseRowsItem {
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
    #[serde(default)]
    pub currency: String,
    pub status: PostV1AgreementsAgreementsListResponseRowsItemStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1AgreementsAgreementsListResponseRowsItem {
    pub fn builder() -> PostV1AgreementsAgreementsListResponseRowsItemBuilder {
        <PostV1AgreementsAgreementsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsAgreementsListResponseRowsItemBuilder {
    id: Option<String>,
    type_id: Option<String>,
    partner_id: Option<String>,
    number: Option<String>,
    name: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    auto_renew: Option<bool>,
    value: Option<String>,
    currency: Option<String>,
    status: Option<PostV1AgreementsAgreementsListResponseRowsItemStatus>,
    notes: Option<String>,
    created_at: Option<String>,
}

impl PostV1AgreementsAgreementsListResponseRowsItemBuilder {
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

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostV1AgreementsAgreementsListResponseRowsItemStatus) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1AgreementsAgreementsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AgreementsAgreementsListResponseRowsItemBuilder::id)
    /// - [`partner_id`](PostV1AgreementsAgreementsListResponseRowsItemBuilder::partner_id)
    /// - [`number`](PostV1AgreementsAgreementsListResponseRowsItemBuilder::number)
    /// - [`start_date`](PostV1AgreementsAgreementsListResponseRowsItemBuilder::start_date)
    /// - [`auto_renew`](PostV1AgreementsAgreementsListResponseRowsItemBuilder::auto_renew)
    /// - [`currency`](PostV1AgreementsAgreementsListResponseRowsItemBuilder::currency)
    /// - [`status`](PostV1AgreementsAgreementsListResponseRowsItemBuilder::status)
    /// - [`created_at`](PostV1AgreementsAgreementsListResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1AgreementsAgreementsListResponseRowsItem, BuildError> {
        Ok(PostV1AgreementsAgreementsListResponseRowsItem {
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
        })
    }
}
