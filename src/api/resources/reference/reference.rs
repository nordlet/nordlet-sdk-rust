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

    /// Fetches the euro exchange rates the Bank of Lithuania web service publishes for `date` (default: the company's current date) and stores them in the rate table shared by all companies, replacing the rates already stored for that date. Answers 404 when no rates are published for the date.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists the stored euro exchange rates, which are shared by all companies. Filterable by `currencyCode` and `date`; sortable by `date` and `currencyCode`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Stores a company exchange rate for `currency` on `date`, in units of the currency per one unit of the company's base currency, replacing the company rate stored for that currency and date. Rate lookups use the company rate when it is dated on or after the latest shared rate on or before the lookup date. Answers 422 when `rate` is not positive.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists the company's own exchange rates set with `reference/exchange-rates/set`. Filterable by `currencyCode` and `date`; sortable by `date` and `currencyCode`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Removes the company's own exchange rate for `currency` on `date`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists every country code Nordlet knows with its EU and EEA membership and its name in each supported locale.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists the ten Lithuanian counties (apskritys) with their code and ISO code.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists the Lithuanian municipalities (savivaldybės) with the county each belongs to; `countyCode` limits the list to one county.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists Lithuanian cities with their municipality; `municipalityCode` limits the list to one municipality and `q` to names that start with the given text (case-insensitive).
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists the banks of the bank directory shared by all companies. Filterable by `countryCode`, `name`, `bic` and `bankCode`; sortable by `name`, `bic` and `countryCode`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Adds a bank to the shared bank directory or, when a bank with the same `countryCode` and `bic` exists, replaces its name, bank code and active flag. Platform administrators only.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists the ten Lithuanian regions (apskritys) with their code and ISO code; the same list as `reference/lt/counties/list`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists the currencies known to Nordlet with their minor units. Filterable and sortable by `code` and `name`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists the company's VAT classifiers (code, country, name and rate). A document line that names a classifier in `vatClassifierCode` takes its rate from it when neither the line nor the item has a rate. Filterable by `code`, `countryCode` and `ratePercent`; sortable by `code` and `ratePercent`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Adds VAT classifiers to the company or replaces the country, name and rate of the ones whose `code` already exists.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Works out the VAT treatment of a supply by this company: the scheme (domestic, intra-EU B2B, reverse charge, OSS, IOSS, marketplace deemed supplier, export, out of scope or SME exempt), the country whose VAT applies, its rates with the company's overrides applied, and the legal basis. The customer comes from `partnerId` (country from the partner's address or VAT number, business status from the VAT number) or from `customerCountryCode` and `customerIsBusiness`; `date` defaults to the company's current date; when `belowDistanceSalesThreshold` is not given, the distance-sales threshold is tracked from the company's ledger. Answers 422 when the partner has neither an address country nor a VAT code.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists the Combined Nomenclature codes of the directory shared by all companies. Filterable by `code`, `name` and `nameLt`; sortable by `code`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Adds Combined Nomenclature codes to the shared directory or replaces the names and supplementary unit of the codes that exist. Platform administrators only.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists the versions of the filing systems, schemas and forms Nordlet's output was checked against, with the date of the check and the source; `country` limits the list to one country.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists the Intrastat thresholds stored per year: the reporting and the statistical-value threshold for arrivals and for dispatches.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists the measurement units of the directory shared by all companies, with Lithuanian and English names. Filterable by `code`, `nameLt` and `nameEn`; sortable by `code`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Creates a numbering series for a document type, prefix and year; the first number the series hands out is at least `startAt`. Answers 409 when a series with the same document type, prefix and year exists.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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

    /// Lists the company's numbering series with the next number of each. Filterable and sortable by `documentType`, `prefix` and `year`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
