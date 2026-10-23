use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct EcommerceClient {
    pub http_client: HttpClient,
}

impl EcommerceClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create a sales order from an online channel in status `new`. The customer is `partnerId` or `partner` details: with an `email` the existing partner with that e-mail is reused, otherwise a new partner is created; without either the call is refused with 422. `currency` defaults to the company's invoice currency, and `shipToCountryCode` and `marketplace` are kept for the VAT treatment of the invoice at fulfilment. A second order with the same `channel` and `externalRef` is refused with 409. Emits `sales_order.created`.
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
        request: &OrdersCreateEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersCreateEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ecommerce/orders/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read one sales order with its lines.
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
        request: &OrdersGetEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersGetEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ecommerce/orders/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List sales orders without their lines. Filterable by `channel`, `externalRef`, `partnerId` and `status`; sortable by `createdAt`.
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
        request: &OrdersListEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersListEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ecommerce/orders/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Reserve stock for a `new` order in `warehouseId` (default: the order's warehouse; 422 when neither is set). Each product line, and each product component of a set line, is checked against the warehouse's on-hand quantity less the existing reservations; when any item is short the call is refused with 422 and nothing is reserved. Otherwise a reservation is written per stock line, `stock.changed` (change type `reserve`) is emitted for each, and the order becomes `reserved`. An order in another status is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn orders_reserve(
        &self,
        request: &OrdersReserveEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersReserveEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ecommerce/orders/reserve",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Fulfil a `new` or `reserved` order: its reservations are released, the stock lines are consumed FIFO from the order's warehouse on `date` (default: the company's today), the cost of goods sold is posted as a journal entry debiting `cogsAccountCode` and crediting the stock accounts of the consumed layers (`inventoryAccountCode` when a layer has none), and a draft sale invoice for the order lines is created and linked as `invoiceId`, ready for `sales/invoices/issue`. With a `shipToCountryCode` on the order the invoice's VAT scheme is resolved for goods shipped there, as a marketplace sale when `marketplace` is set. Account codes default to the company's chart. Refused with 422 when the stock lines have no warehouse or stock is insufficient (unless the company allows negative stock), with 409 when the accounting period is locked or the order is already `fulfilled` or `cancelled`. Emits `sales_order.fulfilled` and `stock.changed`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn orders_fulfill(
        &self,
        request: &OrdersFulfillEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersFulfillEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ecommerce/orders/fulfill",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Cancel an order that is not fulfilled: its stock reservations are released, `stock.changed` (change type `release`) is emitted for each, and the status becomes `cancelled`. A fulfilled order is refused with 409.
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
        request: &OrdersCancelEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersCancelEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ecommerce/orders/cancel",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the company's items as a shop catalogue, in order of last update, with price and stock. `price` is the item's price in `priceListId` when given, else its own sale price, and `currency` that of the price list, else the company's invoice currency; `vatRatePercent` falls back to the rate of the item's VAT classifier. For a product `onHand`, `reserved` and `available` are summed over all warehouses or over `warehouseId` only; for a set `available` is the number of sets its product components can still make; for a service they are null. With `updatedSince` only items changed after that time are returned, deleted ones included (`deleted: true`), so a shop can sync incrementally; without it only live items are listed.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn products_list(
        &self,
        request: &ProductsListEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ProductsListEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ecommerce/products/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Stock position per item and warehouse: `onHand`, `reserved` and their difference `available`, for all warehouses or `warehouseId` only. Only items with a positive quantity on hand are returned.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn stock_list(
        &self,
        request: &StockListEcommerceRequest,
        options: Option<RequestOptions>,
    ) -> Result<StockListEcommerceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/ecommerce/stock/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
