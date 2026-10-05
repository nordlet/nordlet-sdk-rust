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

    pub async fn groups_create(
        &self,
        request: &GroupsCreateAssetsRequest,
        options: Option<RequestOptions>,
    ) -> Result<GroupsCreateAssetsResponse, ApiError> {
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

    pub async fn groups_list(
        &self,
        request: &GroupsListAssetsRequest,
        options: Option<RequestOptions>,
    ) -> Result<GroupsListAssetsResponse, ApiError> {
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

    pub async fn assets_create(
        &self,
        request: &AssetsCreateAssetsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AssetsCreateAssetsResponse, ApiError> {
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

    pub async fn assets_update(
        &self,
        request: &AssetsUpdateAssetsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AssetsUpdateAssetsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/assets/assets/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Record the input VAT facts of a capital good that the annual VAT return needs for the adjustment of the deduction over the adjustment period (Article 187 of the VAT Directive, § 15a UStG): the input VAT on the acquisition, the date of first use, the share of use for deductible turnover at first use, whether it is land or a building (ten-year period instead of five), and every later year in which the share changed or the good was sold or withdrawn. Allowed also after depreciation has been posted.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn assets_input_vat(
        &self,
        request: &AssetsInputVatAssetsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AssetsInputVatAssetsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/assets/assets/input-vat",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn assets_get(
        &self,
        request: &AssetsGetAssetsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AssetsGetAssetsResponse, ApiError> {
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

    pub async fn assets_list(
        &self,
        request: &AssetsListAssetsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AssetsListAssetsResponse, ApiError> {
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

    pub async fn assets_modernize(
        &self,
        request: &AssetsModernizeAssetsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AssetsModernizeAssetsResponse, ApiError> {
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

    /// Dispose of a fixed asset (sold, scrapped or written off). Removes its cost and accumulated depreciation, books the net book value as a disposal loss and the proceeds as a disposal gain (posting rules assets.disposalLoss, assets.disposalGain, assets.disposalProceeds), and stops its depreciation. Depreciation must be posted for every month before the disposal month.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn assets_dispose(
        &self,
        request: &AssetsDisposeAssetsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AssetsDisposeAssetsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/assets/assets/dispose",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn depreciation_preview(
        &self,
        request: &DepreciationPreviewAssetsRequest,
        options: Option<RequestOptions>,
    ) -> Result<DepreciationPreviewAssetsResponse, ApiError> {
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

    pub async fn depreciation_post(
        &self,
        request: &DepreciationPostAssetsRequest,
        options: Option<RequestOptions>,
    ) -> Result<DepreciationPostAssetsResponse, ApiError> {
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
