pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersFindOrCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PostV1PartnersFindOrCreateRequestType>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_customer: Option<bool>,
    #[serde(rename = "isSupplier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_supplier: Option<bool>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostV1PartnersFindOrCreateRequestAddress>,
    #[serde(rename = "correspondenceAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correspondence_address: Option<PostV1PartnersFindOrCreateRequestCorrespondenceAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "documentRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_ref: Option<String>,
    #[serde(rename = "shortName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    #[serde(rename = "eoriCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eori_code: Option<String>,
    #[serde(rename = "otherCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_code: Option<String>,
    #[serde(rename = "foreignTaxNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_tax_number: Option<String>,
    #[serde(rename = "autoDebtReminder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_debt_reminder: Option<bool>,
    #[serde(rename = "lateInterestPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub late_interest_percent: Option<String>,
    #[serde(rename = "firstCallDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_call_date: Option<String>,
    #[serde(rename = "lastCallDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_call_date: Option<String>,
    #[serde(rename = "nextCallDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_call_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<i64>,
    #[serde(rename = "isEmployee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_employee: Option<bool>,
    #[serde(rename = "isGroupMember")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_group_member: Option<bool>,
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "legalCountryClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_country_class: Option<PostV1PartnersFindOrCreateRequestLegalCountryClass>,
}

impl PostV1PartnersFindOrCreateRequest {
    pub fn builder() -> PostV1PartnersFindOrCreateRequestBuilder {
        <PostV1PartnersFindOrCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersFindOrCreateRequestBuilder {
    r#type: Option<PostV1PartnersFindOrCreateRequestType>,
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
    address: Option<PostV1PartnersFindOrCreateRequestAddress>,
    correspondence_address: Option<PostV1PartnersFindOrCreateRequestCorrespondenceAddress>,
    notes: Option<String>,
    document_ref: Option<String>,
    short_name: Option<String>,
    website: Option<String>,
    fax: Option<String>,
    eori_code: Option<String>,
    other_code: Option<String>,
    foreign_tax_number: Option<String>,
    auto_debt_reminder: Option<bool>,
    late_interest_percent: Option<String>,
    first_call_date: Option<String>,
    last_call_date: Option<String>,
    next_call_date: Option<String>,
    rating: Option<i64>,
    is_employee: Option<bool>,
    is_group_member: Option<bool>,
    is_active: Option<bool>,
    legal_country_class: Option<PostV1PartnersFindOrCreateRequestLegalCountryClass>,
}

impl PostV1PartnersFindOrCreateRequestBuilder {
    pub fn r#type(mut self, value: PostV1PartnersFindOrCreateRequestType) -> Self {
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

    pub fn address(mut self, value: PostV1PartnersFindOrCreateRequestAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn correspondence_address(
        mut self,
        value: PostV1PartnersFindOrCreateRequestCorrespondenceAddress,
    ) -> Self {
        self.correspondence_address = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn document_ref(mut self, value: impl Into<String>) -> Self {
        self.document_ref = Some(value.into());
        self
    }

    pub fn short_name(mut self, value: impl Into<String>) -> Self {
        self.short_name = Some(value.into());
        self
    }

    pub fn website(mut self, value: impl Into<String>) -> Self {
        self.website = Some(value.into());
        self
    }

    pub fn fax(mut self, value: impl Into<String>) -> Self {
        self.fax = Some(value.into());
        self
    }

    pub fn eori_code(mut self, value: impl Into<String>) -> Self {
        self.eori_code = Some(value.into());
        self
    }

    pub fn other_code(mut self, value: impl Into<String>) -> Self {
        self.other_code = Some(value.into());
        self
    }

    pub fn foreign_tax_number(mut self, value: impl Into<String>) -> Self {
        self.foreign_tax_number = Some(value.into());
        self
    }

    pub fn auto_debt_reminder(mut self, value: bool) -> Self {
        self.auto_debt_reminder = Some(value);
        self
    }

    pub fn late_interest_percent(mut self, value: impl Into<String>) -> Self {
        self.late_interest_percent = Some(value.into());
        self
    }

    pub fn first_call_date(mut self, value: impl Into<String>) -> Self {
        self.first_call_date = Some(value.into());
        self
    }

    pub fn last_call_date(mut self, value: impl Into<String>) -> Self {
        self.last_call_date = Some(value.into());
        self
    }

    pub fn next_call_date(mut self, value: impl Into<String>) -> Self {
        self.next_call_date = Some(value.into());
        self
    }

    pub fn rating(mut self, value: i64) -> Self {
        self.rating = Some(value);
        self
    }

    pub fn is_employee(mut self, value: bool) -> Self {
        self.is_employee = Some(value);
        self
    }

    pub fn is_group_member(mut self, value: bool) -> Self {
        self.is_group_member = Some(value);
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn legal_country_class(
        mut self,
        value: PostV1PartnersFindOrCreateRequestLegalCountryClass,
    ) -> Self {
        self.legal_country_class = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersFindOrCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1PartnersFindOrCreateRequestBuilder::name)
    pub fn build(self) -> Result<PostV1PartnersFindOrCreateRequest, BuildError> {
        Ok(PostV1PartnersFindOrCreateRequest {
            r#type: self.r#type,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            code: self.code,
            vat_code: self.vat_code,
            peppol_id: self.peppol_id,
            email: self.email,
            phone: self.phone,
            self_employment_cert_no: self.self_employment_cert_no,
            birth_date: self.birth_date,
            is_customer: self.is_customer,
            is_supplier: self.is_supplier,
            payment_term_days: self.payment_term_days,
            credit_limit: self.credit_limit,
            price_list_id: self.price_list_id,
            group_id: self.group_id,
            status_id: self.status_id,
            address: self.address,
            correspondence_address: self.correspondence_address,
            notes: self.notes,
            document_ref: self.document_ref,
            short_name: self.short_name,
            website: self.website,
            fax: self.fax,
            eori_code: self.eori_code,
            other_code: self.other_code,
            foreign_tax_number: self.foreign_tax_number,
            auto_debt_reminder: self.auto_debt_reminder,
            late_interest_percent: self.late_interest_percent,
            first_call_date: self.first_call_date,
            last_call_date: self.last_call_date,
            next_call_date: self.next_call_date,
            rating: self.rating,
            is_employee: self.is_employee,
            is_group_member: self.is_group_member,
            is_active: self.is_active,
            legal_country_class: self.legal_country_class,
        })
    }
}
