use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ReferenceClient {
    pub http_client: HttpClient,
}

impl ReferenceClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn post_v1_reference_exchange_rates_sync(
        &self,
        request: &PostV1ReferenceExchangeRatesSyncRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReferenceExchangeRatesSyncResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/exchange-rates/sync",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reference_exchange_rates_list(
        &self,
        request: &PostV1ReferenceExchangeRatesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReferenceExchangeRatesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/exchange-rates/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reference_exchange_rates_set(
        &self,
        request: &PostV1ReferenceExchangeRatesSetRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReferenceExchangeRatesSetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/exchange-rates/set",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reference_exchange_rates_overrides_list(
        &self,
        request: &PostV1ReferenceExchangeRatesOverridesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReferenceExchangeRatesOverridesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/exchange-rates/overrides/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reference_exchange_rates_overrides_delete(
        &self,
        request: &PostV1ReferenceExchangeRatesOverridesDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReferenceExchangeRatesOverridesDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/exchange-rates/overrides/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reference_countries_list(
        &self,
        request: &PostV1ReferenceCountriesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReferenceCountriesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/countries/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reference_banks_list(
        &self,
        request: &PostV1ReferenceBanksListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReferenceBanksListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/banks/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reference_banks_upsert(
        &self,
        request: &PostV1ReferenceBanksUpsertRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReferenceBanksUpsertResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/banks/upsert",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reference_lt_regions_list(
        &self,
        request: &PostV1ReferenceLtRegionsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReferenceLtRegionsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/lt/regions/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reference_currencies_list(
        &self,
        request: &PostV1ReferenceCurrenciesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReferenceCurrenciesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/currencies/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reference_vat_classifiers_list(
        &self,
        request: &PostV1ReferenceVatClassifiersListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReferenceVatClassifiersListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/vat-classifiers/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reference_vat_classifiers_upsert(
        &self,
        request: &PostV1ReferenceVatClassifiersUpsertRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReferenceVatClassifiersUpsertResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/vat-classifiers/upsert",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reference_eu_vat_rates_list(
        &self,
        request: &PostV1ReferenceEuVatRatesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReferenceEuVatRatesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/eu-vat-rates/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reference_vat_resolve(
        &self,
        request: &PostV1ReferenceVatResolveRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReferenceVatResolveResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/vat/resolve",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reference_cn_codes_list(
        &self,
        request: &PostV1ReferenceCnCodesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReferenceCnCodesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/cn-codes/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reference_cn_codes_upsert(
        &self,
        request: &PostV1ReferenceCnCodesUpsertRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReferenceCnCodesUpsertResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/cn-codes/upsert",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reference_compliance_versions_list(
        &self,
        request: &PostV1ReferenceComplianceVersionsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReferenceComplianceVersionsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/compliance-versions/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reference_intrastat_thresholds_list(
        &self,
        request: &PostV1ReferenceIntrastatThresholdsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReferenceIntrastatThresholdsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/intrastat-thresholds/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reference_units_list(
        &self,
        request: &PostV1ReferenceUnitsListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReferenceUnitsListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/units/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reference_series_create(
        &self,
        request: &PostV1ReferenceSeriesCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReferenceSeriesCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/series/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn post_v1_reference_series_list(
        &self,
        request: &PostV1ReferenceSeriesListRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostV1ReferenceSeriesListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/series/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
