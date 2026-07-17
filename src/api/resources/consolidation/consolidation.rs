use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ConsolidationClient {
    pub http_client: HttpClient,
}

impl ConsolidationClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn post_v1_consolidation_groups_create(
        &self,
        request: &PostV1ConsolidationGroupsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ConsolidationGroupsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/consolidation/groups/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_consolidation_groups_list(
        &self,
        request: &PostV1ConsolidationGroupsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ConsolidationGroupsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/consolidation/groups/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_consolidation_groups_get(
        &self,
        request: &PostV1ConsolidationGroupsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ConsolidationGroupsGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/consolidation/groups/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_consolidation_groups_update(
        &self,
        request: &PostV1ConsolidationGroupsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ConsolidationGroupsUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/consolidation/groups/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_consolidation_groups_delete(
        &self,
        request: &PostV1ConsolidationGroupsDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ConsolidationGroupsDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/consolidation/groups/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_consolidation_members_add(
        &self,
        request: &PostV1ConsolidationMembersAddRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ConsolidationMembersAddResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/consolidation/members/add",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_consolidation_members_remove(
        &self,
        request: &PostV1ConsolidationMembersRemoveRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ConsolidationMembersRemoveResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/consolidation/members/remove",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_consolidation_report(
        &self,
        request: &PostV1ConsolidationReportRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ConsolidationReportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/consolidation/report",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
