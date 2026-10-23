use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PurchasesClient {
    pub http_client: HttpClient,
}

impl PurchasesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create a draft purchase invoice or credit note (`type`) from a supplier document; it is posted with `purchases/invoices/register`. Lines without a unit price take the item's supplier or purchase price, `currency` defaults to the company's invoice currency, `documentRef` links an uploaded file, and the gross total leaves out self-assessed VAT. `creditedInvoiceId` (credit notes only) must be an invoice of the same partner, `purchaseOrderId` an order of the same partner that is not draft, submitted or cancelled, and `operationTypeId` an active purchase operation type, each 422 otherwise; the same `documentNumber` for the same partner is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn invoices_create(
        &self,
        request: &InvoicesCreatePurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesCreatePurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/invoices/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Return one purchase invoice or credit note with its lines.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn invoices_get(
        &self,
        request: &InvoicesGetPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesGetPurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/invoices/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Change a draft purchase invoice (409 once registered); only the fields sent are changed, and `lines` replace all existing lines with the totals recomputed. `purchaseOrderId` and `operationTypeId` are checked as on `purchases/invoices/create` (422). A `documentNumber` already used for the partner is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn invoices_update(
        &self,
        request: &InvoicesUpdatePurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesUpdatePurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/invoices/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Mark a draft purchase invoice as deleted (409 once registered). A document capture linked to it goes back to `extracted` (or `pending` when it has no extraction) and can be confirmed again.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn invoices_delete(
        &self,
        request: &InvoicesDeletePurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesDeletePurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/invoices/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Register a draft purchase invoice or credit note: posts the journal entry dated `registrationDate` (default the document date) and sets the status to `registered`. The entry debits each line's expense account (the line's `accountCode`, else the item's purchase or expense account, else the operation type's expense account, else the `purchases.goodsForResale` or `purchases.defaultExpense` rule account), debits VAT receivable (a company that is not a VAT payer books the VAT into the expense instead), credits payables (the operation type's credit account when set) and credits output VAT for self-assessed VAT; a credit note posts the reverse. Lines with `deferralStartDate` and `deferralEndDate` are debited to the `purchases.prepaidExpenses` account instead and get a monthly release schedule for `purchases/deferrals/post`. With `warehouseId` the product lines of an invoice are received into stock at their base-currency cost (422 on an order-linked invoice, whose goods arrive through `purchases/receipts/create`, and for a line that is also deferred); `returnFromStock` on a credit note that credits an invoice takes the goods back out of stock at their FIFO cost and books the difference to the credited amount on the `inventory.cogs` account (422 on an invoice, without `creditedInvoiceId`, or when no goods of the credited invoice are in stock). A credit note that credits a registered invoice in the same currency is applied to it as a payment up to the open amount. Refused with 409 when already registered or the accounting period is locked, and with 422 when the credit notes would exceed the credited invoice, a posting account is missing or no exchange rate exists for the date. Emits `purchase_invoice.registered`, `purchase_invoice.paid` when the credited invoice becomes fully paid, and `stock.changed` when stock moves.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn invoices_register(
        &self,
        request: &InvoicesRegisterPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesRegisterPurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/invoices/register",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the monthly release rows that `purchases/invoices/register` creates for lines booked as prepaid expenses, each with its amount, expense and prepaid account and `status` (`pending`, `posted`, `cancelled`). Filterable by `invoiceId`, `status` and `scheduleDate`; sortable by `scheduleDate`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn deferrals_list(
        &self,
        request: &DeferralsListPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeferralsListPurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/deferrals/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Post every `pending` prepaid-expense release row dated on or before `asOfDate` (default today in the company's time zone): one journal entry per schedule date debiting the expense account and crediting the prepaid account, and the rows become `posted`. A locked accounting period is refused with 409. The scheduled job `expense-deferrals` runs the same release for every active company.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn deferrals_post(
        &self,
        request: &DeferralsPostPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeferralsPostPurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/deferrals/post",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List purchase invoices and credit notes without lines, with the supplier's `partnerName`. Filterable by `partnerId`, `type`, `status`, `paymentStatus`, `documentNumber`, `documentDate`, `currency`, `operationTypeId`, `purchaseOrderId`, `grossTotal`, `netTotal`, `vatTotal` and `dueDate`; sortable by `createdAt`, `documentDate`, `documentNumber`, `grossTotal` and `dueDate`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn invoices_list(
        &self,
        request: &InvoicesListPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesListPurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/invoices/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create a draft purchase order for a supplier. Lines without a unit price take the item's supplier or purchase price; `orderNumber` comes from the `purchase_order` series with prefix PO when not given; `currency` defaults to the company's invoice currency; `documentRef` links an uploaded file to the order. A number already in use is refused with 409. The order continues with `purchases/orders/submit`, `purchases/orders/approve` and `purchases/receipts/create`.
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
        request: &OrdersCreatePurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersCreatePurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/orders/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Change a draft purchase order (409 once it is submitted); only the fields sent are changed. `lines` replace all existing lines and the totals are recomputed.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn orders_update(
        &self,
        request: &OrdersUpdatePurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersUpdatePurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/orders/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Return one purchase order with its lines; each line carries `receivedQty` and `remainingQty`.
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
        request: &OrdersGetPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersGetPurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/orders/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List purchase orders without lines, with the supplier's `partnerName`. Filterable by `partnerId`, `status`, `orderNumber`, `orderDate`, `currency` and `grossTotal`; sortable by `createdAt`, `orderDate`, `orderNumber` and `grossTotal`.
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
        request: &OrdersListPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersListPurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/orders/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Move a `draft` purchase order to `submitted` for approval (409 from any other status). `reason` is kept in the audit log.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn orders_submit(
        &self,
        request: &OrdersSubmitPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersSubmitPurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/orders/submit",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Approve a `submitted` purchase order (409 from any other status), recording who approved it and when; needs the `purchases:approve` scope. Goods are then received with `purchases/receipts/create`. Emits `purchase_order.approved`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn orders_approve(
        &self,
        request: &OrdersApprovePurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersApprovePurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/orders/approve",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Send a `submitted` purchase order back to `draft` (409 from any other status); needs the `purchases:approve` scope. `reason` is kept in the audit log.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn orders_reject(
        &self,
        request: &OrdersRejectPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersRejectPurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/orders/reject",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Cancel a `draft`, `submitted` or `approved` purchase order (409 from any other status, or when the order already has a goods receipt).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn orders_cancel(
        &self,
        request: &OrdersCancelPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersCancelPurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/orders/cancel",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Close an `approved`, `partially_received` or `received` purchase order (409 from any other status); a closed order accepts no further goods receipts.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn orders_close(
        &self,
        request: &OrdersClosePurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersClosePurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/orders/close",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Mark a `draft` purchase order as deleted (409 once it is submitted).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn orders_delete(
        &self,
        request: &OrdersDeletePurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersDeletePurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/orders/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Record a goods receipt against an `approved` or `partially_received` purchase order (409 otherwise), numbered from the `goods_receipt` series with prefix GR. Lines whose item is a product go into stock in `warehouseId`, else the order's warehouse (422 when neither is set), at their share of the order line's net amount converted at the receipt date's rate; `lotNumber` and `expiryDate` receive them into a lot, and an item that needs lot or serial tracking is refused with 422 without one. A quantity above the line's remaining quantity is refused with 409; the order becomes `received` when every line is fully received, else `partially_received`. Emits `goods_receipt.posted`, `stock.changed` and, when fully received, `purchase_order.received`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn receipts_create(
        &self,
        request: &ReceiptsCreatePurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceiptsCreatePurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/receipts/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Return one goods receipt with its lines and their stock movement ids.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn receipts_get(
        &self,
        request: &ReceiptsGetPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceiptsGetPurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/receipts/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List goods receipts without lines. Filterable by `orderId`, `warehouseId`, `receiptDate` and `receiptNumber`; sortable by `createdAt` and `receiptDate`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn receipts_list(
        &self,
        request: &ReceiptsListPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceiptsListPurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/receipts/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Compare a purchase invoice with its linked purchase order and the goods received so far, grouped by item (by description for lines without one). Each row is `matched`, `not_received` (invoiced but nothing received), `over_invoiced` (invoiced more than received), `price_mismatch` (unit price differs by more than `priceTolerancePercent`), `not_on_order` or `not_invoiced`; `status` is `mismatched` when any row other than `not_invoiced` is not matched. Refused with 422 when the invoice has no `purchaseOrderId` or its currency differs from the order's. Nothing is changed.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn invoices_match(
        &self,
        request: &InvoicesMatchPurchasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicesMatchPurchasesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/purchases/invoices/match",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
