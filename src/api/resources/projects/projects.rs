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

    pub async fn create(
        &self,
        request: &CreateProjectsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateProjectsResponse, ApiError> {
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

    pub async fn update(
        &self,
        request: &UpdateProjectsRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateProjectsResponse, ApiError> {
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

    pub async fn get(
        &self,
        request: &GetProjectsRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetProjectsResponse, ApiError> {
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

    pub async fn list(
        &self,
        request: &ListProjectsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListProjectsResponse, ApiError> {
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

    pub async fn time_entries_create(
        &self,
        request: &TimeEntriesCreateProjectsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TimeEntriesCreateProjectsResponse, ApiError> {
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

    pub async fn time_entries_update(
        &self,
        request: &TimeEntriesUpdateProjectsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TimeEntriesUpdateProjectsResponse, ApiError> {
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

    pub async fn time_entries_delete(
        &self,
        request: &TimeEntriesDeleteProjectsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TimeEntriesDeleteProjectsResponse, ApiError> {
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

    pub async fn time_entries_list(
        &self,
        request: &TimeEntriesListProjectsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TimeEntriesListProjectsResponse, ApiError> {
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

    pub async fn time_entries_bill(
        &self,
        request: &TimeEntriesBillProjectsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TimeEntriesBillProjectsResponse, ApiError> {
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

    pub async fn report(
        &self,
        request: &ReportProjectsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReportProjectsResponse, ApiError> {
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
