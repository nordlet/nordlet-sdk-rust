use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct CashClient {
    pub http_client: HttpClient,
}

impl CashClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates a numbered cash receipt (series `KPO`) or cash disbursement (`KIO`) and posts it in the base currency between the cash account (`cashAccountCode`, default the cash account of the company's chart) and `counterAccountCode`; a receipt debits the cash account, a disbursement credits it. Linked to an invoice - `saleInvoiceId` for a receipt against an issued invoice or advance invoice or a disbursement against an issued credit note, `purchaseInvoiceId` for a disbursement against a registered purchase invoice - the counter account is the receivable or payable account of the posting rules, the invoice's paid amount and payment status (`partial` or `paid`) are updated, and `sale_invoice.paid` or `purchase_invoice.paid` is emitted when the invoice becomes fully paid. Answers 422 when both invoice ids are given, a receipt names a purchase invoice, `counterAccountCode` differs from the invoice's account, or the invoice is in another currency or belongs to another partner; 409 when `amount` exceeds the invoice's remaining amount or the accounting period of `date` is locked.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn orders_create(
        &self,
        request: &OrdersCreateCashRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersCreateCashResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/cash/orders/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns one cash order.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn orders_get(
        &self,
        request: &OrdersGetCashRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersGetCashResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/cash/orders/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists cash orders. Filterable by `type`, `date`, `partnerId`, `employeeId`, `fullNumber` and `amount`; sortable by `date`, `fullNumber`, `createdAt` and `amount`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn orders_list(
        &self,
        request: &OrdersListCashRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersListCashResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/cash/orders/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the balance of a cash account (default: the cash account of the company's chart) from the posted journal entries, up to and including `asOf` when given.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn balance(
        &self,
        request: &BalanceCashRequest,
        options: Option<RequestOptions>,
    ) -> Result<BalanceCashResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/cash/balance",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates a numbered expense report (series `AA`) for the employee and posts it: each line's net amount is debited to its `accountCode` and the VAT total to the input VAT account of the posting rules (`purchases.vatReceivable`), or the gross amount to the line account when the company is not a VAT payer, and the total is credited to the cash advances account (`cash.advances`), which settles the employee's advance balance shown by `cash/advance-holders/balances`. Answers 422 when the total is zero or a line account is missing from the chart or not postable, and 409 when the accounting period of `date` is locked.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn expense_reports_create(
        &self,
        request: &ExpenseReportsCreateCashRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExpenseReportsCreateCashResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/cash/expense-reports/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns one expense report with its lines.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn expense_reports_get(
        &self,
        request: &ExpenseReportsGetCashRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExpenseReportsGetCashResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/cash/expense-reports/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists expense reports without their lines. Filterable by `employeeId` and `date`; sortable by `date` and `number`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn expense_reports_list(
        &self,
        request: &ExpenseReportsListCashRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExpenseReportsListCashResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/cash/expense-reports/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns, per employee, the cash advances issued to them (disbursements posted to the `cash.advances` account with the employee set), the amounts returned (receipts on the same account), the total settled by expense reports and the open balance (issued less returned less settled).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn advance_holders_balances(
        &self,
        request: &AdvanceHoldersBalancesCashRequest,
        options: Option<RequestOptions>,
    ) -> Result<AdvanceHoldersBalancesCashResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/cash/advance-holders/balances",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
