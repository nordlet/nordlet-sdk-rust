pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksValidateMigrationRequestPartnersItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<BooksValidateMigrationRequestPartnersItemType>,
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
    pub address: Option<BooksValidateMigrationRequestPartnersItemAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl BooksValidateMigrationRequestPartnersItem {
    pub fn builder() -> BooksValidateMigrationRequestPartnersItemBuilder {
        <BooksValidateMigrationRequestPartnersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksValidateMigrationRequestPartnersItemBuilder {
    code: Option<String>,
    name: Option<String>,
    r#type: Option<BooksValidateMigrationRequestPartnersItemType>,
    vat_code: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    is_customer: Option<bool>,
    is_supplier: Option<bool>,
    payment_term_days: Option<i64>,
    address: Option<BooksValidateMigrationRequestPartnersItemAddress>,
    notes: Option<String>,
}

impl BooksValidateMigrationRequestPartnersItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: BooksValidateMigrationRequestPartnersItemType) -> Self {
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

    pub fn address(mut self, value: BooksValidateMigrationRequestPartnersItemAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BooksValidateMigrationRequestPartnersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](BooksValidateMigrationRequestPartnersItemBuilder::code)
    /// - [`name`](BooksValidateMigrationRequestPartnersItemBuilder::name)
    pub fn build(self) -> Result<BooksValidateMigrationRequestPartnersItem, BuildError> {
        Ok(BooksValidateMigrationRequestPartnersItem {
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
