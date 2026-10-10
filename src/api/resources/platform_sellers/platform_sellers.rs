use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PlatformSellersClient {
    pub http_client: HttpClient,
}

impl PlatformSellersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Individuals and entities that sell goods, rent out property or transport, or perform personal services through the platform the company operates. The yearly DAC7 report is built from them.
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
        request: &ListPlatformSellersRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPlatformSellersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/platform-sellers/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn get(
        &self,
        request: &GetPlatformSellersRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetPlatformSellersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/platform-sellers/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn create(
        &self,
        request: &CreatePlatformSellersRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreatePlatformSellersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/platform-sellers/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn update(
        &self,
        request: &UpdatePlatformSellersRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdatePlatformSellersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/platform-sellers/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn delete(
        &self,
        request: &DeletePlatformSellersRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeletePlatformSellersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/platform-sellers/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
