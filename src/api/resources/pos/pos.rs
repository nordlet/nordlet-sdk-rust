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

    /// Record a daily cash register report (Z-report) and post it to the ledger in one journal entry: the cash and card takings are debited to `cashAccountCode` and `cardAccountCode`, the net of the `vatLines` is credited to `revenueAccountCode` and their VAT to `vatAccountCode`. `itemLines` write the sold products off `warehouseId` FIFO (a set is expanded to its product components) and add the cost of goods sold, debited to `cogsAccountCode` against the stock accounts. Account codes default to the company's chart. Refused with 422 when cash plus card differs from the gross total, the item lines have no warehouse or the device is inactive, with 409 when the accounting period is locked or the `reportNumber` already exists. Emits `pos_report.created` and, per item line, `stock.changed`.
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

    /// Close an open shift: its receipts are summed by VAT rate and by product into one POS report, created as by `pos/reports/create` with number `reportNumber` (default `Z-<device serial>-<n>`, n being the device's closed shifts plus one) on `date` (default: the company's today), which posts the takings and the stock write-off to the ledger and emits `pos_report.created`. The response adds `expectedCash` (opening cash plus the cash receipts) and `cashDifference` (`countedCash` less expected). A shift without receipts is refused with 422, a closed one with 409.
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
