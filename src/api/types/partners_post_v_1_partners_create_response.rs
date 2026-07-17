pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1PartnersCreateResponse {
    #[serde(default)]
    pub id: String,
    pub r#type: PostV1PartnersCreateResponseType,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "vatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<String>,
    #[serde(rename = "peppolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peppol_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "selfEmploymentCertNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_employment_cert_no: Option<String>,
    #[serde(rename = "birthDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    #[serde(rename = "isCustomer")]
    #[serde(default)]
    pub is_customer: bool,
    #[serde(rename = "isSupplier")]
    #[serde(default)]
    pub is_supplier: bool,
    #[serde(rename = "paymentTermDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_term_days: Option<i64>,
    #[serde(rename = "creditLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_limit: Option<String>,
    #[serde(rename = "priceListId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_list_id: Option<String>,
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "statusId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_id: Option<String>,
    #[serde(rename = "vatValid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_valid: Option<bool>,
    #[serde(rename = "vatValidatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_validated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostV1PartnersCreateResponseAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl PostV1PartnersCreateResponse {
    pub fn builder() -> PostV1PartnersCreateResponseBuilder {
        <PostV1PartnersCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersCreateResponseBuilder {
    id: Option<String>,
    r#type: Option<PostV1PartnersCreateResponseType>,
    name: Option<String>,
    code: Option<String>,
    vat_code: Option<String>,
    peppol_id: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    self_employment_cert_no: Option<String>,
    birth_date: Option<String>,
    is_customer: Option<bool>,
    is_supplier: Option<bool>,
    payment_term_days: Option<i64>,
    credit_limit: Option<String>,
    price_list_id: Option<String>,
    group_id: Option<String>,
    status_id: Option<String>,
    vat_valid: Option<bool>,
    vat_validated_at: Option<String>,
    address: Option<PostV1PartnersCreateResponseAddress>,
    notes: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl PostV1PartnersCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1PartnersCreateResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn vat_code(mut self, value: impl Into<String>) -> Self {
        self.vat_code = Some(value.into());
        self
    }

    pub fn peppol_id(mut self, value: impl Into<String>) -> Self {
        self.peppol_id = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn self_employment_cert_no(mut self, value: impl Into<String>) -> Self {
        self.self_employment_cert_no = Some(value.into());
        self
    }

    pub fn birth_date(mut self, value: impl Into<String>) -> Self {
        self.birth_date = Some(value.into());
        self
    }

    pub fn is_customer(mut self, value: bool) -> Self {
        self.is_customer = Some(value);
        self
    }

    pub fn is_supplier(mut self, value: bool) -> Self {
        self.is_supplier = Some(value);
        self
    }

    pub fn payment_term_days(mut self, value: i64) -> Self {
        self.payment_term_days = Some(value);
        self
    }

    pub fn credit_limit(mut self, value: impl Into<String>) -> Self {
        self.credit_limit = Some(value.into());
        self
    }

    pub fn price_list_id(mut self, value: impl Into<String>) -> Self {
        self.price_list_id = Some(value.into());
        self
    }

    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn status_id(mut self, value: impl Into<String>) -> Self {
        self.status_id = Some(value.into());
        self
    }

    pub fn vat_valid(mut self, value: bool) -> Self {
        self.vat_valid = Some(value);
        self
    }

    pub fn vat_validated_at(mut self, value: impl Into<String>) -> Self {
        self.vat_validated_at = Some(value.into());
        self
    }

    pub fn address(mut self, value: PostV1PartnersCreateResponseAddress) -> Self {
        self.address = Some(value);
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

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersCreateResponseBuilder::id)
    /// - [`r#type`](PostV1PartnersCreateResponseBuilder::r#type)
    /// - [`name`](PostV1PartnersCreateResponseBuilder::name)
    /// - [`is_customer`](PostV1PartnersCreateResponseBuilder::is_customer)
    /// - [`is_supplier`](PostV1PartnersCreateResponseBuilder::is_supplier)
    /// - [`created_at`](PostV1PartnersCreateResponseBuilder::created_at)
    /// - [`updated_at`](PostV1PartnersCreateResponseBuilder::updated_at)
    pub fn build(self) -> Result<PostV1PartnersCreateResponse, BuildError> {
        Ok(PostV1PartnersCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            code: self.code,
            vat_code: self.vat_code,
            peppol_id: self.peppol_id,
            email: self.email,
            phone: self.phone,
            self_employment_cert_no: self.self_employment_cert_no,
            birth_date: self.birth_date,
            is_customer: self
                .is_customer
                .ok_or_else(|| BuildError::missing_field("is_customer"))?,
            is_supplier: self
                .is_supplier
                .ok_or_else(|| BuildError::missing_field("is_supplier"))?,
            payment_term_days: self.payment_term_days,
            credit_limit: self.credit_limit,
            price_list_id: self.price_list_id,
            group_id: self.group_id,
            status_id: self.status_id,
            vat_valid: self.vat_valid,
            vat_validated_at: self.vat_validated_at,
            address: self.address,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
