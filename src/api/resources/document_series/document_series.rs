use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct DocumentSeriesClient {
    pub http_client: HttpClient,
}

impl DocumentSeriesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create a numbering series for sale invoices, credit notes, proformas or advance invoices (`documentType`). Numbers run from `nextNumber`, padded to `numberLength` digits, and are written as `prefix`-number or, with `printSeries` false, as the bare number; a series may be bound to an operation type (a default series of an operation type is picked for its documents that name no series) and to a warehouse, which `sales/invoices/issue` takes the goods from. `isDefault` makes it the default of its document type and clears the previous one. Answers 422 when only one end of the allocated range `allocatedFrom`..`allocatedTo` is given or the range is reversed, 409 when the prefix exists for the document type.
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
        request: &CreateDocumentSeriesRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateDocumentSeriesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/document-series/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Change a document series; fields left out keep their value. `nextNumber` cannot go below its current value (409); `isDefault` clears the previous default of the document type. Answers 422 for an incomplete or reversed allocated range, 409 when the prefix is taken for the document type.
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
        request: &UpdateDocumentSeriesRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateDocumentSeriesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/document-series/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Return one document series.
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
        request: &GetDocumentSeriesRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetDocumentSeriesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/document-series/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a document series no sale document uses. Answers 409 when invoices reference it; deactivate it with `isActive` instead.
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
        request: &DeleteDocumentSeriesRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeleteDocumentSeriesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/document-series/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List document series. Filterable by documentType, prefix, operationTypeId and isActive; sortable by prefix, documentType and createdAt.
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
        request: &ListDocumentSeriesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListDocumentSeriesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/document-series/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
