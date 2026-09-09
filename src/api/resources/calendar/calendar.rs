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
