use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct DeclarationsClient {
    pub http_client: HttpClient,
}

impl DeclarationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn post_v1_declarations_lt_intrastat_compute(
        &self,
        request: &PostV1DeclarationsLtIntrastatComputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtIntrastatComputeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/intrastat/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_lt_ivaz_generate(
        &self,
        request: &PostV1DeclarationsLtIvazGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtIvazGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/ivaz/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_lt_intrastat_obligation(
        &self,
        request: &PostV1DeclarationsLtIntrastatObligationRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtIntrastatObligationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/intrastat/obligation",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_lt_isaf_generate(
        &self,
        request: &PostV1DeclarationsLtIsafGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtIsafGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/isaf/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_lt_fr0600_compute(
        &self,
        request: &PostV1DeclarationsLtFr0600ComputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtFr0600ComputeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/fr0600/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_lt_gpm313_compute(
        &self,
        request: &PostV1DeclarationsLtGpm313ComputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtGpm313ComputeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/gpm313/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_lt_sam_compute(
        &self,
        request: &PostV1DeclarationsLtSamComputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtSamComputeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/sam/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_lt_sd_generate(
        &self,
        request: &PostV1DeclarationsLtSdGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtSdGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/sd/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_lt_saft_generate(
        &self,
        request: &PostV1DeclarationsLtSaftGenerateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsLtSaftGenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/lt/saft/generate",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_eu_oss_compute(
        &self,
        request: &PostV1DeclarationsEuOssComputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsEuOssComputeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/oss/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_eu_ioss_compute(
        &self,
        request: &PostV1DeclarationsEuIossComputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsEuIossComputeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/ioss/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_eu_distance_sales_threshold_get(
        &self,
        request: &PostV1DeclarationsEuDistanceSalesThresholdGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsEuDistanceSalesThresholdGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/distance-sales-threshold/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_eu_union_turnover_get(
        &self,
        request: &PostV1DeclarationsEuUnionTurnoverGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsEuUnionTurnoverGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/union-turnover/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_eu_sme_cross_border_report_compute(
        &self,
        request: &PostV1DeclarationsEuSmeCrossBorderReportComputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsEuSmeCrossBorderReportComputeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/sme-cross-border-report/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_eu_sme_thresholds_list(
        &self,
        request: &PostV1DeclarationsEuSmeThresholdsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsEuSmeThresholdsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/sme-thresholds/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_eu_sme_threshold_get(
        &self,
        request: &PostV1DeclarationsEuSmeThresholdGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsEuSmeThresholdGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/sme-threshold/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_eu_vat_return_packs_list(
        &self,
        request: &PostV1DeclarationsEuVatReturnPacksListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsEuVatReturnPacksListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/vat-return/packs/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_eu_vat_return_compute(
        &self,
        request: &PostV1DeclarationsEuVatReturnComputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsEuVatReturnComputeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/eu/vat-return/compute",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_configs_list(
        &self,
        request: &PostV1DeclarationsConfigsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsConfigsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/configs/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_configs_update(
        &self,
        request: &PostV1DeclarationsConfigsUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsConfigsUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/configs/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_submissions_create(
        &self,
        request: &PostV1DeclarationsSubmissionsCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsSubmissionsCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/submissions/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_submissions_mark(
        &self,
        request: &PostV1DeclarationsSubmissionsMarkRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsSubmissionsMarkResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/submissions/mark",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_declarations_submissions_list(
        &self,
        request: &PostV1DeclarationsSubmissionsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1DeclarationsSubmissionsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/declarations/submissions/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
