use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct CatalogClient {
    pub http_client: HttpClient,
}

impl CatalogClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn post_v1_catalog_items_create(
        &self,
        request: &PostV1CatalogItemsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogItemsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/items/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_items_get(
        &self,
        request: &PostV1CatalogItemsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogItemsGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/items/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_items_update(
        &self,
        request: &PostV1CatalogItemsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogItemsUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/items/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_items_delete(
        &self,
        request: &PostV1CatalogItemsDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogItemsDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/items/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_items_list(
        &self,
        request: &PostV1CatalogItemsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogItemsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/items/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_items_files_list(
        &self,
        request: &PostV1CatalogItemsFilesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogItemsFilesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/items/files/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_items_kinds_create(
        &self,
        request: &PostV1CatalogItemsKindsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogItemsKindsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/items/kinds/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_items_kinds_update(
        &self,
        request: &PostV1CatalogItemsKindsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogItemsKindsUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/items/kinds/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_items_kinds_delete(
        &self,
        request: &PostV1CatalogItemsKindsDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogItemsKindsDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/items/kinds/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_items_kinds_list(
        &self,
        request: &PostV1CatalogItemsKindsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogItemsKindsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/items/kinds/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_units_create(
        &self,
        request: &PostV1CatalogUnitsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogUnitsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/units/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_units_update(
        &self,
        request: &PostV1CatalogUnitsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogUnitsUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/units/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_units_delete(
        &self,
        request: &PostV1CatalogUnitsDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogUnitsDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/units/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_units_list(
        &self,
        request: &PostV1CatalogUnitsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogUnitsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/units/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_units_options(
        &self,
        request: &PostV1CatalogUnitsOptionsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogUnitsOptionsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/units/options",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_item_groups_create(
        &self,
        request: &PostV1CatalogItemGroupsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogItemGroupsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/item-groups/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_item_groups_update(
        &self,
        request: &PostV1CatalogItemGroupsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogItemGroupsUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/item-groups/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_item_groups_delete(
        &self,
        request: &PostV1CatalogItemGroupsDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogItemGroupsDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/item-groups/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_item_groups_list(
        &self,
        request: &PostV1CatalogItemGroupsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogItemGroupsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/item-groups/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_items_suppliers_upsert(
        &self,
        request: &PostV1CatalogItemsSuppliersUpsertRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogItemsSuppliersUpsertResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/items/suppliers/upsert",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_items_suppliers_list(
        &self,
        request: &PostV1CatalogItemsSuppliersListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogItemsSuppliersListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/items/suppliers/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_items_suppliers_delete(
        &self,
        request: &PostV1CatalogItemsSuppliersDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogItemsSuppliersDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/items/suppliers/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_price_lists_create(
        &self,
        request: &PostV1CatalogPriceListsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogPriceListsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/price-lists/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_price_lists_update(
        &self,
        request: &PostV1CatalogPriceListsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogPriceListsUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/price-lists/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_price_lists_list(
        &self,
        request: &PostV1CatalogPriceListsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogPriceListsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/price-lists/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_price_lists_items_set(
        &self,
        request: &PostV1CatalogPriceListsItemsSetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogPriceListsItemsSetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/price-lists/items/set",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_price_lists_items_list(
        &self,
        request: &PostV1CatalogPriceListsItemsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogPriceListsItemsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/price-lists/items/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_catalog_price_lists_items_delete(
        &self,
        request: &PostV1CatalogPriceListsItemsDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CatalogPriceListsItemsDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/catalog/price-lists/items/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
