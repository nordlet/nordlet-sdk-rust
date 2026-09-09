pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1PartnersListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    pub r#type: PostV1PartnersListResponseRowsItemType,
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
    pub address: Option<PostV1PartnersListResponseRowsItemAddress>,
    #[serde(rename = "correspondenceAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correspondence_address: Option<PostV1PartnersListResponseRowsItemCorrespondenceAddress>,
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
    #[serde(default)]
    pub auto_debt_reminder: bool,
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
    #[serde(default)]
    pub is_employee: bool,
    #[serde(rename = "isGroupMember")]
    #[serde(default)]
    pub is_group_member: bool,
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
    #[serde(rename = "legalCountryClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_country_class: Option<PostV1PartnersListResponseRowsItemLegalCountryClass>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl PostV1PartnersListResponseRowsItem {
    pub fn builder() -> PostV1PartnersListResponseRowsItemBuilder {
        <PostV1PartnersListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersListResponseRowsItemBuilder {
    id: Option<String>,
    r#type: Option<PostV1PartnersListResponseRowsItemType>,
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
    address: Option<PostV1PartnersListResponseRowsItemAddress>,
    correspondence_address: Option<PostV1PartnersListResponseRowsItemCorrespondenceAddress>,
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
    legal_country_class: Option<PostV1PartnersListResponseRowsItemLegalCountryClass>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl PostV1PartnersListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1PartnersListResponseRowsItemType) -> Self {
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

    pub fn address(mut self, value: PostV1PartnersListResponseRowsItemAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn correspondence_address(
        mut self,
        value: PostV1PartnersListResponseRowsItemCorrespondenceAddress,
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
        value: PostV1PartnersListResponseRowsItemLegalCountryClass,
    ) -> Self {
        self.legal_country_class = Some(value);
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

    /// Consumes the builder and constructs a [`PostV1PartnersListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersListResponseRowsItemBuilder::id)
    /// - [`r#type`](PostV1PartnersListResponseRowsItemBuilder::r#type)
    /// - [`name`](PostV1PartnersListResponseRowsItemBuilder::name)
    /// - [`is_customer`](PostV1PartnersListResponseRowsItemBuilder::is_customer)
    /// - [`is_supplier`](PostV1PartnersListResponseRowsItemBuilder::is_supplier)
    /// - [`auto_debt_reminder`](PostV1PartnersListResponseRowsItemBuilder::auto_debt_reminder)
    /// - [`is_employee`](PostV1PartnersListResponseRowsItemBuilder::is_employee)
    /// - [`is_group_member`](PostV1PartnersListResponseRowsItemBuilder::is_group_member)
    /// - [`is_active`](PostV1PartnersListResponseRowsItemBuilder::is_active)
    /// - [`created_at`](PostV1PartnersListResponseRowsItemBuilder::created_at)
    /// - [`updated_at`](PostV1PartnersListResponseRowsItemBuilder::updated_at)
    pub fn build(self) -> Result<PostV1PartnersListResponseRowsItem, BuildError> {
        Ok(PostV1PartnersListResponseRowsItem {
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
            correspondence_address: self.correspondence_address,
            notes: self.notes,
            document_ref: self.document_ref,
            short_name: self.short_name,
            website: self.website,
            fax: self.fax,
            eori_code: self.eori_code,
            other_code: self.other_code,
            foreign_tax_number: self.foreign_tax_number,
            auto_debt_reminder: self
                .auto_debt_reminder
                .ok_or_else(|| BuildError::missing_field("auto_debt_reminder"))?,
            late_interest_percent: self.late_interest_percent,
            first_call_date: self.first_call_date,
            last_call_date: self.last_call_date,
            next_call_date: self.next_call_date,
            rating: self.rating,
            is_employee: self
                .is_employee
                .ok_or_else(|| BuildError::missing_field("is_employee"))?,
            is_group_member: self
                .is_group_member
                .ok_or_else(|| BuildError::missing_field("is_group_member"))?,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            legal_country_class: self.legal_country_class,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
