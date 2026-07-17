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

    pub async fn post_v1_inventory_settings_get(
        &self,
        request: &PostV1InventorySettingsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1InventorySettingsGetResponse, ApiError> {
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

    pub async fn post_v1_inventory_settings_update(
        &self,
        request: &PostV1InventorySettingsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1InventorySettingsUpdateResponse, ApiError> {
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

    pub async fn post_v1_inventory_warehouses_create(
        &self,
        request: &PostV1InventoryWarehousesCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1InventoryWarehousesCreateResponse, ApiError> {
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

    pub async fn post_v1_inventory_warehouses_list(
        &self,
        request: &PostV1InventoryWarehousesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1InventoryWarehousesListResponse, ApiError> {
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

    pub async fn post_v1_inventory_stock_receive(
        &self,
        request: &PostV1InventoryStockReceiveRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1InventoryStockReceiveResponse, ApiError> {
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

    pub async fn post_v1_inventory_stock_write_off(
        &self,
        request: &PostV1InventoryStockWriteOffRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1InventoryStockWriteOffResponse, ApiError> {
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

    pub async fn post_v1_inventory_stock_transfer(
        &self,
        request: &PostV1InventoryStockTransferRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1InventoryStockTransferResponse, ApiError> {
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

    pub async fn post_v1_inventory_stock_take(
        &self,
        request: &PostV1InventoryStockTakeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1InventoryStockTakeResponse, ApiError> {
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

    pub async fn post_v1_inventory_stock_levels(
        &self,
        request: &PostV1InventoryStockLevelsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1InventoryStockLevelsResponse, ApiError> {
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

    pub async fn post_v1_inventory_stock_movements_list(
        &self,
        request: &PostV1InventoryStockMovementsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1InventoryStockMovementsListResponse, ApiError> {
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
}
