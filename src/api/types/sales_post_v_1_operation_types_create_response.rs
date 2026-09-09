pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1OperationTypesCreateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "invoiceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_type: Option<PostV1OperationTypesCreateResponseInvoiceType>,
    #[serde(rename = "payerPartnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_partner_id: Option<String>,
    #[serde(rename = "debitAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_account_code: Option<String>,
    #[serde(rename = "creditAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_account_code: Option<String>,
    #[serde(rename = "vatAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_account_code: Option<String>,
    #[serde(rename = "expenseAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_account_code: Option<String>,
    #[serde(rename = "advanceAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advance_account_code: Option<String>,
    #[serde(rename = "incomeAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub income_account_code: Option<String>,
    #[serde(rename = "isPurchase")]
    #[serde(default)]
    pub is_purchase: bool,
    #[serde(rename = "isSale")]
    #[serde(default)]
    pub is_sale: bool,
    #[serde(rename = "isWriteOff")]
    #[serde(default)]
    pub is_write_off: bool,
    #[serde(rename = "isInternalMovement")]
    #[serde(default)]
    pub is_internal_movement: bool,
    #[serde(rename = "isPurchaseReturn")]
    #[serde(default)]
    pub is_purchase_return: bool,
    #[serde(rename = "isSalesReturn")]
    #[serde(default)]
    pub is_sales_return: bool,
    #[serde(rename = "isConsignment")]
    #[serde(default)]
    pub is_consignment: bool,
    #[serde(rename = "isProduction")]
    #[serde(default)]
    pub is_production: bool,
    #[serde(rename = "isAssetIn")]
    #[serde(default)]
    pub is_asset_in: bool,
    #[serde(rename = "isAssetOut")]
    #[serde(default)]
    pub is_asset_out: bool,
    #[serde(rename = "isCashRegisterSale")]
    #[serde(default)]
    pub is_cash_register_sale: bool,
    #[serde(rename = "includeInVatRegister")]
    #[serde(default)]
    pub include_in_vat_register: bool,
    #[serde(rename = "includeInSaft")]
    #[serde(default)]
    pub include_in_saft: bool,
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    pub sort_order: i64,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl PostV1OperationTypesCreateResponse {
    pub fn builder() -> PostV1OperationTypesCreateResponseBuilder {
        <PostV1OperationTypesCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1OperationTypesCreateResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    invoice_type: Option<PostV1OperationTypesCreateResponseInvoiceType>,
    payer_partner_id: Option<String>,
    debit_account_code: Option<String>,
    credit_account_code: Option<String>,
    vat_account_code: Option<String>,
    expense_account_code: Option<String>,
    advance_account_code: Option<String>,
    income_account_code: Option<String>,
    is_purchase: Option<bool>,
    is_sale: Option<bool>,
    is_write_off: Option<bool>,
    is_internal_movement: Option<bool>,
    is_purchase_return: Option<bool>,
    is_sales_return: Option<bool>,
    is_consignment: Option<bool>,
    is_production: Option<bool>,
    is_asset_in: Option<bool>,
    is_asset_out: Option<bool>,
    is_cash_register_sale: Option<bool>,
    include_in_vat_register: Option<bool>,
    include_in_saft: Option<bool>,
    is_active: Option<bool>,
    sort_order: Option<i64>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl PostV1OperationTypesCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn invoice_type(mut self, value: PostV1OperationTypesCreateResponseInvoiceType) -> Self {
        self.invoice_type = Some(value);
        self
    }

    pub fn payer_partner_id(mut self, value: impl Into<String>) -> Self {
        self.payer_partner_id = Some(value.into());
        self
    }

    pub fn debit_account_code(mut self, value: impl Into<String>) -> Self {
        self.debit_account_code = Some(value.into());
        self
    }

    pub fn credit_account_code(mut self, value: impl Into<String>) -> Self {
        self.credit_account_code = Some(value.into());
        self
    }

    pub fn vat_account_code(mut self, value: impl Into<String>) -> Self {
        self.vat_account_code = Some(value.into());
        self
    }

    pub fn expense_account_code(mut self, value: impl Into<String>) -> Self {
        self.expense_account_code = Some(value.into());
        self
    }

    pub fn advance_account_code(mut self, value: impl Into<String>) -> Self {
        self.advance_account_code = Some(value.into());
        self
    }

    pub fn income_account_code(mut self, value: impl Into<String>) -> Self {
        self.income_account_code = Some(value.into());
        self
    }

    pub fn is_purchase(mut self, value: bool) -> Self {
        self.is_purchase = Some(value);
        self
    }

    pub fn is_sale(mut self, value: bool) -> Self {
        self.is_sale = Some(value);
        self
    }

    pub fn is_write_off(mut self, value: bool) -> Self {
        self.is_write_off = Some(value);
        self
    }

    pub fn is_internal_movement(mut self, value: bool) -> Self {
        self.is_internal_movement = Some(value);
        self
    }

    pub fn is_purchase_return(mut self, value: bool) -> Self {
        self.is_purchase_return = Some(value);
        self
    }

    pub fn is_sales_return(mut self, value: bool) -> Self {
        self.is_sales_return = Some(value);
        self
    }

    pub fn is_consignment(mut self, value: bool) -> Self {
        self.is_consignment = Some(value);
        self
    }

    pub fn is_production(mut self, value: bool) -> Self {
        self.is_production = Some(value);
        self
    }

    pub fn is_asset_in(mut self, value: bool) -> Self {
        self.is_asset_in = Some(value);
        self
    }

    pub fn is_asset_out(mut self, value: bool) -> Self {
        self.is_asset_out = Some(value);
        self
    }

    pub fn is_cash_register_sale(mut self, value: bool) -> Self {
        self.is_cash_register_sale = Some(value);
        self
    }

    pub fn include_in_vat_register(mut self, value: bool) -> Self {
        self.include_in_vat_register = Some(value);
        self
    }

    pub fn include_in_saft(mut self, value: bool) -> Self {
        self.include_in_saft = Some(value);
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn sort_order(mut self, value: i64) -> Self {
        self.sort_order = Some(value);
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

    /// Consumes the builder and constructs a [`PostV1OperationTypesCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1OperationTypesCreateResponseBuilder::id)
    /// - [`code`](PostV1OperationTypesCreateResponseBuilder::code)
    /// - [`name`](PostV1OperationTypesCreateResponseBuilder::name)
    /// - [`is_purchase`](PostV1OperationTypesCreateResponseBuilder::is_purchase)
    /// - [`is_sale`](PostV1OperationTypesCreateResponseBuilder::is_sale)
    /// - [`is_write_off`](PostV1OperationTypesCreateResponseBuilder::is_write_off)
    /// - [`is_internal_movement`](PostV1OperationTypesCreateResponseBuilder::is_internal_movement)
    /// - [`is_purchase_return`](PostV1OperationTypesCreateResponseBuilder::is_purchase_return)
    /// - [`is_sales_return`](PostV1OperationTypesCreateResponseBuilder::is_sales_return)
    /// - [`is_consignment`](PostV1OperationTypesCreateResponseBuilder::is_consignment)
    /// - [`is_production`](PostV1OperationTypesCreateResponseBuilder::is_production)
    /// - [`is_asset_in`](PostV1OperationTypesCreateResponseBuilder::is_asset_in)
    /// - [`is_asset_out`](PostV1OperationTypesCreateResponseBuilder::is_asset_out)
    /// - [`is_cash_register_sale`](PostV1OperationTypesCreateResponseBuilder::is_cash_register_sale)
    /// - [`include_in_vat_register`](PostV1OperationTypesCreateResponseBuilder::include_in_vat_register)
    /// - [`include_in_saft`](PostV1OperationTypesCreateResponseBuilder::include_in_saft)
    /// - [`is_active`](PostV1OperationTypesCreateResponseBuilder::is_active)
    /// - [`sort_order`](PostV1OperationTypesCreateResponseBuilder::sort_order)
    /// - [`created_at`](PostV1OperationTypesCreateResponseBuilder::created_at)
    /// - [`updated_at`](PostV1OperationTypesCreateResponseBuilder::updated_at)
    pub fn build(self) -> Result<PostV1OperationTypesCreateResponse, BuildError> {
        Ok(PostV1OperationTypesCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            invoice_type: self.invoice_type,
            payer_partner_id: self.payer_partner_id,
            debit_account_code: self.debit_account_code,
            credit_account_code: self.credit_account_code,
            vat_account_code: self.vat_account_code,
            expense_account_code: self.expense_account_code,
            advance_account_code: self.advance_account_code,
            income_account_code: self.income_account_code,
            is_purchase: self
                .is_purchase
                .ok_or_else(|| BuildError::missing_field("is_purchase"))?,
            is_sale: self
                .is_sale
                .ok_or_else(|| BuildError::missing_field("is_sale"))?,
            is_write_off: self
                .is_write_off
                .ok_or_else(|| BuildError::missing_field("is_write_off"))?,
            is_internal_movement: self
                .is_internal_movement
                .ok_or_else(|| BuildError::missing_field("is_internal_movement"))?,
            is_purchase_return: self
                .is_purchase_return
                .ok_or_else(|| BuildError::missing_field("is_purchase_return"))?,
            is_sales_return: self
                .is_sales_return
                .ok_or_else(|| BuildError::missing_field("is_sales_return"))?,
            is_consignment: self
                .is_consignment
                .ok_or_else(|| BuildError::missing_field("is_consignment"))?,
            is_production: self
                .is_production
                .ok_or_else(|| BuildError::missing_field("is_production"))?,
            is_asset_in: self
                .is_asset_in
                .ok_or_else(|| BuildError::missing_field("is_asset_in"))?,
            is_asset_out: self
                .is_asset_out
                .ok_or_else(|| BuildError::missing_field("is_asset_out"))?,
            is_cash_register_sale: self
                .is_cash_register_sale
                .ok_or_else(|| BuildError::missing_field("is_cash_register_sale"))?,
            include_in_vat_register: self
                .include_in_vat_register
                .ok_or_else(|| BuildError::missing_field("include_in_vat_register"))?,
            include_in_saft: self
                .include_in_saft
                .ok_or_else(|| BuildError::missing_field("include_in_saft"))?,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            sort_order: self
                .sort_order
                .ok_or_else(|| BuildError::missing_field("sort_order"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
