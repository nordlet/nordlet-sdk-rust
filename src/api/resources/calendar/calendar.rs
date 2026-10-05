use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct CalendarClient {
    pub http_client: HttpClient,
}

impl CalendarClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn list(
        &self,
        request: &ListCalendarRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCalendarResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/calendar/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn get(
        &self,
        request: &GetCalendarRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetCalendarResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/calendar/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// With amend: true the return is filed again as a correction of the one already submitted or accepted for the period; only returns whose format has a correction mark accept it.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn submit(
        &self,
        request: &SubmitCalendarRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubmitCalendarResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/calendar/submit",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Builds the file of a deadline whose format Nordlet produces but whose administration takes it only through the company's own account or program. Nothing is sent and no filing is recorded.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn download(
        &self,
        request: &DownloadCalendarRequest,
        options: Option<RequestOptions>,
    ) -> Result<DownloadCalendarResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/calendar/download",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn create(
        &self,
        request: &CreateCalendarRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateCalendarResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/calendar/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn update(
        &self,
        request: &UpdateCalendarRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateCalendarResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/calendar/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn delete(
        &self,
        request: &DeleteCalendarRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeleteCalendarResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/calendar/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
