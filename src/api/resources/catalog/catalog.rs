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

    pub async fn items_create(
        &self,
        request: &ItemsCreateCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<ItemsCreateCatalogResponse, ApiError> {
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

    pub async fn items_get(
        &self,
        request: &ItemsGetCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<ItemsGetCatalogResponse, ApiError> {
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

    pub async fn items_update(
        &self,
        request: &ItemsUpdateCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<ItemsUpdateCatalogResponse, ApiError> {
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

    pub async fn items_delete(
        &self,
        request: &ItemsDeleteCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<ItemsDeleteCatalogResponse, ApiError> {
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

    pub async fn items_list(
        &self,
        request: &ItemsListCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<ItemsListCatalogResponse, ApiError> {
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

    pub async fn items_files_list(
        &self,
        request: &ItemsFilesListCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<ItemsFilesListCatalogResponse, ApiError> {
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

    pub async fn items_kinds_create(
        &self,
        request: &ItemsKindsCreateCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<ItemsKindsCreateCatalogResponse, ApiError> {
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

    pub async fn items_kinds_update(
        &self,
        request: &ItemsKindsUpdateCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<ItemsKindsUpdateCatalogResponse, ApiError> {
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

    pub async fn items_kinds_delete(
        &self,
        request: &ItemsKindsDeleteCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<ItemsKindsDeleteCatalogResponse, ApiError> {
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

    pub async fn items_kinds_list(
        &self,
        request: &ItemsKindsListCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<ItemsKindsListCatalogResponse, ApiError> {
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

    pub async fn units_create(
        &self,
        request: &UnitsCreateCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<UnitsCreateCatalogResponse, ApiError> {
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

    pub async fn units_update(
        &self,
        request: &UnitsUpdateCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<UnitsUpdateCatalogResponse, ApiError> {
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

    pub async fn units_delete(
        &self,
        request: &UnitsDeleteCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<UnitsDeleteCatalogResponse, ApiError> {
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

    pub async fn units_list(
        &self,
        request: &UnitsListCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<UnitsListCatalogResponse, ApiError> {
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

    pub async fn units_options(
        &self,
        request: &UnitsOptionsCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<UnitsOptionsCatalogResponse, ApiError> {
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

    pub async fn item_groups_create(
        &self,
        request: &ItemGroupsCreateCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<ItemGroupsCreateCatalogResponse, ApiError> {
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

    pub async fn item_groups_update(
        &self,
        request: &ItemGroupsUpdateCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<ItemGroupsUpdateCatalogResponse, ApiError> {
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

    pub async fn item_groups_delete(
        &self,
        request: &ItemGroupsDeleteCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<ItemGroupsDeleteCatalogResponse, ApiError> {
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

    pub async fn item_groups_list(
        &self,
        request: &ItemGroupsListCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<ItemGroupsListCatalogResponse, ApiError> {
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

    pub async fn items_suppliers_upsert(
        &self,
        request: &ItemsSuppliersUpsertCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<ItemsSuppliersUpsertCatalogResponse, ApiError> {
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

    pub async fn items_suppliers_list(
        &self,
        request: &ItemsSuppliersListCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<ItemsSuppliersListCatalogResponse, ApiError> {
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

    pub async fn items_suppliers_delete(
        &self,
        request: &ItemsSuppliersDeleteCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<ItemsSuppliersDeleteCatalogResponse, ApiError> {
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

    pub async fn price_lists_create(
        &self,
        request: &PriceListsCreateCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<PriceListsCreateCatalogResponse, ApiError> {
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

    pub async fn price_lists_update(
        &self,
        request: &PriceListsUpdateCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<PriceListsUpdateCatalogResponse, ApiError> {
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

    pub async fn price_lists_list(
        &self,
        request: &PriceListsListCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<PriceListsListCatalogResponse, ApiError> {
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

    pub async fn price_lists_items_set(
        &self,
        request: &PriceListsItemsSetCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<PriceListsItemsSetCatalogResponse, ApiError> {
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

    pub async fn price_lists_items_list(
        &self,
        request: &PriceListsItemsListCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<PriceListsItemsListCatalogResponse, ApiError> {
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

    pub async fn price_lists_items_delete(
        &self,
        request: &PriceListsItemsDeleteCatalogRequest,
        options: Option<RequestOptions>,
    ) -> Result<PriceListsItemsDeleteCatalogResponse, ApiError> {
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
