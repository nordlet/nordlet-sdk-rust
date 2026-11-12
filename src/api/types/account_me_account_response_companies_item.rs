pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MeAccountResponseCompaniesItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "vatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<String>,
    #[serde(default)]
    pub role: String,
    #[serde(rename = "isSandbox")]
    #[serde(default)]
    pub is_sandbox: bool,
    #[serde(rename = "cashBasis")]
    #[serde(default)]
    pub cash_basis: bool,
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "baseCurrency")]
    #[serde(default)]
    pub base_currency: String,
    #[serde(rename = "isVatPayer")]
    #[serde(default)]
    pub is_vat_payer: bool,
    #[serde(rename = "vatPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_period: Option<MeAccountResponseCompaniesItemVatPeriod>,
    #[serde(rename = "fiscalYearEndMonth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiscal_year_end_month: Option<i64>,
    pub status: MeAccountResponseCompaniesItemStatus,
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deleted_at: Option<DateTime<FixedOffset>>,
}

impl MeAccountResponseCompaniesItem {
    pub fn builder() -> MeAccountResponseCompaniesItemBuilder {
        <MeAccountResponseCompaniesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeAccountResponseCompaniesItemBuilder {
    id: Option<String>,
    name: Option<String>,
    code: Option<String>,
    vat_code: Option<String>,
    role: Option<String>,
    is_sandbox: Option<bool>,
    cash_basis: Option<bool>,
    country_code: Option<String>,
    base_currency: Option<String>,
    is_vat_payer: Option<bool>,
    vat_period: Option<MeAccountResponseCompaniesItemVatPeriod>,
    fiscal_year_end_month: Option<i64>,
    status: Option<MeAccountResponseCompaniesItemStatus>,
    deleted_at: Option<DateTime<FixedOffset>>,
}

impl MeAccountResponseCompaniesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn is_sandbox(mut self, value: bool) -> Self {
        self.is_sandbox = Some(value);
        self
    }

    pub fn cash_basis(mut self, value: bool) -> Self {
        self.cash_basis = Some(value);
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn base_currency(mut self, value: impl Into<String>) -> Self {
        self.base_currency = Some(value.into());
        self
    }

    pub fn is_vat_payer(mut self, value: bool) -> Self {
        self.is_vat_payer = Some(value);
        self
    }

    pub fn vat_period(mut self, value: MeAccountResponseCompaniesItemVatPeriod) -> Self {
        self.vat_period = Some(value);
        self
    }

    pub fn fiscal_year_end_month(mut self, value: i64) -> Self {
        self.fiscal_year_end_month = Some(value);
        self
    }

    pub fn status(mut self, value: MeAccountResponseCompaniesItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MeAccountResponseCompaniesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MeAccountResponseCompaniesItemBuilder::id)
    /// - [`name`](MeAccountResponseCompaniesItemBuilder::name)
    /// - [`role`](MeAccountResponseCompaniesItemBuilder::role)
    /// - [`is_sandbox`](MeAccountResponseCompaniesItemBuilder::is_sandbox)
    /// - [`cash_basis`](MeAccountResponseCompaniesItemBuilder::cash_basis)
    /// - [`country_code`](MeAccountResponseCompaniesItemBuilder::country_code)
    /// - [`base_currency`](MeAccountResponseCompaniesItemBuilder::base_currency)
    /// - [`is_vat_payer`](MeAccountResponseCompaniesItemBuilder::is_vat_payer)
    /// - [`status`](MeAccountResponseCompaniesItemBuilder::status)
    pub fn build(self) -> Result<MeAccountResponseCompaniesItem, BuildError> {
        Ok(MeAccountResponseCompaniesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            code: self.code,
            vat_code: self.vat_code,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            is_sandbox: self
                .is_sandbox
                .ok_or_else(|| BuildError::missing_field("is_sandbox"))?,
            cash_basis: self
                .cash_basis
                .ok_or_else(|| BuildError::missing_field("cash_basis"))?,
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            base_currency: self
                .base_currency
                .ok_or_else(|| BuildError::missing_field("base_currency"))?,
            is_vat_payer: self
                .is_vat_payer
                .ok_or_else(|| BuildError::missing_field("is_vat_payer"))?,
            vat_period: self.vat_period,
            fiscal_year_end_month: self.fiscal_year_end_month,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            deleted_at: self.deleted_at,
        })
    }
}
