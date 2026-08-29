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

    /// Partners in member companies that look like other members of the same group (matched on company code or VAT code), with any existing intercompany link. Confirming a candidate via intercompany/links/set enables invoice mirroring.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_consolidation_intercompany_candidates(
        &self,
        request: &PostV1ConsolidationIntercompanyCandidatesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ConsolidationIntercompanyCandidatesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/consolidation/intercompany/candidates",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Confirm that a partner record in one member company represents another member company of the group. Once links exist in both directions, issuing an intercompany sale invoice automatically creates the matching draft purchase invoice in the counterparty.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_consolidation_intercompany_links_set(
        &self,
        request: &PostV1ConsolidationIntercompanyLinksSetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ConsolidationIntercompanyLinksSetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/consolidation/intercompany/links/set",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_consolidation_intercompany_links_list(
        &self,
        request: &PostV1ConsolidationIntercompanyLinksListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ConsolidationIntercompanyLinksListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/consolidation/intercompany/links/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_consolidation_intercompany_links_remove(
        &self,
        request: &PostV1ConsolidationIntercompanyLinksRemoveRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ConsolidationIntercompanyLinksRemoveResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/consolidation/intercompany/links/remove",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Intercompany reconciliation for a period: every issued intercompany sale invoice with its mirrored or manually recorded counterpart, unmatched documents on both sides, and per-currency totals with differences. Confirmed pairs are the basis for consolidation eliminations.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_v1_consolidation_intercompany_report(
        &self,
        request: &PostV1ConsolidationIntercompanyReportRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ConsolidationIntercompanyReportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/consolidation/intercompany/report",
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
