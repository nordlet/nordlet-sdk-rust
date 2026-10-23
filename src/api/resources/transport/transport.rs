use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct TransportClient {
    pub http_client: HttpClient,
}

impl TransportClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create a draft waybill to the consignee. The cargo lines are `lines` or, when none are given and `saleInvoiceId` is set, the product lines of that sale invoice (422 when it has none); a line without `description`, `unit` or `productCode` takes them from its item. `documentDate` defaults to the company's today and `series` to `VAZ`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn waybills_create(
        &self,
        request: &WaybillsCreateTransportRequest,
        options: Option<RequestOptions>,
    ) -> Result<WaybillsCreateTransportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/transport/waybills/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Change a draft waybill; omitted fields keep their value, the optional ones can be cleared with null, and `lines`, when given, replace all existing lines. A waybill that is already issued or cancelled is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn waybills_update(
        &self,
        request: &WaybillsUpdateTransportRequest,
        options: Option<RequestOptions>,
    ) -> Result<WaybillsUpdateTransportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/transport/waybills/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Issue a draft waybill: it takes the next number of its `series` for the year of `documentDate` and becomes `issued`. Without a vehicle plate or without lines the call is refused with 422, a waybill that is not a draft with 409. Only issued waybills can go into the Lithuanian i.VAZ file (`declarations/lt/ivaz/generate`) and the Romanian e-Transport declaration (`declarations/ro/etransport/build`).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn waybills_issue(
        &self,
        request: &WaybillsIssueTransportRequest,
        options: Option<RequestOptions>,
    ) -> Result<WaybillsIssueTransportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/transport/waybills/issue",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Cancel a draft or issued waybill. An already cancelled one is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn waybills_cancel(
        &self,
        request: &WaybillsCancelTransportRequest,
        options: Option<RequestOptions>,
    ) -> Result<WaybillsCancelTransportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/transport/waybills/cancel",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read one waybill with its lines.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn waybills_get(
        &self,
        request: &WaybillsGetTransportRequest,
        options: Option<RequestOptions>,
    ) -> Result<WaybillsGetTransportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/transport/waybills/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List waybills with the consignee's name. Filterable by `status`, `consigneePartnerId`, `documentDate`, `fullNumber` and `dispatchAt`; sortable by `createdAt`, `dispatchAt` and `fullNumber`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn waybills_list(
        &self,
        request: &WaybillsListTransportRequest,
        options: Option<RequestOptions>,
    ) -> Result<WaybillsListTransportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/transport/waybills/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
