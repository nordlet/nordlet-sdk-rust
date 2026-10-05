use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct OfficersClient {
    pub http_client: HttpClient,
}

impl OfficersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Directors, board members, the company secretary, representatives and liquidators, with their personal identifier, appointment and resignation dates and whether they sign the annual accounts. Annual returns and registry deposits are built from this register.
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
        request: &ListOfficersRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListOfficersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/officers/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn create(
        &self,
        request: &CreateOfficersRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateOfficersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/officers/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn update(
        &self,
        request: &UpdateOfficersRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateOfficersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/officers/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn delete(
        &self,
        request: &DeleteOfficersRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeleteOfficersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/officers/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
