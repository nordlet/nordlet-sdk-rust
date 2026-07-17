use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AssetsClient {
    pub http_client: HttpClient,
}

impl AssetsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn post_v1_assets_groups_create(
        &self,
        request: &PostV1AssetsGroupsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AssetsGroupsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/assets/groups/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_assets_groups_list(
        &self,
        request: &PostV1AssetsGroupsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AssetsGroupsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/assets/groups/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_assets_assets_create(
        &self,
        request: &PostV1AssetsAssetsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AssetsAssetsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/assets/assets/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_assets_assets_get(
        &self,
        request: &PostV1AssetsAssetsGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AssetsAssetsGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/assets/assets/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_assets_assets_list(
        &self,
        request: &PostV1AssetsAssetsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AssetsAssetsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/assets/assets/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_assets_assets_modernize(
        &self,
        request: &PostV1AssetsAssetsModernizeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AssetsAssetsModernizeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/assets/assets/modernize",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_assets_depreciation_preview(
        &self,
        request: &PostV1AssetsDepreciationPreviewRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AssetsDepreciationPreviewResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/assets/depreciation/preview",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_assets_depreciation_post(
        &self,
        request: &PostV1AssetsDepreciationPostRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1AssetsDepreciationPostResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/assets/depreciation/post",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
