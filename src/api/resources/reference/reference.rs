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

    pub async fn exchange_rates_sync(
        &self,
        request: &ExchangeRatesSyncReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExchangeRatesSyncReferenceResponse, ApiError> {
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

    pub async fn exchange_rates_list(
        &self,
        request: &ExchangeRatesListReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExchangeRatesListReferenceResponse, ApiError> {
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

    pub async fn exchange_rates_set(
        &self,
        request: &ExchangeRatesSetReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExchangeRatesSetReferenceResponse, ApiError> {
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

    pub async fn exchange_rates_overrides_list(
        &self,
        request: &ExchangeRatesOverridesListReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExchangeRatesOverridesListReferenceResponse, ApiError> {
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

    pub async fn exchange_rates_overrides_delete(
        &self,
        request: &ExchangeRatesOverridesDeleteReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExchangeRatesOverridesDeleteReferenceResponse, ApiError> {
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

    pub async fn countries_list(
        &self,
        request: &CountriesListReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<CountriesListReferenceResponse, ApiError> {
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

    pub async fn lt_counties_list(
        &self,
        request: &LtCountiesListReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<LtCountiesListReferenceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/lt/counties/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn lt_municipalities_list(
        &self,
        request: &LtMunicipalitiesListReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<LtMunicipalitiesListReferenceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/lt/municipalities/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn lt_cities_list(
        &self,
        request: &LtCitiesListReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<LtCitiesListReferenceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/lt/cities/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn banks_list(
        &self,
        request: &BanksListReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<BanksListReferenceResponse, ApiError> {
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

    pub async fn banks_upsert(
        &self,
        request: &BanksUpsertReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<BanksUpsertReferenceResponse, ApiError> {
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

    pub async fn lt_regions_list(
        &self,
        request: &LtRegionsListReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<LtRegionsListReferenceResponse, ApiError> {
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

    pub async fn currencies_list(
        &self,
        request: &CurrenciesListReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<CurrenciesListReferenceResponse, ApiError> {
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

    pub async fn vat_classifiers_list(
        &self,
        request: &VatClassifiersListReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<VatClassifiersListReferenceResponse, ApiError> {
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

    pub async fn vat_classifiers_upsert(
        &self,
        request: &VatClassifiersUpsertReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<VatClassifiersUpsertReferenceResponse, ApiError> {
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

    /// Effective EU VAT rate mapping for this company: EC TEDB defaults, replaced per country by any company overrides. Verify the mapping fits the goods and services you sell before relying on it.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn eu_vat_rates_list(
        &self,
        request: &EuVatRatesListReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<EuVatRatesListReferenceResponse, ApiError> {
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

    /// Replace the VAT rate mapping this company uses for one EU country. Pass an empty rates array to drop the overrides and return to the TEDB defaults. Overrides feed rate suggestions (vat/resolve) and OSS/IOSS return rate classification.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn eu_vat_rates_set_overrides(
        &self,
        request: &EuVatRatesSetOverridesReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<EuVatRatesSetOverridesReferenceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/reference/eu-vat-rates/set-overrides",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn vat_resolve(
        &self,
        request: &VatResolveReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<VatResolveReferenceResponse, ApiError> {
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

    pub async fn cn_codes_list(
        &self,
        request: &CnCodesListReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<CnCodesListReferenceResponse, ApiError> {
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

    pub async fn cn_codes_upsert(
        &self,
        request: &CnCodesUpsertReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<CnCodesUpsertReferenceResponse, ApiError> {
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

    pub async fn compliance_versions_list(
        &self,
        request: &ComplianceVersionsListReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<ComplianceVersionsListReferenceResponse, ApiError> {
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

    pub async fn intrastat_thresholds_list(
        &self,
        request: &IntrastatThresholdsListReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<IntrastatThresholdsListReferenceResponse, ApiError> {
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

    pub async fn units_list(
        &self,
        request: &UnitsListReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<UnitsListReferenceResponse, ApiError> {
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

    pub async fn series_create(
        &self,
        request: &SeriesCreateReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<SeriesCreateReferenceResponse, ApiError> {
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

    pub async fn series_list(
        &self,
        request: &SeriesListReferenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<SeriesListReferenceResponse, ApiError> {
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
