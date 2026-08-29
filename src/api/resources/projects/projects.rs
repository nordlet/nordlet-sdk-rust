use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ProjectsClient {
    pub http_client: HttpClient,
}

impl ProjectsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn post_v1_projects_create(
        &self,
        request: &PostV1ProjectsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProjectsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/projects/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_projects_update(
        &self,
        request: &PostV1ProjectsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProjectsUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/projects/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_projects_get(
        &self,
        request: &PostV1ProjectsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProjectsGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/projects/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_projects_list(
        &self,
        request: &PostV1ProjectsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProjectsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/projects/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_projects_time_entries_create(
        &self,
        request: &PostV1ProjectsTimeEntriesCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProjectsTimeEntriesCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/projects/time-entries/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_projects_time_entries_update(
        &self,
        request: &PostV1ProjectsTimeEntriesUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProjectsTimeEntriesUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/projects/time-entries/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_projects_time_entries_delete(
        &self,
        request: &PostV1ProjectsTimeEntriesDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProjectsTimeEntriesDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/projects/time-entries/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_projects_time_entries_list(
        &self,
        request: &PostV1ProjectsTimeEntriesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProjectsTimeEntriesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/projects/time-entries/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_projects_time_entries_bill(
        &self,
        request: &PostV1ProjectsTimeEntriesBillRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProjectsTimeEntriesBillResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/projects/time-entries/bill",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_projects_report(
        &self,
        request: &PostV1ProjectsReportRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ProjectsReportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/projects/report",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
