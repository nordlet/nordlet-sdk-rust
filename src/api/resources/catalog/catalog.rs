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

    /// Create a catalog item: a product, a service or a set of other items. `unit` must be `vnt`, a global measurement unit or one of the company's own units (`catalog/units/options`), the sale, purchase and expense account codes must be postable accounts of the chart, and `priceFrom`/`priceTo` and `validFrom`/`validTo` must be in order; otherwise 422. A `set` needs at least one component and none of them may be a set; `components` on any other type is refused with 422. 409 when an active item already carries the `code`. `documentRef` is the storage key of a file uploaded with `files/upload`, which is attached to the item. Emits `item.created`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Return one item; a set comes with its components.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Change the fields of an item; the fields that are nullable in the request can be set to null to clear them. The checks of `catalog/items/create` apply. `components` replaces the whole component list and is accepted only when the item is or becomes a set (422); turning an item into a set needs components and is refused with 409 while the item is itself a component of another set. Turning a set into another type removes its components. 409 when an active item already carries the new `code`. Emits `item.updated`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Soft-delete an item. Refused with 409 while the item is a component of an active set. Emits `item.deleted`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List the active items. Filterable: name, code, barcode, type, vatClassifierCode, groupId, kindId, externalId, manufacturer, department. Sortable: name, code, createdAt, updatedAt, salePriceExclVat; default name ascending. Rows carry no components; `catalog/items/get` returns them.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List the files attached to an item, newest first.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Create an item kind: a code, a name, the SAF-T item type, whether quantities are accounted for, and a sort order. 409 when the code already exists.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Change an item kind. 409 when the new code already exists.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Delete an item kind. Refused with 409 while an active item uses it; soft-deleted items that still carry it are detached.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List the company's item kinds in sort order, then by code.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Create a unit of measure of the company's own, in addition to the global measurement units (`reference/units/list`). 409 when the code already exists.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Change the code, name or active flag of a company unit. 409 when the new code already exists.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Delete a company unit of measure.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List the company's own units of measure, ordered by code.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Return the unit codes an item can use: the company's active own units first, then the global measurement units, named in the requested `locale` (Lithuanian for `lt`, English otherwise). A company unit hides a global unit with the same code.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Create an item group, optionally under `parentId`. 409 when the code already exists.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Change the code, name or parent of an item group; `parentId` null moves it to the top level. 422 when the new parent is the group itself or one of its descendants. 409 when the new code already exists.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Delete an item group. Refused with 409 while it has child groups or active items assigned.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List the company's item groups, ordered by code.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Create or replace the supplier record of an item for a partner (one per item and partner): supplier code, purchase price excluding VAT, currency (EUR when omitted) and notes. Fields omitted from the request are cleared on an existing record.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List the supplier records, optionally for one `itemId` and/or `partnerId`, ordered by partner name.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Delete the supplier record of an item.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Create a price list (currency EUR and active when omitted). 409 when the code already exists.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Change a price list. 409 when the new code already exists.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List the company's price lists, ordered by code.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Set the unit price excluding VAT of up to 500 items on a price list; a price already on the list is replaced and the other items on the list are kept. 422 when an item appears twice in the request. Emits `price_list.updated` with the item ids.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List the prices of a price list with the item name and code, ordered by item name.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Remove one item from a price list.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
