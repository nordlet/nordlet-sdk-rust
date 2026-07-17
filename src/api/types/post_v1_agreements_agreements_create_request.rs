pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsAgreementsCreateRequest {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PostV1AgreementsAgreementsCreateRequestStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PostV1AgreementsAgreementsCreateRequestItemsItem>>,
}

impl PostV1AgreementsAgreementsCreateRequest {
    pub fn builder() -> PostV1AgreementsAgreementsCreateRequestBuilder {
        <PostV1AgreementsAgreementsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsAgreementsCreateRequestBuilder {
    type_id: Option<String>,
    partner_id: Option<String>,
    number: Option<String>,
    name: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    auto_renew: Option<bool>,
    value: Option<String>,
    currency: Option<String>,
    status: Option<PostV1AgreementsAgreementsCreateRequestStatus>,
    notes: Option<String>,
    items: Option<Vec<PostV1AgreementsAgreementsCreateRequestItemsItem>>,
}

impl PostV1AgreementsAgreementsCreateRequestBuilder {
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

    pub fn status(mut self, value: PostV1AgreementsAgreementsCreateRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn items(mut self, value: Vec<PostV1AgreementsAgreementsCreateRequestItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsAgreementsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_id`](PostV1AgreementsAgreementsCreateRequestBuilder::partner_id)
    /// - [`number`](PostV1AgreementsAgreementsCreateRequestBuilder::number)
    /// - [`start_date`](PostV1AgreementsAgreementsCreateRequestBuilder::start_date)
    pub fn build(self) -> Result<PostV1AgreementsAgreementsCreateRequest, BuildError> {
        Ok(PostV1AgreementsAgreementsCreateRequest {
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
            auto_renew: self.auto_renew,
            value: self.value,
            currency: self.currency,
            status: self.status,
            notes: self.notes,
            items: self.items,
        })
    }
}
