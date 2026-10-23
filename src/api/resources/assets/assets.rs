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

    /// Return the company's fixed asset setting `autoDepreciation`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn settings_get(
        &self,
        request: &SettingsGetAssetsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettingsGetAssetsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/assets/settings/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Turn automatic depreciation on or off. When it is on, the scheduler posts the previous month's depreciation for the company once a day, as `assets/depreciation/post` does.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn settings_update(
        &self,
        request: &SettingsUpdateAssetsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SettingsUpdateAssetsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/assets/settings/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create an asset group: the asset, accumulated depreciation and depreciation expense accounts its assets are posted to, and an optional default useful life in months. `expenseAccountCode` defaults to the chart's depreciation expense account; each account must exist and be postable (422). A duplicate code is refused with 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List the asset groups; filterable and sortable by `code` and `name`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Register a fixed asset in a group with its acquisition date and cost, salvage value, useful life in months (defaults to the group's default; 422 when neither is set) and depreciation start date (defaults to the acquisition date). The salvage value must be below the acquisition cost (422); a duplicate code is refused with 409. Depreciation is posted with `assets/depreciation/post`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Change an asset that has no posted depreciation yet: group, code, name, dates, cost, salvage value, useful life, notes or documents. Refused with 409 once depreciation is posted or the asset is disposed, and with 422 when the salvage value would reach the acquisition cost; a duplicate code is 409.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Return one fixed asset with its totals (cost including modernisations, accumulated depreciation, net book value, total life in months), its input VAT facts and its disposal.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// List the fixed assets; filterable by `code`, `name`, `groupId`, `status`, `acquisitionDate`, `acquisitionCost` and `accumulatedDepreciation`, sortable by `code`, `name`, `acquisitionDate` and `acquisitionCost`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Record a modernisation of a fixed asset: `amount` is added to its cost and `addedLifeMonths` to its useful life from the month of `date` on, and the remaining depreciable value is spread over the months left. Refused with 422 when the asset is disposed or when no depreciation month would be left (give `addedLifeMonths` of at least 1); a fully depreciated asset becomes active again.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Preview what `assets/depreciation/post` would post for a month: per active asset the amount and the months it covers, earlier unposted months included, and whether the month is already posted. Nothing is written.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Post the depreciation of a month: one journal entry dated the last day of the month that debits each group's expense account and credits its accumulated depreciation account, for every active asset whose depreciation has started, covering that month and any earlier month not yet posted. Each month's amount is the remaining depreciable value (cost plus modernisations less salvage value less accumulated depreciation) divided by the months left; an asset whose life or value is used up becomes `fully_depreciated`. Answers 409 when the period is locked, and `journalTransactionId` null when nothing is due.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
