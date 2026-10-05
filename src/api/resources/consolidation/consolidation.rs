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

    pub async fn groups_create(
        &self,
        request: &GroupsCreateConsolidationRequest,
        options: Option<RequestOptions>,
    ) -> Result<GroupsCreateConsolidationResponse, ApiError> {
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

    pub async fn groups_list(
        &self,
        request: &GroupsListConsolidationRequest,
        options: Option<RequestOptions>,
    ) -> Result<GroupsListConsolidationResponse, ApiError> {
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

    pub async fn groups_get(
        &self,
        request: &GroupsGetConsolidationRequest,
        options: Option<RequestOptions>,
    ) -> Result<GroupsGetConsolidationResponse, ApiError> {
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

    pub async fn groups_update(
        &self,
        request: &GroupsUpdateConsolidationRequest,
        options: Option<RequestOptions>,
    ) -> Result<GroupsUpdateConsolidationResponse, ApiError> {
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

    pub async fn groups_delete(
        &self,
        request: &GroupsDeleteConsolidationRequest,
        options: Option<RequestOptions>,
    ) -> Result<GroupsDeleteConsolidationResponse, ApiError> {
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

    pub async fn members_add(
        &self,
        request: &MembersAddConsolidationRequest,
        options: Option<RequestOptions>,
    ) -> Result<MembersAddConsolidationResponse, ApiError> {
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

    pub async fn members_remove(
        &self,
        request: &MembersRemoveConsolidationRequest,
        options: Option<RequestOptions>,
    ) -> Result<MembersRemoveConsolidationResponse, ApiError> {
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
    pub async fn intercompany_candidates(
        &self,
        request: &IntercompanyCandidatesConsolidationRequest,
        options: Option<RequestOptions>,
    ) -> Result<IntercompanyCandidatesConsolidationResponse, ApiError> {
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
    pub async fn intercompany_links_set(
        &self,
        request: &IntercompanyLinksSetConsolidationRequest,
        options: Option<RequestOptions>,
    ) -> Result<IntercompanyLinksSetConsolidationResponse, ApiError> {
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

    pub async fn intercompany_links_list(
        &self,
        request: &IntercompanyLinksListConsolidationRequest,
        options: Option<RequestOptions>,
    ) -> Result<IntercompanyLinksListConsolidationResponse, ApiError> {
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

    pub async fn intercompany_links_remove(
        &self,
        request: &IntercompanyLinksRemoveConsolidationRequest,
        options: Option<RequestOptions>,
    ) -> Result<IntercompanyLinksRemoveConsolidationResponse, ApiError> {
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
    pub async fn intercompany_report(
        &self,
        request: &IntercompanyReportConsolidationRequest,
        options: Option<RequestOptions>,
    ) -> Result<IntercompanyReportConsolidationResponse, ApiError> {
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

    pub async fn report(
        &self,
        request: &ReportConsolidationRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReportConsolidationResponse, ApiError> {
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
