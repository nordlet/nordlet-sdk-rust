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

    /// Read one platform seller with its activities, ordered by year.
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

    /// Record a seller on the company's platform for the DAC7 report: an `individual` (first and last name required) or an `entity` (`entityName` required), with its `activities` per year and quarter. An `immovable_property` activity needs `propertyAddress`; property address, type and rented days on any other kind of activity are refused. These checks answer 422; a `partnerId` that does not exist answers 404. The report is built from the sellers with `declarations/eu/dac7/preview` and `declarations/eu/dac7/xml`.
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

    /// Replace a platform seller's details and all of its activities with the ones given, under the same checks as `platform-sellers/create`.
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

    /// Mark a platform seller as deleted. It leaves the lists and the DAC7 report.
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
