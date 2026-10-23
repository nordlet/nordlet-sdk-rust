use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct OperationTypesClient {
    pub http_client: HttpClient,
}

impl OperationTypesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create an operation type: a `code` with the flags that say what it is for (`isSale`, `isPurchase`, ...), an optional `invoiceType` (`invoice`, `credit_note`, `proforma`, `advance`) and optional account codes. An invoice names it in `operationTypeId`: a sale invoice needs `isSale` and a matching `invoiceType`, a purchase invoice `isPurchase`, and the type's account codes then replace the default posting accounts when the invoice is issued (`sales/invoices/issue`) or registered (`purchases/invoices/register`). `includeInVatRegister: false` keeps the type's invoices out of the Lithuanian i.SAF and FR0600, `includeInSaft: false` out of the SAF-T file. Account codes must exist and be postable and `payerPartnerId` must be an active partner (422 otherwise); a repeated `code` is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &CreateOperationTypesRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateOperationTypesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/operation-types/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Change an operation type's fields; omitted ones keep their value. A `code` already used by another type is refused with 409, a body without changes with 400.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        request: &UpdateOperationTypesRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateOperationTypesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/operation-types/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read one operation type.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        request: &GetOperationTypesRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetOperationTypesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/operation-types/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete an operation type that no sale invoice, purchase invoice or document series refers to. Otherwise the call is refused with 409; deactivate the type with `operation-types/update` instead.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        request: &DeleteOperationTypesRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeleteOperationTypesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/operation-types/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List operation types. Filterable by `code`, `name`, `isSale`, `isPurchase` and `isActive`; sortable by `code`, `name`, `sortOrder` and `createdAt`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &ListOperationTypesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListOperationTypesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/operation-types/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
