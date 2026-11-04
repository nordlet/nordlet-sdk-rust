use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PosClient {
    pub http_client: HttpClient,
}

impl PosClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Register a cash register (POS device). The serial number must be unique in the company; a repeat is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn devices_create(
        &self,
        request: &DevicesCreatePosRequest,
        options: Option<RequestOptions>,
    ) -> Result<DevicesCreatePosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/devices/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Change a POS device's details or set `isActive`; an inactive device accepts no new shifts or reports. A serial number already used by another device is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn devices_update(
        &self,
        request: &DevicesUpdatePosRequest,
        options: Option<RequestOptions>,
    ) -> Result<DevicesUpdatePosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/devices/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List POS devices. Filterable by `name` and `serialNumber`; sortable by `name`, `serialNumber` and `createdAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn devices_list(
        &self,
        request: &DevicesListPosRequest,
        options: Option<RequestOptions>,
    ) -> Result<DevicesListPosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/devices/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Record a daily cash register report (Z-report) and post it to the ledger in one journal entry: the cash and card takings are debited to `cashAccountCode` and `cardAccountCode`, the net of the `vatLines` is credited to `revenueAccountCode` and their VAT to `vatAccountCode`. `itemLines` write the sold products off `warehouseId`, the first-expiring lot first (lots without an expiry date after those with one, then the oldest layers first), leaving out expired lots (a set is expanded to its product components) and add the cost of goods sold, debited to `cogsAccountCode` against the stock accounts. `cogsAccountCode` and `inventoryAccountCode` default to the posting rules `inventory.cogs` and `inventory.stock`, the other account codes to the company's chart. Refused with 422 when cash plus card differs from the gross total, the item lines have no warehouse or the device is inactive, with 409 when the accounting period is locked or the `reportNumber` already exists. Emits `pos_report.created` and, per item line, `stock.changed`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn reports_create(
        &self,
        request: &ReportsCreatePosRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReportsCreatePosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/reports/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read one POS report with its VAT lines.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn reports_get(
        &self,
        request: &ReportsGetPosRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReportsGetPosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/reports/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List POS reports without their VAT lines. Filterable by `date`, `reportNumber` and `deviceId`; sortable by `date`, `reportNumber` and `createdAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn reports_list(
        &self,
        request: &ReportsListPosRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReportsListPosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/reports/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Open a shift on a POS device with the cash float `openingCash`. A device has at most one open shift (409 for a second); an inactive device is refused with 422. Give `warehouseId` when the shift will sell stock items; they are written off that warehouse when the shift is closed.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn shifts_open(
        &self,
        request: &ShiftsOpenPosRequest,
        options: Option<RequestOptions>,
    ) -> Result<ShiftsOpenPosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/shifts/open",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read one POS shift.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn shifts_get(
        &self,
        request: &ShiftsGetPosRequest,
        options: Option<RequestOptions>,
    ) -> Result<ShiftsGetPosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/shifts/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List POS shifts. Filterable by `deviceId` and `status`; sortable by `openedAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn shifts_list(
        &self,
        request: &ShiftsListPosRequest,
        options: Option<RequestOptions>,
    ) -> Result<ShiftsListPosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/shifts/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Record a receipt in an open shift, numbered per shift from 1. Each line's net, VAT and gross come from its VAT-inclusive unit price; a line without `description` takes its item's name. Refused with 422 when cash plus card differs from the receipt total or a stock item is sold in a shift opened without a warehouse, with 409 when the shift is closed.
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
        request: &ReceiptsCreatePosRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceiptsCreatePosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/receipts/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List receipts without their lines. Filterable by `shiftId`; sortable by `number` and `createdAt`.
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
        request: &ReceiptsListPosRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceiptsListPosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/receipts/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read one receipt with its lines.
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
        request: &ReceiptsGetPosRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReceiptsGetPosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/receipts/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Refund `quantity` of lines of a receipt, paid in cash out of the open shift `shiftId`, on `date` (default: the company's today). A line refunded in part is priced from its VAT-inclusive unit price, the rest of a line at what is left of its amounts. One journal entry debits the chart's POS revenue and VAT accounts with the net and VAT and credits the chart's cash account with the gross. When the receipt's shift is closed, the stock lines go back into the warehouse at the cost they left it on that shift's report, debited to their stock accounts (`inventory.stock` for layers without one) against `inventory.cogs`; when it is still open, the refunded quantities are left out of the stock write-off at its close. A receipt of another open shift is refused with 422, as are a line not on the receipt, a line named twice, a zero quantity and a quantity above what is left to refund; 409 when the shift is closed or the accounting period is locked.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn refunds_create(
        &self,
        request: &RefundsCreatePosRequest,
        options: Option<RequestOptions>,
    ) -> Result<RefundsCreatePosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/refunds/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read one POS refund with its lines.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn refunds_get(
        &self,
        request: &RefundsGetPosRequest,
        options: Option<RequestOptions>,
    ) -> Result<RefundsGetPosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/refunds/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List POS refunds without their lines. Filterable by `shiftId` and `receiptId`; sortable by `date` and `createdAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn refunds_list(
        &self,
        request: &RefundsListPosRequest,
        options: Option<RequestOptions>,
    ) -> Result<RefundsListPosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/refunds/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Close an open shift: its receipts are summed by VAT rate and by product into one POS report, created as by `pos/reports/create` with number `reportNumber` (default `Z-<device serial>-<n>`, n being the device's closed shifts plus one) on `date` (default: the company's today), which posts the takings and the stock write-off to the ledger and emits `pos_report.created`. Quantities refunded from the shift's receipts by `pos/refunds/create` are left out of the stock write-off. The response adds `expectedCash` (opening cash plus the cash receipts less the cash refunds paid out of the shift) and `cashDifference` (`countedCash` less expected). A difference is posted in its own journal entry on `date` (document type `pos_shift`): a shortage debits the `pos.cashShortage` rule account and credits the chart's cash account, a surplus debits the cash account and credits `pos.cashSurplus`; 422 when the company's chart lacks the account. A shift without receipts is refused with 422, a closed one with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn shifts_close(
        &self,
        request: &ShiftsClosePosRequest,
        options: Option<RequestOptions>,
    ) -> Result<ShiftsClosePosResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pos/shifts/close",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
