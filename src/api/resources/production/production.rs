use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ProductionClient {
    pub http_client: HttpClient,
}

impl ProductionClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create a work center with its `costPerHour`. `costAccountCode` must be a postable account of the chart (422 otherwise); when a production order completes, the labor cost of the operations run on this center is credited to it instead of the `production.laborApplied` posting rule account. A duplicate `code` is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn work_centers_create(
        &self,
        request: &WorkCentersCreateProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkCentersCreateProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/work-centers/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Change a work center; only the fields sent are changed. `costAccountCode` must be a postable account of the chart (422 otherwise); a `code` already used by another work center is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn work_centers_update(
        &self,
        request: &WorkCentersUpdateProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkCentersUpdateProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/work-centers/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List work centers. `nextMaintenanceDate` is the latest completed maintenance date (the creation date when there is none) plus `maintenanceIntervalDays`, null without an interval. Filterable by `code`, `name` and `isActive`; sortable by `code`, `name` and `createdAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn work_centers_list(
        &self,
        request: &WorkCentersListProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkCentersListProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/work-centers/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create a routing with its operations: `sequence`, work center, `setupMinutes`, `runMinutesPerUnit` and an optional `qualityCheckName` that becomes a pending quality check on every production order using the routing. Operation sequences must be unique (422); a duplicate `code` is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn routings_create(
        &self,
        request: &RoutingsCreateProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<RoutingsCreateProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/routings/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Return one routing with its operations in sequence order.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn routings_get(
        &self,
        request: &RoutingsGetProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<RoutingsGetProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/routings/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List routings without their operations. Filterable by `code`, `name` and `isActive`; sortable by `code`, `name` and `createdAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn routings_list(
        &self,
        request: &RoutingsListProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<RoutingsListProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/routings/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Record a `preventive` or `corrective` maintenance planned for a work center on `plannedDate`; it starts as `planned`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn maintenance_create(
        &self,
        request: &MaintenanceCreateProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<MaintenanceCreateProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/maintenance/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Mark a `planned` maintenance as completed on `completedDate`, with optional `downtimeHours` and `cost` (409 when it is already completed or cancelled). The work center's `nextMaintenanceDate` is then counted from the latest completed maintenance date.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn maintenance_complete(
        &self,
        request: &MaintenanceCompleteProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<MaintenanceCompleteProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/maintenance/complete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Cancel a `planned` maintenance (409 when it is already completed or cancelled).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn maintenance_cancel(
        &self,
        request: &MaintenanceCancelProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<MaintenanceCancelProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/maintenance/cancel",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List maintenance records. Filterable by `workCenterId`, `status` and `type`; sortable by `plannedDate` and `createdAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn maintenance_list(
        &self,
        request: &MaintenanceListProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<MaintenanceListProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/maintenance/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create a bill of materials for a finished item. `outputQuantity` is the finished quantity made per unit of order quantity; each line's `quantity` is the component consumed per unit of order quantity, increased by its `scrapPercent` at completion; `routingId` is the default routing of assembly orders. A BOM that consumes its own finished item is refused with 422, a duplicate `code` with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn boms_create(
        &self,
        request: &BomsCreateProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<BomsCreateProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/boms/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Return one bill of materials with its component lines.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn boms_get(
        &self,
        request: &BomsGetProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<BomsGetProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/boms/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List bills of materials without their lines. Filterable by `code`, `name`, `finishedItemId` and `isActive`; sortable by `code` and `name`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn boms_list(
        &self,
        request: &BomsListProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<BomsListProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/boms/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create a draft production order for `quantity` units of a BOM in a warehouse; stock moves only at `production/orders/complete`. An `assembly` order takes its routing from `routingId`, else from the BOM, and copies the routing's operations onto the order (`plannedMinutes` = setup minutes + run minutes per unit × finished quantity, priced at the work center's current `costPerHour`) with a pending quality check for every operation that names one. A `routingId` on a `disassembly` order, an inactive routing or a routing without operations is refused with 422.
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
        request: &OrdersCreateProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersCreateProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/orders/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Record the `actualMinutes` spent on one operation of a draft production order; at completion the actual minutes, else the planned ones, price the operation's labor cost. Refused with 409 once the order is completed.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn orders_record_operation(
        &self,
        request: &OrdersRecordOperationProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersRecordOperationProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/orders/record-operation",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Add a pending quality check to a draft production order (409 once the order is completed). The order cannot complete while a check is pending.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn quality_checks_add(
        &self,
        request: &QualityChecksAddProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<QualityChecksAddProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/quality-checks/add",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Record the result of a quality check as `passed` or `failed`, with the time and the actor (409 once the order is completed). Only a `pending` check blocks `production/orders/complete`; a `failed` one does not.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn quality_checks_record(
        &self,
        request: &QualityChecksRecordProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<QualityChecksRecordProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/quality-checks/record",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List quality checks. Filterable by `orderId`, `result` and `name`; sortable by `createdAt` and `name`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn quality_checks_list(
        &self,
        request: &QualityChecksListProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<QualityChecksListProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/quality-checks/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Complete a draft production order on the order's date. An `assembly` consumes the components FIFO from the warehouse (line quantity × order quantity, increased by `scrapPercent`), prices each operation at its actual or planned minutes × `costPerHour`, receives the finished item at material cost + labor cost less the share of `scrappedQuantity`, and, when there is labor, scrap or a component stock account other than the finished one, posts a journal entry debiting the finished stock account (and the `production.scrap` rule account for the scrapped share) and crediting the component stock accounts and the work centers' cost accounts (or the `production.laborApplied` rule account); a `disassembly` consumes the finished item and receives the components at its cost shared by quantity. `componentsAccountCode` and `finishedAccountCode` default to the chart's production accounts. Refused with 409 when the order is already completed, a quality check is still pending or the accounting period is locked, and with 422 when the BOM has no lines, `scrappedQuantity` is given on a disassembly or exceeds the finished quantity, the assembly cost is zero, stock is insufficient, a received item needs lot or serial tracking, or a posting account is missing. Emits `stock.changed`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn orders_complete(
        &self,
        request: &OrdersCompleteProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersCompleteProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/orders/complete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Return one production order with its operations and quality checks.
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
        request: &OrdersGetProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersGetProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/orders/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List production orders without operations and quality checks. Filterable by `bomId`, `warehouseId`, `status` and `type`; sortable by `date` and `createdAt`.
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
        request: &OrdersListProductionRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrdersListProductionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/production/orders/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
