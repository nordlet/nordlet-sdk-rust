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
