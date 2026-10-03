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

    pub async fn post_v1_calendar_list(
        &self,
        request: &PostV1CalendarListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CalendarListResponse, ApiError> {
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

    pub async fn post_v1_calendar_get(
        &self,
        request: &PostV1CalendarGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CalendarGetResponse, ApiError> {
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

    pub async fn generate_the_filing_for_a_deadline_and_send_it_to_the_administration(
        &self,
        request: &PostV1CalendarSubmitRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CalendarSubmitResponse, ApiError> {
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
    pub async fn generate_the_file_of_a_deadline_for_the_company_to_send_itself(
        &self,
        request: &PostV1CalendarDownloadRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CalendarDownloadResponse, ApiError> {
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

    pub async fn post_v1_calendar_create(
        &self,
        request: &PostV1CalendarCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CalendarCreateResponse, ApiError> {
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

    pub async fn post_v1_calendar_update(
        &self,
        request: &PostV1CalendarUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CalendarUpdateResponse, ApiError> {
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

    pub async fn post_v1_calendar_delete(
        &self,
        request: &PostV1CalendarDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1CalendarDeleteResponse, ApiError> {
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
