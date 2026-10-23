use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct InventoryClient {
    pub http_client: HttpClient,
}

impl InventoryClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Return the company's negative stock policy.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn settings_get(
        &self,
        request: &SettingsGetInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettingsGetInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/settings/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Set the negative stock policy. With `reject` a consumption larger than the stock on hand of the item in the warehouse is refused with 422. With `allow` the missing quantity is written as a shortage priced at the unit cost of the last receipt in that warehouse (the item's purchase price when there is none); the next receipt of the item there settles the shortage and posts any cost difference between `inventory.cogs` and `inventory.stock`. A consumption from a named lot is refused with 422 under both policies.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn settings_update(
        &self,
        request: &SettingsUpdateInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettingsUpdateInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/settings/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create a warehouse. 409 when the code already exists.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn warehouses_create(
        &self,
        request: &WarehousesCreateInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WarehousesCreateInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/warehouses/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the warehouses. Filterable and sortable: code, name.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn warehouses_list(
        &self,
        request: &WarehousesListInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WarehousesListInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/warehouses/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Change the name or the country code of a warehouse.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn warehouses_update(
        &self,
        request: &WarehousesUpdateInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WarehousesUpdateInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/warehouses/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Record a manual receipt of an item into a warehouse at `unitCost` (document type `manual_receipt`); the quantity must be positive (422). A lot- or serial-tracked item needs `lotNumber` (422); an unknown lot is created, and `expiryDate` is stored on a lot that has none. A serial-tracked item is received one unit at a time (422), and 409 when that serial number is already in stock. Open shortages of the item in the warehouse (negative stock policy `allow`) are settled from the receipt. Emits `stock.changed`. Receipts against a purchase order go through `purchases/receipts/create`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn stock_receive(
        &self,
        request: &StockReceiveInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<StockReceiveInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/stock/receive",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Consume a quantity of an item from a warehouse, oldest layers first (from one lot when `lotNumber` is given), and post a journal entry: debit the expense account (`expenseAccountCode`, default the chart's inventory expense account), credit the stock account each consumed layer was received on (the chart's inventory account for layers without one), or `inventoryAccountCode` for the whole cost when given. 409 when the accounting period of `date` is locked; 422 when the stock is insufficient under the `reject` policy or in the named lot. Emits `stock.changed` and, when the consumption takes the available quantity from above a reorder rule's minimum to at or below it, `stock.reorder_needed`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn stock_write_off(
        &self,
        request: &StockWriteOffInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<StockWriteOffInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/stock/write-off",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Move a quantity of an item from one warehouse to another: the source layers are consumed oldest first (from one lot when `lotNumber` is given) and received in the target with the same quantities, costs, lots and stock accounts (document type `warehouse_transfer`). 422 when both warehouses are the same or the source stock is insufficient under the `reject` policy. `inMovementId` is the first receipt when the consumption spans several lots or stock accounts. Emits `stock.changed` for both warehouses and `stock.reorder_needed` as `inventory/stock/write-off` does.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn stock_transfer(
        &self,
        request: &StockTransferInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<StockTransferInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/stock/transfer",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Post a stock count for a warehouse. Each line names an item by `itemId` or `barcode` with its `countedQty`; the difference to the quantity on hand in the warehouse (all lots together) is written off oldest layers first or received as a surplus at `unitCost` (the item's purchase price when omitted; 422 when neither is set), with document type `stock_take`, and a surplus of a lot- or serial-tracked item needs `lotNumber` (422). The shortage cost less the surplus cost is posted in one journal entry between the expense account (`expenseAccountCode`, default the chart's inventory expense account) and the stock accounts (`inventoryAccountCode` for everything when given, else the accounts of the consumed layers and the chart's inventory account for the surpluses); nothing is posted and `journalTransactionId` is null when the net is zero. 409 when the accounting period of `date` is locked. Emits `stock.changed` per adjusted item.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn stock_take(
        &self,
        request: &StockTakeInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<StockTakeInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/stock/take",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Return the quantity and value on hand per item and warehouse, summed from the stock movements, optionally for one `warehouseId` and/or `itemId`. Rows with a zero quantity are left out.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn stock_levels(
        &self,
        request: &StockLevelsInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<StockLevelsInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/stock/levels",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the stock movements. Filterable: itemId, warehouseId, lotId, direction, documentType, date, quantity, totalCost. Sortable: date, createdAt, quantity, totalCost; default newest first.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn stock_movements_list(
        &self,
        request: &StockMovementsListInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<StockMovementsListInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/stock/movements/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the lots with their quantity on hand (the remaining quantity of the lot's incoming movements). Filterable: itemId, lotNumber, expiryDate. Sortable: lotNumber, expiryDate, createdAt; default newest first.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn lots_list(
        &self,
        request: &LotsListInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<LotsListInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/lots/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Return a lot with its quantity on hand and its movements in date order.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn lots_get(
        &self,
        request: &LotsGetInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<LotsGetInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/lots/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Change the expiry date or the notes of a lot; null clears the value.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn lots_update(
        &self,
        request: &LotsUpdateInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<LotsUpdateInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/lots/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Allocate an extra cost `amount` (positive, whole cents; 422 otherwise) over incoming stock movements, named by `movementIds` or taken from the lines of a goods receipt (`goodsReceiptId`; 422 when the receipt has no stock movements), in proportion to their cost (`by_value`) or quantity (`by_quantity`), largest remainder to the cent. The part of each share that belongs to the quantity still on hand raises the movement's remaining cost and recomputes its unit cost; the part that belongs to quantity already consumed is posted in a journal entry dated `date` debiting `inventory.cogs` and crediting the movement's stock account (`inventory.stock` when it has none), 409 when that period is locked. 422 when a movement is not incoming or the weights sum to zero. `sourceInvoiceId` records the purchase invoice the cost came from. Goods receipts come from `purchases/receipts/create`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn landed_costs_create(
        &self,
        request: &LandedCostsCreateInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<LandedCostsCreateInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/landed-costs/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Return a landed cost with its allocation lines and the journal transaction it posted, if any.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn landed_costs_get(
        &self,
        request: &LandedCostsGetInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<LandedCostsGetInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/landed-costs/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the landed costs without their lines. Filterable: date, method, goodsReceiptId, sourceInvoiceId. Sortable: date, createdAt, amount; default newest first.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn landed_costs_list(
        &self,
        request: &LandedCostsListInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<LandedCostsListInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/landed-costs/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create a reorder rule for an item, for one warehouse or, without `warehouseId`, for the item over all warehouses; one rule per item and warehouse (409). While the rule is active, an outgoing movement or a reservation that takes the available quantity (on hand less reserved) from above `minQty` to at or below it emits `stock.reorder_needed` with the `reorderQty`. `inventory/reorder-rules/check` lists the rules currently at or below their minimum.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn reorder_rules_create(
        &self,
        request: &ReorderRulesCreateInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReorderRulesCreateInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/reorder-rules/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Change the minimum quantity, the reorder quantity (null clears it), the active flag or the notes of a reorder rule.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn reorder_rules_update(
        &self,
        request: &ReorderRulesUpdateInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReorderRulesUpdateInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/reorder-rules/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a reorder rule.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn reorder_rules_delete(
        &self,
        request: &ReorderRulesDeleteInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReorderRulesDeleteInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/reorder-rules/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List the reorder rules. Filterable: itemId, warehouseId, isActive. Sortable: createdAt, minQty; default newest first.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn reorder_rules_list(
        &self,
        request: &ReorderRulesListInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReorderRulesListInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/reorder-rules/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Return the active reorder rules whose available quantity (on hand less reserved, in the rule's warehouse or over all warehouses) is at or below `minQty`, with the quantities. Emits no event.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn reorder_rules_check(
        &self,
        request: &ReorderRulesCheckInventoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReorderRulesCheckInventoryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/inventory/reorder-rules/check",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
