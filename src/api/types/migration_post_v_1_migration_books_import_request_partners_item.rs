pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksImportRequestPartnersItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PostV1MigrationBooksImportRequestPartnersItemType>,
    #[serde(rename = "vatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "isCustomer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_customer: Option<bool>,
    #[serde(rename = "isSupplier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_supplier: Option<bool>,
    #[serde(rename = "paymentTermDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_term_days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostV1MigrationBooksImportRequestPartnersItemAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1MigrationBooksImportRequestPartnersItem {
    pub fn builder() -> PostV1MigrationBooksImportRequestPartnersItemBuilder {
        <PostV1MigrationBooksImportRequestPartnersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksImportRequestPartnersItemBuilder {
    code: Option<String>,
    name: Option<String>,
    r#type: Option<PostV1MigrationBooksImportRequestPartnersItemType>,
    vat_code: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    is_customer: Option<bool>,
    is_supplier: Option<bool>,
    payment_term_days: Option<i64>,
    address: Option<PostV1MigrationBooksImportRequestPartnersItemAddress>,
    notes: Option<String>,
}

impl PostV1MigrationBooksImportRequestPartnersItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1MigrationBooksImportRequestPartnersItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn vat_code(mut self, value: impl Into<String>) -> Self {
        self.vat_code = Some(value.into());
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

    pub fn address(mut self, value: PostV1MigrationBooksImportRequestPartnersItemAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksImportRequestPartnersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1MigrationBooksImportRequestPartnersItemBuilder::code)
    /// - [`name`](PostV1MigrationBooksImportRequestPartnersItemBuilder::name)
    pub fn build(self) -> Result<PostV1MigrationBooksImportRequestPartnersItem, BuildError> {
        Ok(PostV1MigrationBooksImportRequestPartnersItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type,
            vat_code: self.vat_code,
            email: self.email,
            phone: self.phone,
            is_customer: self.is_customer,
            is_supplier: self.is_supplier,
            payment_term_days: self.payment_term_days,
            address: self.address,
            notes: self.notes,
        })
    }
}
