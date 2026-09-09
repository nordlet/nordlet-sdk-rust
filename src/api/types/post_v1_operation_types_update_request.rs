pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1OperationTypesUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "invoiceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_type: Option<PostV1OperationTypesUpdateRequestInvoiceType>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_purchase: Option<bool>,
    #[serde(rename = "isSale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sale: Option<bool>,
    #[serde(rename = "isWriteOff")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_write_off: Option<bool>,
    #[serde(rename = "isInternalMovement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_internal_movement: Option<bool>,
    #[serde(rename = "isPurchaseReturn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_purchase_return: Option<bool>,
    #[serde(rename = "isSalesReturn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sales_return: Option<bool>,
    #[serde(rename = "isConsignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_consignment: Option<bool>,
    #[serde(rename = "isProduction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_production: Option<bool>,
    #[serde(rename = "isAssetIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_asset_in: Option<bool>,
    #[serde(rename = "isAssetOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_asset_out: Option<bool>,
    #[serde(rename = "isCashRegisterSale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cash_register_sale: Option<bool>,
    #[serde(rename = "includeInVatRegister")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_in_vat_register: Option<bool>,
    #[serde(rename = "includeInSaft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_in_saft: Option<bool>,
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i64>,
}

impl PostV1OperationTypesUpdateRequest {
    pub fn builder() -> PostV1OperationTypesUpdateRequestBuilder {
        <PostV1OperationTypesUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1OperationTypesUpdateRequestBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    invoice_type: Option<PostV1OperationTypesUpdateRequestInvoiceType>,
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
}

impl PostV1OperationTypesUpdateRequestBuilder {
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

    pub fn invoice_type(mut self, value: PostV1OperationTypesUpdateRequestInvoiceType) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1OperationTypesUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1OperationTypesUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1OperationTypesUpdateRequest, BuildError> {
        Ok(PostV1OperationTypesUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code,
            name: self.name,
            invoice_type: self.invoice_type,
            payer_partner_id: self.payer_partner_id,
            debit_account_code: self.debit_account_code,
            credit_account_code: self.credit_account_code,
            vat_account_code: self.vat_account_code,
            expense_account_code: self.expense_account_code,
            advance_account_code: self.advance_account_code,
            income_account_code: self.income_account_code,
            is_purchase: self.is_purchase,
            is_sale: self.is_sale,
            is_write_off: self.is_write_off,
            is_internal_movement: self.is_internal_movement,
            is_purchase_return: self.is_purchase_return,
            is_sales_return: self.is_sales_return,
            is_consignment: self.is_consignment,
            is_production: self.is_production,
            is_asset_in: self.is_asset_in,
            is_asset_out: self.is_asset_out,
            is_cash_register_sale: self.is_cash_register_sale,
            include_in_vat_register: self.include_in_vat_register,
            include_in_saft: self.include_in_saft,
            is_active: self.is_active,
            sort_order: self.sort_order,
        })
    }
}
