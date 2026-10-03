# Reference
## Reference
<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_exchange_rates_sync</a>(request: PostV1ReferenceExchangeRatesSyncRequest) -> Result&lt;PostV1ReferenceExchangeRatesSyncResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_exchange_rates_sync(
            &PostV1ReferenceExchangeRatesSyncRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**date:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_exchange_rates_list</a>(request: PostV1ReferenceExchangeRatesListRequest) -> Result&lt;PostV1ReferenceExchangeRatesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_exchange_rates_list(
            &PostV1ReferenceExchangeRatesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1ReferenceExchangeRatesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1ReferenceExchangeRatesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_exchange_rates_set</a>(request: PostV1ReferenceExchangeRatesSetRequest) -> Result&lt;PostV1ReferenceExchangeRatesSetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_exchange_rates_set(
            &PostV1ReferenceExchangeRatesSetRequest {
                currency: "currency".to_string(),
                date: "date".to_string(),
                rate: "rate".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**currency:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**rate:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_exchange_rates_overrides_list</a>(request: PostV1ReferenceExchangeRatesOverridesListRequest) -> Result&lt;PostV1ReferenceExchangeRatesOverridesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_exchange_rates_overrides_list(
            &PostV1ReferenceExchangeRatesOverridesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1ReferenceExchangeRatesOverridesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1ReferenceExchangeRatesOverridesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_exchange_rates_overrides_delete</a>(request: PostV1ReferenceExchangeRatesOverridesDeleteRequest) -> Result&lt;PostV1ReferenceExchangeRatesOverridesDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_exchange_rates_overrides_delete(
            &PostV1ReferenceExchangeRatesOverridesDeleteRequest {
                currency: "currency".to_string(),
                date: "date".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**currency:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_countries_list</a>(request: PostV1ReferenceCountriesListRequest) -> Result&lt;PostV1ReferenceCountriesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_countries_list(
            &PostV1ReferenceCountriesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_lt_counties_list</a>(request: PostV1ReferenceLtCountiesListRequest) -> Result&lt;PostV1ReferenceLtCountiesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_lt_counties_list(
            &PostV1ReferenceLtCountiesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_lt_municipalities_list</a>(request: PostV1ReferenceLtMunicipalitiesListRequest) -> Result&lt;PostV1ReferenceLtMunicipalitiesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_lt_municipalities_list(
            &PostV1ReferenceLtMunicipalitiesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**county_code:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_lt_cities_list</a>(request: PostV1ReferenceLtCitiesListRequest) -> Result&lt;PostV1ReferenceLtCitiesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_lt_cities_list(
            &PostV1ReferenceLtCitiesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**municipality_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**q:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_banks_list</a>(request: PostV1ReferenceBanksListRequest) -> Result&lt;PostV1ReferenceBanksListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_banks_list(
            &PostV1ReferenceBanksListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1ReferenceBanksListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1ReferenceBanksListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_banks_upsert</a>(request: PostV1ReferenceBanksUpsertRequest) -> Result&lt;PostV1ReferenceBanksUpsertResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_banks_upsert(
            &PostV1ReferenceBanksUpsertRequest {
                country_code: "countryCode".to_string(),
                name: "name".to_string(),
                bic: "bic".to_string(),
                bank_code: None,
                is_active: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**country_code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**bic:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**bank_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_lt_regions_list</a>(request: PostV1ReferenceLtRegionsListRequest) -> Result&lt;PostV1ReferenceLtRegionsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_lt_regions_list(
            &PostV1ReferenceLtRegionsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_currencies_list</a>(request: PostV1ReferenceCurrenciesListRequest) -> Result&lt;PostV1ReferenceCurrenciesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_currencies_list(
            &PostV1ReferenceCurrenciesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1ReferenceCurrenciesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1ReferenceCurrenciesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_vat_classifiers_list</a>(request: PostV1ReferenceVatClassifiersListRequest) -> Result&lt;PostV1ReferenceVatClassifiersListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_vat_classifiers_list(
            &PostV1ReferenceVatClassifiersListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1ReferenceVatClassifiersListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1ReferenceVatClassifiersListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_vat_classifiers_upsert</a>(request: PostV1ReferenceVatClassifiersUpsertRequest) -> Result&lt;PostV1ReferenceVatClassifiersUpsertResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_vat_classifiers_upsert(
            &PostV1ReferenceVatClassifiersUpsertRequest {
                rows: vec![PostV1ReferenceVatClassifiersUpsertRequestRowsItem {
                    code: "code".to_string(),
                    name: "name".to_string(),
                    ..Default::default()
                }],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**rows:** `Vec<PostV1ReferenceVatClassifiersUpsertRequestRowsItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_eu_vat_rates_list</a>(request: PostV1ReferenceEuVatRatesListRequest) -> Result&lt;PostV1ReferenceEuVatRatesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Effective EU VAT rate mapping for this company: EC TEDB defaults, replaced per country by any company overrides. Verify the mapping fits the goods and services you sell before relying on it.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_eu_vat_rates_list(
            &PostV1ReferenceEuVatRatesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**country_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_eu_vat_rates_set_overrides</a>(request: PostV1ReferenceEuVatRatesSetOverridesRequest) -> Result&lt;PostV1ReferenceEuVatRatesSetOverridesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Replace the VAT rate mapping this company uses for one EU country. Pass an empty rates array to drop the overrides and return to the TEDB defaults. Overrides feed rate suggestions (vat/resolve) and OSS/IOSS return rate classification.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_eu_vat_rates_set_overrides(
            &PostV1ReferenceEuVatRatesSetOverridesRequest {
                country_code: "countryCode".to_string(),
                rates: vec![PostV1ReferenceEuVatRatesSetOverridesRequestRatesItem {
                    category:
                        PostV1ReferenceEuVatRatesSetOverridesRequestRatesItemCategory::Standard,
                    rate_percent: "ratePercent".to_string(),
                }],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**country_code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**rates:** `Vec<PostV1ReferenceEuVatRatesSetOverridesRequestRatesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_vat_resolve</a>(request: PostV1ReferenceVatResolveRequest) -> Result&lt;PostV1ReferenceVatResolveResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_vat_resolve(
            &PostV1ReferenceVatResolveRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**partner_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**customer_country_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**customer_is_business:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**supply_type:** `Option<PostV1ReferenceVatResolveRequestSupplyType>` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**below_distance_sales_threshold:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**facilitated_by_marketplace:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**acting_as_marketplace:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**seller_established_in_eu:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**imported_consignment_value_eur:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_cn_codes_list</a>(request: PostV1ReferenceCnCodesListRequest) -> Result&lt;PostV1ReferenceCnCodesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_cn_codes_list(
            &PostV1ReferenceCnCodesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1ReferenceCnCodesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1ReferenceCnCodesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_cn_codes_upsert</a>(request: PostV1ReferenceCnCodesUpsertRequest) -> Result&lt;PostV1ReferenceCnCodesUpsertResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_cn_codes_upsert(
            &PostV1ReferenceCnCodesUpsertRequest {
                rows: vec![PostV1ReferenceCnCodesUpsertRequestRowsItem {
                    code: "code".to_string(),
                    name: "name".to_string(),
                    ..Default::default()
                }],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**rows:** `Vec<PostV1ReferenceCnCodesUpsertRequestRowsItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_compliance_versions_list</a>(request: PostV1ReferenceComplianceVersionsListRequest) -> Result&lt;PostV1ReferenceComplianceVersionsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_compliance_versions_list(
            &PostV1ReferenceComplianceVersionsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**country:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_intrastat_thresholds_list</a>(request: PostV1ReferenceIntrastatThresholdsListRequest) -> Result&lt;PostV1ReferenceIntrastatThresholdsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_intrastat_thresholds_list(
            &PostV1ReferenceIntrastatThresholdsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_units_list</a>(request: PostV1ReferenceUnitsListRequest) -> Result&lt;PostV1ReferenceUnitsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_units_list(
            &PostV1ReferenceUnitsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1ReferenceUnitsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1ReferenceUnitsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_series_create</a>(request: PostV1ReferenceSeriesCreateRequest) -> Result&lt;PostV1ReferenceSeriesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_series_create(
            &PostV1ReferenceSeriesCreateRequest {
                document_type: "documentType".to_string(),
                year: 1000000,
                prefix: None,
                start_at: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**document_type:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**prefix:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**start_at:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">post_v1_reference_series_list</a>(request: PostV1ReferenceSeriesListRequest) -> Result&lt;PostV1ReferenceSeriesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reference
        .post_v1reference_series_list(
            &PostV1ReferenceSeriesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1ReferenceSeriesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1ReferenceSeriesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Partners
<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_addresses_create</a>(request: PostV1PartnersAddressesCreateRequest) -> Result&lt;PostV1PartnersAddressesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_addresses_create(
            &PostV1PartnersAddressesCreateRequest {
                partner_id: "partnerId".to_string(),
                r#type: None,
                street: None,
                city: None,
                postal_code: None,
                country_code: None,
                is_default: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**type_:** `Option<PostV1PartnersAddressesCreateRequestType>` 
    
</dd>
</dl>

<dl>
<dd>

**street:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**city:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**postal_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**country_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_default:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**partner_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_addresses_update</a>(request: PostV1PartnersAddressesUpdateRequest) -> Result&lt;PostV1PartnersAddressesUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_addresses_update(
            &PostV1PartnersAddressesUpdateRequest {
                id: "id".to_string(),
                r#type: None,
                street: None,
                city: None,
                postal_code: None,
                country_code: None,
                is_default: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**type_:** `Option<PostV1PartnersAddressesUpdateRequestType>` 
    
</dd>
</dl>

<dl>
<dd>

**street:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**city:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**postal_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**country_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_default:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_addresses_delete</a>(request: PostV1PartnersAddressesDeleteRequest) -> Result&lt;PostV1PartnersAddressesDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_addresses_delete(
            &PostV1PartnersAddressesDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_addresses_list</a>(request: PostV1PartnersAddressesListRequest) -> Result&lt;PostV1PartnersAddressesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_addresses_list(
            &PostV1PartnersAddressesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1PartnersAddressesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1PartnersAddressesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_contacts_create</a>(request: PostV1PartnersContactsCreateRequest) -> Result&lt;PostV1PartnersContactsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_contacts_create(
            &PostV1PartnersContactsCreateRequest {
                name: "name".to_string(),
                partner_id: "partnerId".to_string(),
                role: None,
                email: None,
                phone: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**role:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**phone:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**partner_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_contacts_update</a>(request: PostV1PartnersContactsUpdateRequest) -> Result&lt;PostV1PartnersContactsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_contacts_update(
            &PostV1PartnersContactsUpdateRequest {
                id: "id".to_string(),
                name: None,
                role: None,
                email: None,
                phone: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**role:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**phone:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_contacts_delete</a>(request: PostV1PartnersContactsDeleteRequest) -> Result&lt;PostV1PartnersContactsDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_contacts_delete(
            &PostV1PartnersContactsDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_contacts_list</a>(request: PostV1PartnersContactsListRequest) -> Result&lt;PostV1PartnersContactsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_contacts_list(
            &PostV1PartnersContactsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1PartnersContactsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1PartnersContactsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_bank_accounts_create</a>(request: PostV1PartnersBankAccountsCreateRequest) -> Result&lt;PostV1PartnersBankAccountsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_bank_accounts_create(
            &PostV1PartnersBankAccountsCreateRequest {
                iban: "iban".to_string(),
                partner_id: "partnerId".to_string(),
                bank_name: None,
                bic: None,
                currency: None,
                is_default: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**iban:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**bank_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**bic:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_default:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**partner_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_bank_accounts_update</a>(request: PostV1PartnersBankAccountsUpdateRequest) -> Result&lt;PostV1PartnersBankAccountsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_bank_accounts_update(
            &PostV1PartnersBankAccountsUpdateRequest {
                id: "id".to_string(),
                iban: None,
                bank_name: None,
                bic: None,
                currency: None,
                is_default: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**iban:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**bank_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**bic:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_default:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_bank_accounts_delete</a>(request: PostV1PartnersBankAccountsDeleteRequest) -> Result&lt;PostV1PartnersBankAccountsDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_bank_accounts_delete(
            &PostV1PartnersBankAccountsDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_bank_accounts_list</a>(request: PostV1PartnersBankAccountsListRequest) -> Result&lt;PostV1PartnersBankAccountsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_bank_accounts_list(
            &PostV1PartnersBankAccountsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1PartnersBankAccountsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1PartnersBankAccountsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_files_list</a>(request: PostV1PartnersFilesListRequest) -> Result&lt;PostV1PartnersFilesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_files_list(
            &PostV1PartnersFilesListRequest {
                partner_id: "partnerId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**partner_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">reminders_the_overnight_debt_reminder_job_would_send_today_for_this_company</a>(request: PostV1PartnersDebtRemindersPreviewRequest) -> Result&lt;PostV1PartnersDebtRemindersPreviewResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .reminders_the_overnight_debt_reminder_job_would_send_today_for_this_company(
            &PostV1PartnersDebtRemindersPreviewRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_debt_reminders_list</a>(request: PostV1PartnersDebtRemindersListRequest) -> Result&lt;PostV1PartnersDebtRemindersListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_debt_reminders_list(
            &PostV1PartnersDebtRemindersListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1PartnersDebtRemindersListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1PartnersDebtRemindersListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_validate_vat</a>(request: PostV1PartnersValidateVatRequest) -> Result&lt;PostV1PartnersValidateVatResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_validate_vat(
            &PostV1PartnersValidateVatRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**vat_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**partner_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_vat_reviews_list</a>(request: PostV1PartnersVatReviewsListRequest) -> Result&lt;PostV1PartnersVatReviewsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_vat_reviews_list(
            &PostV1PartnersVatReviewsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1PartnersVatReviewsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1PartnersVatReviewsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_vat_reviews_resolve</a>(request: PostV1PartnersVatReviewsResolveRequest) -> Result&lt;PostV1PartnersVatReviewsResolveResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_vat_reviews_resolve(
            &PostV1PartnersVatReviewsResolveRequest {
                id: "id".to_string(),
                resolution: PostV1PartnersVatReviewsResolveRequestResolution::ConfirmedValid,
                note: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**resolution:** `PostV1PartnersVatReviewsResolveRequestResolution` 
    
</dd>
</dl>

<dl>
<dd>

**note:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_create</a>(request: PostV1PartnersCreateRequest) -> Result&lt;PostV1PartnersCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_create(
            &PostV1PartnersCreateRequest {
                name: "name".to_string(),
                r#type: None,
                code: None,
                vat_code: None,
                peppol_id: None,
                email: None,
                phone: None,
                self_employment_cert_no: None,
                birth_date: None,
                is_customer: None,
                is_supplier: None,
                payment_term_days: None,
                credit_limit: None,
                price_list_id: None,
                group_id: None,
                status_id: None,
                address: None,
                correspondence_address: None,
                notes: None,
                document_ref: None,
                short_name: None,
                website: None,
                fax: None,
                eori_code: None,
                other_code: None,
                foreign_tax_number: None,
                auto_debt_reminder: None,
                late_interest_percent: None,
                first_call_date: None,
                last_call_date: None,
                next_call_date: None,
                rating: None,
                is_employee: None,
                is_group_member: None,
                is_active: None,
                legal_country_class: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**type_:** `Option<PostV1PartnersCreateRequestType>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**peppol_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**phone:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**self_employment_cert_no:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**birth_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_customer:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_supplier:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**payment_term_days:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**credit_limit:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**price_list_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**group_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**status_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**address:** `Option<PostV1PartnersCreateRequestAddress>` 
    
</dd>
</dl>

<dl>
<dd>

**correspondence_address:** `Option<PostV1PartnersCreateRequestCorrespondenceAddress>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**document_ref:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**short_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**website:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**fax:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**eori_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**other_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**foreign_tax_number:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**auto_debt_reminder:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**late_interest_percent:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**first_call_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**last_call_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**next_call_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**rating:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_employee:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_group_member:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**legal_country_class:** `Option<PostV1PartnersCreateRequestLegalCountryClass>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_find_or_create</a>(request: PostV1PartnersFindOrCreateRequest) -> Result&lt;PostV1PartnersFindOrCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_find_or_create(
            &PostV1PartnersFindOrCreateRequest {
                name: "name".to_string(),
                r#type: None,
                code: None,
                vat_code: None,
                peppol_id: None,
                email: None,
                phone: None,
                self_employment_cert_no: None,
                birth_date: None,
                is_customer: None,
                is_supplier: None,
                payment_term_days: None,
                credit_limit: None,
                price_list_id: None,
                group_id: None,
                status_id: None,
                address: None,
                correspondence_address: None,
                notes: None,
                document_ref: None,
                short_name: None,
                website: None,
                fax: None,
                eori_code: None,
                other_code: None,
                foreign_tax_number: None,
                auto_debt_reminder: None,
                late_interest_percent: None,
                first_call_date: None,
                last_call_date: None,
                next_call_date: None,
                rating: None,
                is_employee: None,
                is_group_member: None,
                is_active: None,
                legal_country_class: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**type_:** `Option<PostV1PartnersFindOrCreateRequestType>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**peppol_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**phone:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**self_employment_cert_no:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**birth_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_customer:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_supplier:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**payment_term_days:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**credit_limit:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**price_list_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**group_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**status_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**address:** `Option<PostV1PartnersFindOrCreateRequestAddress>` 
    
</dd>
</dl>

<dl>
<dd>

**correspondence_address:** `Option<PostV1PartnersFindOrCreateRequestCorrespondenceAddress>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**document_ref:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**short_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**website:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**fax:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**eori_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**other_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**foreign_tax_number:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**auto_debt_reminder:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**late_interest_percent:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**first_call_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**last_call_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**next_call_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**rating:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_employee:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_group_member:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**legal_country_class:** `Option<PostV1PartnersFindOrCreateRequestLegalCountryClass>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_get</a>(request: PostV1PartnersGetRequest) -> Result&lt;PostV1PartnersGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_get(
            &PostV1PartnersGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_update</a>(request: PostV1PartnersUpdateRequest) -> Result&lt;PostV1PartnersUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_update(
            &PostV1PartnersUpdateRequest {
                id: "id".to_string(),
                r#type: None,
                name: None,
                code: None,
                vat_code: None,
                peppol_id: None,
                email: None,
                phone: None,
                self_employment_cert_no: None,
                birth_date: None,
                is_customer: None,
                is_supplier: None,
                payment_term_days: None,
                credit_limit: None,
                price_list_id: None,
                group_id: None,
                status_id: None,
                address: None,
                correspondence_address: None,
                notes: None,
                document_ref: None,
                short_name: None,
                website: None,
                fax: None,
                eori_code: None,
                other_code: None,
                foreign_tax_number: None,
                auto_debt_reminder: None,
                late_interest_percent: None,
                first_call_date: None,
                last_call_date: None,
                next_call_date: None,
                rating: None,
                is_employee: None,
                is_group_member: None,
                is_active: None,
                legal_country_class: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<PostV1PartnersUpdateRequestType>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**peppol_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**phone:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**self_employment_cert_no:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**birth_date:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**is_customer:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_supplier:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**payment_term_days:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**credit_limit:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**price_list_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**group_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**status_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**address:** `Option<Option<PostV1PartnersUpdateRequestAddress>>` 
    
</dd>
</dl>

<dl>
<dd>

**correspondence_address:** `Option<Option<PostV1PartnersUpdateRequestCorrespondenceAddress>>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**document_ref:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**short_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**website:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**fax:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**eori_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**other_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**foreign_tax_number:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**auto_debt_reminder:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**late_interest_percent:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**first_call_date:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**last_call_date:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**next_call_date:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**rating:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**is_employee:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_group_member:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**legal_country_class:** `Option<Option<PostV1PartnersUpdateRequestLegalCountryClass>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_delete</a>(request: PostV1PartnersDeleteRequest) -> Result&lt;PostV1PartnersDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_delete(
            &PostV1PartnersDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">blank_a_partners_personal_data_and_hide_the_record</a>(request: PostV1PartnersAnonymizeRequest) -> Result&lt;PostV1PartnersAnonymizeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Removes birth date, self-employment certificate number, email, phone, address, notes, contacts, addresses and bank accounts, then hides the partner. The name, code and VAT number stay because issued invoices must keep identifying the counterparty for the statutory retention period.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .blank_a_partners_personal_data_and_hide_the_record(
            &PostV1PartnersAnonymizeRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_list</a>(request: PostV1PartnersListRequest) -> Result&lt;PostV1PartnersListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_list(
            &PostV1PartnersListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1PartnersListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1PartnersListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_groups_create</a>(request: PostV1PartnersGroupsCreateRequest) -> Result&lt;PostV1PartnersGroupsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_groups_create(
            &PostV1PartnersGroupsCreateRequest {
                code: "code".to_string(),
                name: "name".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_groups_update</a>(request: PostV1PartnersGroupsUpdateRequest) -> Result&lt;PostV1PartnersGroupsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_groups_update(
            &PostV1PartnersGroupsUpdateRequest {
                id: "id".to_string(),
                code: None,
                name: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_groups_delete</a>(request: PostV1PartnersGroupsDeleteRequest) -> Result&lt;PostV1PartnersGroupsDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_groups_delete(
            &PostV1PartnersGroupsDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_groups_list</a>(request: PostV1PartnersGroupsListRequest) -> Result&lt;PostV1PartnersGroupsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_groups_list(
            &PostV1PartnersGroupsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_statuses_create</a>(request: PostV1PartnersStatusesCreateRequest) -> Result&lt;PostV1PartnersStatusesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_statuses_create(
            &PostV1PartnersStatusesCreateRequest {
                code: "code".to_string(),
                name: "name".to_string(),
                sort_order: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**sort_order:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_statuses_update</a>(request: PostV1PartnersStatusesUpdateRequest) -> Result&lt;PostV1PartnersStatusesUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_statuses_update(
            &PostV1PartnersStatusesUpdateRequest {
                id: "id".to_string(),
                code: None,
                name: None,
                sort_order: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort_order:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_statuses_delete</a>(request: PostV1PartnersStatusesDeleteRequest) -> Result&lt;PostV1PartnersStatusesDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_statuses_delete(
            &PostV1PartnersStatusesDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_statuses_list</a>(request: PostV1PartnersStatusesListRequest) -> Result&lt;PostV1PartnersStatusesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_statuses_list(
            &PostV1PartnersStatusesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_inquiries_create</a>(request: PostV1PartnersInquiriesCreateRequest) -> Result&lt;PostV1PartnersInquiriesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_inquiries_create(
            &PostV1PartnersInquiriesCreateRequest {
                subject: "subject".to_string(),
                partner_id: None,
                contact_name: None,
                contact_email: None,
                contact_phone: None,
                body: None,
                channel: None,
                assigned_user_id: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**partner_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**contact_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**contact_email:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**contact_phone:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**subject:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**body:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**channel:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**assigned_user_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_inquiries_update</a>(request: PostV1PartnersInquiriesUpdateRequest) -> Result&lt;PostV1PartnersInquiriesUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_inquiries_update(
            &PostV1PartnersInquiriesUpdateRequest {
                id: "id".to_string(),
                partner_id: None,
                subject: None,
                body: None,
                channel: None,
                status: None,
                assigned_user_id: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**partner_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**subject:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**body:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**channel:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<PostV1PartnersInquiriesUpdateRequestStatus>` 
    
</dd>
</dl>

<dl>
<dd>

**assigned_user_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_inquiries_get</a>(request: PostV1PartnersInquiriesGetRequest) -> Result&lt;PostV1PartnersInquiriesGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_inquiries_get(
            &PostV1PartnersInquiriesGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_inquiries_list</a>(request: PostV1PartnersInquiriesListRequest) -> Result&lt;PostV1PartnersInquiriesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_inquiries_list(
            &PostV1PartnersInquiriesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1PartnersInquiriesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1PartnersInquiriesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_partners_credit_check</a>(request: PostV1PartnersCreditCheckRequest) -> Result&lt;PostV1PartnersCreditCheckResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1partners_credit_check(
            &PostV1PartnersCreditCheckRequest {
                partner_id: "partnerId".to_string(),
                additional_amount: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**partner_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**additional_amount:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_leads_create</a>(request: PostV1LeadsCreateRequest) -> Result&lt;PostV1LeadsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1leads_create(
            &PostV1LeadsCreateRequest {
                name: "name".to_string(),
                contact_name: None,
                email: None,
                phone: None,
                website: None,
                country_code: None,
                source_id: None,
                status: None,
                estimated_value: None,
                currency: None,
                description: None,
                assigned_user_id: None,
                documents: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**contact_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**phone:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**website:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**country_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**source_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<PostV1LeadsCreateRequestStatus>` 
    
</dd>
</dl>

<dl>
<dd>

**estimated_value:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**assigned_user_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**documents:** `Option<Vec<PostV1LeadsCreateRequestDocumentsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<Vec<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_leads_get</a>(request: PostV1LeadsGetRequest) -> Result&lt;PostV1LeadsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1leads_get(
            &PostV1LeadsGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_leads_update</a>(request: PostV1LeadsUpdateRequest) -> Result&lt;PostV1LeadsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1leads_update(
            &PostV1LeadsUpdateRequest {
                id: "id".to_string(),
                name: None,
                contact_name: None,
                email: None,
                phone: None,
                website: None,
                country_code: None,
                source_id: None,
                status: None,
                estimated_value: None,
                currency: None,
                description: None,
                assigned_user_id: None,
                documents: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**contact_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**phone:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**website:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**country_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**source_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<PostV1LeadsUpdateRequestStatus>` 
    
</dd>
</dl>

<dl>
<dd>

**estimated_value:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**assigned_user_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**documents:** `Option<Vec<PostV1LeadsUpdateRequestDocumentsItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_leads_delete</a>(request: PostV1LeadsDeleteRequest) -> Result&lt;PostV1LeadsDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1leads_delete(
            &PostV1LeadsDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_leads_list</a>(request: PostV1LeadsListRequest) -> Result&lt;PostV1LeadsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1leads_list(
            &PostV1LeadsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1LeadsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1LeadsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_leads_notes_create</a>(request: PostV1LeadsNotesCreateRequest) -> Result&lt;PostV1LeadsNotesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1leads_notes_create(
            &PostV1LeadsNotesCreateRequest {
                lead_id: "leadId".to_string(),
                body: "body".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**lead_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**body:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_leads_notes_delete</a>(request: PostV1LeadsNotesDeleteRequest) -> Result&lt;PostV1LeadsNotesDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1leads_notes_delete(
            &PostV1LeadsNotesDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_leads_notes_list</a>(request: PostV1LeadsNotesListRequest) -> Result&lt;PostV1LeadsNotesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1leads_notes_list(
            &PostV1LeadsNotesListRequest {
                lead_id: "leadId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**lead_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_leads_files_list</a>(request: PostV1LeadsFilesListRequest) -> Result&lt;PostV1LeadsFilesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1leads_files_list(
            &PostV1LeadsFilesListRequest {
                lead_id: "leadId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**lead_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_leads_sources_create</a>(request: PostV1LeadsSourcesCreateRequest) -> Result&lt;PostV1LeadsSourcesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1leads_sources_create(
            &PostV1LeadsSourcesCreateRequest {
                name: "name".to_string(),
                is_active: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_leads_sources_update</a>(request: PostV1LeadsSourcesUpdateRequest) -> Result&lt;PostV1LeadsSourcesUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1leads_sources_update(
            &PostV1LeadsSourcesUpdateRequest {
                id: "id".to_string(),
                name: None,
                is_active: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_leads_sources_delete</a>(request: PostV1LeadsSourcesDeleteRequest) -> Result&lt;PostV1LeadsSourcesDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1leads_sources_delete(
            &PostV1LeadsSourcesDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_leads_sources_list</a>(request: PostV1LeadsSourcesListRequest) -> Result&lt;PostV1LeadsSourcesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1leads_sources_list(
            &PostV1LeadsSourcesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_leads_sources_options</a>(request: PostV1LeadsSourcesOptionsRequest) -> Result&lt;PostV1LeadsSourcesOptionsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1leads_sources_options(
            &PostV1LeadsSourcesOptionsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">post_v1_leads_convert</a>(request: PostV1LeadsConvertRequest) -> Result&lt;PostV1LeadsConvertResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a customer partner from the lead, move the lead files to the partner, copy the lead notes into the partner notes and mark the lead as converted.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .partners
        .post_v1leads_convert(
            &PostV1LeadsConvertRequest {
                id: "id".to_string(),
                partner_type: None,
                code: None,
                vat_code: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**partner_type:** `Option<PostV1LeadsConvertRequestPartnerType>` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_code:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Catalog
<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_items_create</a>(request: PostV1CatalogItemsCreateRequest) -> Result&lt;PostV1CatalogItemsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_items_create(
            &PostV1CatalogItemsCreateRequest {
                name: "name".to_string(),
                r#type: None,
                tracking: None,
                code: None,
                barcode: None,
                unit: None,
                vat_classifier_code: None,
                vat_rate_percent: None,
                sale_price_excl_vat: None,
                purchase_price_excl_vat: None,
                cn_code: None,
                origin_country: None,
                net_mass_kg: None,
                supplementary_unit: None,
                supplementary_qty_per_unit: None,
                description: None,
                group_id: None,
                attributes: None,
                document_ref: None,
                translations: None,
                components: None,
                kind_id: None,
                sale_account_code: None,
                purchase_account_code: None,
                expense_account_code: None,
                manufacturer: None,
                gross_mass_kg: None,
                min_quantity: None,
                cost_price: None,
                is_free_price: None,
                external_id: None,
                is_returnable: None,
                comment_required: None,
                price_from: None,
                price_to: None,
                min_price: None,
                discount_percent: None,
                max_discount_percent: None,
                loyalty_points: None,
                department: None,
                age_restriction: None,
                package_quantity: None,
                tara_code: None,
                certificate_number: None,
                certificate_date: None,
                valid_from: None,
                valid_to: None,
                pos_flags: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**type_:** `Option<PostV1CatalogItemsCreateRequestType>` 
    
</dd>
</dl>

<dl>
<dd>

**tracking:** `Option<PostV1CatalogItemsCreateRequestTracking>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**barcode:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**unit:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_classifier_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_rate_percent:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sale_price_excl_vat:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**purchase_price_excl_vat:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**cn_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**origin_country:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**net_mass_kg:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**supplementary_unit:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**supplementary_qty_per_unit:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**group_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**attributes:** `Option<std::collections::HashMap<String, String>>` 
    
</dd>
</dl>

<dl>
<dd>

**document_ref:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**translations:** `Option<std::collections::HashMap<String, PostV1CatalogItemsCreateRequestTranslationsValue>>` 
    
</dd>
</dl>

<dl>
<dd>

**components:** `Option<Vec<PostV1CatalogItemsCreateRequestComponentsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**kind_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sale_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**purchase_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**expense_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**manufacturer:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**gross_mass_kg:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**min_quantity:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**cost_price:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_free_price:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**external_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_returnable:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**comment_required:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**price_from:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**price_to:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**min_price:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**discount_percent:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**max_discount_percent:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**loyalty_points:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**department:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**age_restriction:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**package_quantity:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**tara_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**certificate_number:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**certificate_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**valid_from:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**valid_to:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**pos_flags:** `Option<std::collections::HashMap<String, bool>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_items_get</a>(request: PostV1CatalogItemsGetRequest) -> Result&lt;PostV1CatalogItemsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_items_get(
            &PostV1CatalogItemsGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_items_update</a>(request: PostV1CatalogItemsUpdateRequest) -> Result&lt;PostV1CatalogItemsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_items_update(
            &PostV1CatalogItemsUpdateRequest {
                id: "id".to_string(),
                r#type: None,
                tracking: None,
                name: None,
                code: None,
                barcode: None,
                unit: None,
                vat_classifier_code: None,
                vat_rate_percent: None,
                sale_price_excl_vat: None,
                purchase_price_excl_vat: None,
                cn_code: None,
                origin_country: None,
                net_mass_kg: None,
                supplementary_unit: None,
                supplementary_qty_per_unit: None,
                description: None,
                group_id: None,
                attributes: None,
                document_ref: None,
                translations: None,
                components: None,
                kind_id: None,
                sale_account_code: None,
                purchase_account_code: None,
                expense_account_code: None,
                manufacturer: None,
                gross_mass_kg: None,
                min_quantity: None,
                cost_price: None,
                is_free_price: None,
                external_id: None,
                is_returnable: None,
                comment_required: None,
                price_from: None,
                price_to: None,
                min_price: None,
                discount_percent: None,
                max_discount_percent: None,
                loyalty_points: None,
                department: None,
                age_restriction: None,
                package_quantity: None,
                tara_code: None,
                certificate_number: None,
                certificate_date: None,
                valid_from: None,
                valid_to: None,
                pos_flags: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<PostV1CatalogItemsUpdateRequestType>` 
    
</dd>
</dl>

<dl>
<dd>

**tracking:** `Option<PostV1CatalogItemsUpdateRequestTracking>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**barcode:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**unit:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_classifier_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_rate_percent:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**sale_price_excl_vat:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**purchase_price_excl_vat:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**cn_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**origin_country:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**net_mass_kg:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**supplementary_unit:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**supplementary_qty_per_unit:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**group_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**attributes:** `Option<Option<std::collections::HashMap<String, Option<String>>>>` 
    
</dd>
</dl>

<dl>
<dd>

**document_ref:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**translations:** `Option<Option<std::collections::HashMap<String, Option<PostV1CatalogItemsUpdateRequestTranslationsValue>>>>` 
    
</dd>
</dl>

<dl>
<dd>

**components:** `Option<Vec<PostV1CatalogItemsUpdateRequestComponentsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**kind_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**sale_account_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**purchase_account_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**expense_account_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**manufacturer:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**gross_mass_kg:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**min_quantity:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**cost_price:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**is_free_price:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**external_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**is_returnable:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**comment_required:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**price_from:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**price_to:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**min_price:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**discount_percent:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**max_discount_percent:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**loyalty_points:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**department:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**age_restriction:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**package_quantity:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**tara_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**certificate_number:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**certificate_date:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**valid_from:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**valid_to:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**pos_flags:** `Option<Option<std::collections::HashMap<String, Option<bool>>>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_items_delete</a>(request: PostV1CatalogItemsDeleteRequest) -> Result&lt;PostV1CatalogItemsDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_items_delete(
            &PostV1CatalogItemsDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_items_list</a>(request: PostV1CatalogItemsListRequest) -> Result&lt;PostV1CatalogItemsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_items_list(
            &PostV1CatalogItemsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1CatalogItemsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1CatalogItemsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_items_files_list</a>(request: PostV1CatalogItemsFilesListRequest) -> Result&lt;PostV1CatalogItemsFilesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_items_files_list(
            &PostV1CatalogItemsFilesListRequest {
                item_id: "itemId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**item_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_items_kinds_create</a>(request: PostV1CatalogItemsKindsCreateRequest) -> Result&lt;PostV1CatalogItemsKindsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_items_kinds_create(
            &PostV1CatalogItemsKindsCreateRequest {
                code: "code".to_string(),
                name: "name".to_string(),
                saft_type: None,
                quantity_accounting: None,
                sort_order: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**saft_type:** `Option<PostV1CatalogItemsKindsCreateRequestSaftType>` 
    
</dd>
</dl>

<dl>
<dd>

**quantity_accounting:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**sort_order:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_items_kinds_update</a>(request: PostV1CatalogItemsKindsUpdateRequest) -> Result&lt;PostV1CatalogItemsKindsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_items_kinds_update(
            &PostV1CatalogItemsKindsUpdateRequest {
                id: "id".to_string(),
                code: None,
                name: None,
                saft_type: None,
                quantity_accounting: None,
                sort_order: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**saft_type:** `Option<PostV1CatalogItemsKindsUpdateRequestSaftType>` 
    
</dd>
</dl>

<dl>
<dd>

**quantity_accounting:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**sort_order:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_items_kinds_delete</a>(request: PostV1CatalogItemsKindsDeleteRequest) -> Result&lt;PostV1CatalogItemsKindsDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_items_kinds_delete(
            &PostV1CatalogItemsKindsDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_items_kinds_list</a>(request: PostV1CatalogItemsKindsListRequest) -> Result&lt;PostV1CatalogItemsKindsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_items_kinds_list(
            &PostV1CatalogItemsKindsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_units_create</a>(request: PostV1CatalogUnitsCreateRequest) -> Result&lt;PostV1CatalogUnitsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_units_create(
            &PostV1CatalogUnitsCreateRequest {
                code: "code".to_string(),
                name: "name".to_string(),
                is_active: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_units_update</a>(request: PostV1CatalogUnitsUpdateRequest) -> Result&lt;PostV1CatalogUnitsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_units_update(
            &PostV1CatalogUnitsUpdateRequest {
                id: "id".to_string(),
                code: None,
                name: None,
                is_active: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_units_delete</a>(request: PostV1CatalogUnitsDeleteRequest) -> Result&lt;PostV1CatalogUnitsDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_units_delete(
            &PostV1CatalogUnitsDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_units_list</a>(request: PostV1CatalogUnitsListRequest) -> Result&lt;PostV1CatalogUnitsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_units_list(
            &PostV1CatalogUnitsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_units_options</a>(request: PostV1CatalogUnitsOptionsRequest) -> Result&lt;PostV1CatalogUnitsOptionsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_units_options(
            &PostV1CatalogUnitsOptionsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**locale:** `Option<PostV1CatalogUnitsOptionsRequestLocale>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_item_groups_create</a>(request: PostV1CatalogItemGroupsCreateRequest) -> Result&lt;PostV1CatalogItemGroupsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_item_groups_create(
            &PostV1CatalogItemGroupsCreateRequest {
                code: "code".to_string(),
                name: "name".to_string(),
                parent_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**parent_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_item_groups_update</a>(request: PostV1CatalogItemGroupsUpdateRequest) -> Result&lt;PostV1CatalogItemGroupsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_item_groups_update(
            &PostV1CatalogItemGroupsUpdateRequest {
                id: "id".to_string(),
                code: None,
                name: None,
                parent_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**parent_id:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_item_groups_delete</a>(request: PostV1CatalogItemGroupsDeleteRequest) -> Result&lt;PostV1CatalogItemGroupsDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_item_groups_delete(
            &PostV1CatalogItemGroupsDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_item_groups_list</a>(request: PostV1CatalogItemGroupsListRequest) -> Result&lt;PostV1CatalogItemGroupsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_item_groups_list(
            &PostV1CatalogItemGroupsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_items_suppliers_upsert</a>(request: PostV1CatalogItemsSuppliersUpsertRequest) -> Result&lt;PostV1CatalogItemsSuppliersUpsertResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_items_suppliers_upsert(
            &PostV1CatalogItemsSuppliersUpsertRequest {
                item_id: "itemId".to_string(),
                partner_id: "partnerId".to_string(),
                supplier_code: None,
                purchase_price_excl_vat: None,
                currency: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**item_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**partner_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**supplier_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**purchase_price_excl_vat:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_items_suppliers_list</a>(request: PostV1CatalogItemsSuppliersListRequest) -> Result&lt;PostV1CatalogItemsSuppliersListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_items_suppliers_list(
            &PostV1CatalogItemsSuppliersListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**item_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**partner_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_items_suppliers_delete</a>(request: PostV1CatalogItemsSuppliersDeleteRequest) -> Result&lt;PostV1CatalogItemsSuppliersDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_items_suppliers_delete(
            &PostV1CatalogItemsSuppliersDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_price_lists_create</a>(request: PostV1CatalogPriceListsCreateRequest) -> Result&lt;PostV1CatalogPriceListsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_price_lists_create(
            &PostV1CatalogPriceListsCreateRequest {
                code: "code".to_string(),
                name: "name".to_string(),
                currency: None,
                is_active: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_price_lists_update</a>(request: PostV1CatalogPriceListsUpdateRequest) -> Result&lt;PostV1CatalogPriceListsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_price_lists_update(
            &PostV1CatalogPriceListsUpdateRequest {
                id: "id".to_string(),
                code: None,
                name: None,
                currency: None,
                is_active: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_price_lists_list</a>(request: PostV1CatalogPriceListsListRequest) -> Result&lt;PostV1CatalogPriceListsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_price_lists_list(
            &PostV1CatalogPriceListsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_price_lists_items_set</a>(request: PostV1CatalogPriceListsItemsSetRequest) -> Result&lt;PostV1CatalogPriceListsItemsSetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_price_lists_items_set(
            &PostV1CatalogPriceListsItemsSetRequest {
                price_list_id: "priceListId".to_string(),
                items: vec![PostV1CatalogPriceListsItemsSetRequestItemsItem {
                    item_id: "itemId".to_string(),
                    unit_price_excl_vat: "unitPriceExclVat".to_string(),
                    ..Default::default()
                }],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**price_list_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**items:** `Vec<PostV1CatalogPriceListsItemsSetRequestItemsItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_price_lists_items_list</a>(request: PostV1CatalogPriceListsItemsListRequest) -> Result&lt;PostV1CatalogPriceListsItemsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_price_lists_items_list(
            &PostV1CatalogPriceListsItemsListRequest {
                price_list_id: "priceListId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**price_list_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">post_v1_catalog_price_lists_items_delete</a>(request: PostV1CatalogPriceListsItemsDeleteRequest) -> Result&lt;PostV1CatalogPriceListsItemsDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .catalog
        .post_v1catalog_price_lists_items_delete(
            &PostV1CatalogPriceListsItemsDeleteRequest {
                price_list_id: "priceListId".to_string(),
                item_id: "itemId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**price_list_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**item_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Sales
<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_invoices_create</a>(request: PostV1SalesInvoicesCreateRequest) -> Result&lt;PostV1SalesInvoicesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_invoices_create(
            &PostV1SalesInvoicesCreateRequest {
                partner_id: "partnerId".to_string(),
                lines: vec![PostV1SalesInvoicesCreateRequestLinesItem {
                    ..Default::default()
                }],
                r#type: None,
                currency: None,
                issue_date: None,
                due_date: None,
                credited_invoice_id: None,
                agreement_id: None,
                vat_scheme: None,
                intrastat_transport_mode: None,
                intrastat_delivery_terms: None,
                intrastat_region: None,
                intrastat_nature_of_transaction: None,
                vat_country_code: None,
                deemed_supplier: None,
                notes: None,
                document_ref: None,
                operation_type_id: None,
                document_series_id: None,
                series_label: None,
                order_number: None,
                issued_by_name: None,
                issued_by_title: None,
                received_by_name: None,
                received_by_title: None,
                discount_percent: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**partner_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<PostV1SalesInvoicesCreateRequestType>` 
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**issue_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**due_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**credited_invoice_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**agreement_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_scheme:** `Option<PostV1SalesInvoicesCreateRequestVatScheme>` 
    
</dd>
</dl>

<dl>
<dd>

**intrastat_transport_mode:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**intrastat_delivery_terms:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**intrastat_region:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**intrastat_nature_of_transaction:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_country_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**deemed_supplier:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**document_ref:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**operation_type_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**document_series_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**series_label:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**order_number:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**issued_by_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**issued_by_title:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**received_by_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**received_by_title:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**discount_percent:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Vec<PostV1SalesInvoicesCreateRequestLinesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_invoices_get</a>(request: PostV1SalesInvoicesGetRequest) -> Result&lt;PostV1SalesInvoicesGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_invoices_get(
            &PostV1SalesInvoicesGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_invoices_pdf</a>(request: PostV1SalesInvoicesPdfRequest) -> Result&lt;PostV1SalesInvoicesPdfResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_invoices_pdf(
            &PostV1SalesInvoicesPdfRequest {
                id: "id".to_string(),
                locale: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**locale:** `Option<PostV1SalesInvoicesPdfRequestLocale>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_invoices_send</a>(request: PostV1SalesInvoicesSendRequest) -> Result&lt;PostV1SalesInvoicesSendResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_invoices_send(
            &PostV1SalesInvoicesSendRequest {
                id: "id".to_string(),
                to: None,
                locale: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**locale:** `Option<PostV1SalesInvoicesSendRequestLocale>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_invoices_peppol_xml</a>(request: PostV1SalesInvoicesPeppolXmlRequest) -> Result&lt;PostV1SalesInvoicesPeppolXmlResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_invoices_peppol_xml(
            &PostV1SalesInvoicesPeppolXMLRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_invoices_peppol_send</a>(request: PostV1SalesInvoicesPeppolSendRequest) -> Result&lt;PostV1SalesInvoicesPeppolSendResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_invoices_peppol_send(
            &PostV1SalesInvoicesPeppolSendRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_invoices_einvoice_xml</a>(request: PostV1SalesInvoicesEinvoiceXmlRequest) -> Result&lt;PostV1SalesInvoicesEinvoiceXmlResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Render an issued invoice as the national e-invoicing payload for the company country: FatturaPA (IT), KSeF FA(3) (PL) or UBL CIUS-RO (RO). Review the warnings - data the invoice does not carry is flagged, never invented.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_invoices_einvoice_xml(
            &PostV1SalesInvoicesEinvoiceXMLRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_invoices_einvoice_send</a>(request: PostV1SalesInvoicesEinvoiceSendRequest) -> Result&lt;PostV1SalesInvoicesEinvoiceSendResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Build the national e-invoicing payload and deliver it over the transport configured for the country gateway in compliance settings. With transport=direct the request talks to the tax authority itself - SdICoop over 2-way TLS for Italy, a KSeF session for Poland, ANAF SPV OAuth for Romania - and returns the national number as soon as the channel assigns one. With transport=bridge the payload goes to the configured bridge endpoint (an accredited intermediary or connector) instead.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_invoices_einvoice_send(
            &PostV1SalesInvoicesEinvoiceSendRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_invoices_einvoice_status</a>(request: PostV1SalesInvoicesEinvoiceStatusRequest) -> Result&lt;PostV1SalesInvoicesEinvoiceStatusResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Ask the national e-invoicing channel what happened to an invoice that was already sent, and store the answer. Italy, Poland and Romania return the outcome only on request - none of them calls back - so this is the way the national number and any rejection reason reach the invoice.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_invoices_einvoice_status(
            &PostV1SalesInvoicesEinvoiceStatusRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_invoices_update</a>(request: PostV1SalesInvoicesUpdateRequest) -> Result&lt;PostV1SalesInvoicesUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_invoices_update(
            &PostV1SalesInvoicesUpdateRequest {
                id: "id".to_string(),
                partner_id: None,
                agreement_id: None,
                currency: None,
                issue_date: None,
                due_date: None,
                vat_scheme: None,
                intrastat_transport_mode: None,
                intrastat_delivery_terms: None,
                intrastat_region: None,
                intrastat_nature_of_transaction: None,
                vat_country_code: None,
                deemed_supplier: None,
                notes: None,
                operation_type_id: None,
                document_series_id: None,
                series_label: None,
                discount_percent: None,
                order_number: None,
                issued_by_name: None,
                issued_by_title: None,
                received_by_name: None,
                received_by_title: None,
                lines: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**partner_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**agreement_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**issue_date:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**due_date:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_scheme:** `Option<Option<PostV1SalesInvoicesUpdateRequestVatScheme>>` 
    
</dd>
</dl>

<dl>
<dd>

**intrastat_transport_mode:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**intrastat_delivery_terms:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**intrastat_region:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**intrastat_nature_of_transaction:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_country_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**deemed_supplier:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**operation_type_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**document_series_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**series_label:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**discount_percent:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**order_number:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**issued_by_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**issued_by_title:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**received_by_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**received_by_title:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Option<Vec<PostV1SalesInvoicesUpdateRequestLinesItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_invoices_delete</a>(request: PostV1SalesInvoicesDeleteRequest) -> Result&lt;PostV1SalesInvoicesDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_invoices_delete(
            &PostV1SalesInvoicesDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_invoices_issue</a>(request: PostV1SalesInvoicesIssueRequest) -> Result&lt;PostV1SalesInvoicesIssueResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_invoices_issue(
            &PostV1SalesInvoicesIssueRequest {
                id: "id".to_string(),
                series: None,
                issue_date: None,
                warehouse_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**series:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**issue_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**warehouse_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_invoices_lock</a>(request: PostV1SalesInvoicesLockRequest) -> Result&lt;PostV1SalesInvoicesLockResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_invoices_lock(
            &PostV1SalesInvoicesLockRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_invoices_unlock</a>(request: PostV1SalesInvoicesUnlockRequest) -> Result&lt;PostV1SalesInvoicesUnlockResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_invoices_unlock(
            &PostV1SalesInvoicesUnlockRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_invoices_payment_link</a>(request: PostV1SalesInvoicesPaymentLinkRequest) -> Result&lt;PostV1SalesInvoicesPaymentLinkResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_invoices_payment_link(
            &PostV1SalesInvoicesPaymentLinkRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_invoices_payment_settings_get</a>(request: PostV1SalesInvoicesPaymentSettingsGetRequest) -> Result&lt;PostV1SalesInvoicesPaymentSettingsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_invoices_payment_settings_get(
            &PostV1SalesInvoicesPaymentSettingsGetRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_invoices_payment_settings_update</a>(request: PostV1SalesInvoicesPaymentSettingsUpdateRequest) -> Result&lt;PostV1SalesInvoicesPaymentSettingsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_invoices_payment_settings_update(
            &PostV1SalesInvoicesPaymentSettingsUpdateRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**payment_link_template:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_recognition_schedules_list</a>(request: PostV1SalesRecognitionSchedulesListRequest) -> Result&lt;PostV1SalesRecognitionSchedulesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_recognition_schedules_list(
            &PostV1SalesRecognitionSchedulesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1SalesRecognitionSchedulesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1SalesRecognitionSchedulesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_invoices_apply_advance</a>(request: PostV1SalesInvoicesApplyAdvanceRequest) -> Result&lt;PostV1SalesInvoicesApplyAdvanceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_invoices_apply_advance(
            &PostV1SalesInvoicesApplyAdvanceRequest {
                advance_id: "advanceId".to_string(),
                invoice_id: "invoiceId".to_string(),
                date: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**advance_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**invoice_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_invoices_list</a>(request: PostV1SalesInvoicesListRequest) -> Result&lt;PostV1SalesInvoicesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_invoices_list(
            &PostV1SalesInvoicesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1SalesInvoicesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1SalesInvoicesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_acts_create</a>(request: PostV1SalesActsCreateRequest) -> Result&lt;PostV1SalesActsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_acts_create(
            &PostV1SalesActsCreateRequest {
                partner_id: "partnerId".to_string(),
                r#type: None,
                document_date: None,
                sale_invoice_id: None,
                transferred_by_name: None,
                transferred_by_title: None,
                accepted_by_name: None,
                accepted_by_title: None,
                notes: None,
                series: None,
                lines: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**partner_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<PostV1SalesActsCreateRequestType>` 
    
</dd>
</dl>

<dl>
<dd>

**document_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sale_invoice_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**transferred_by_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**transferred_by_title:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**accepted_by_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**accepted_by_title:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**series:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Option<Vec<PostV1SalesActsCreateRequestLinesItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_acts_update</a>(request: PostV1SalesActsUpdateRequest) -> Result&lt;PostV1SalesActsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_acts_update(
            &PostV1SalesActsUpdateRequest {
                id: "id".to_string(),
                partner_id: None,
                r#type: None,
                document_date: None,
                sale_invoice_id: None,
                transferred_by_name: None,
                transferred_by_title: None,
                accepted_by_name: None,
                accepted_by_title: None,
                notes: None,
                series: None,
                lines: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**partner_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<PostV1SalesActsUpdateRequestType>` 
    
</dd>
</dl>

<dl>
<dd>

**document_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sale_invoice_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**transferred_by_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**transferred_by_title:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**accepted_by_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**accepted_by_title:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**series:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Option<Vec<PostV1SalesActsUpdateRequestLinesItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_acts_issue</a>(request: PostV1SalesActsIssueRequest) -> Result&lt;PostV1SalesActsIssueResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_acts_issue(
            &PostV1SalesActsIssueRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_acts_cancel</a>(request: PostV1SalesActsCancelRequest) -> Result&lt;PostV1SalesActsCancelResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_acts_cancel(
            &PostV1SalesActsCancelRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_acts_get</a>(request: PostV1SalesActsGetRequest) -> Result&lt;PostV1SalesActsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_acts_get(
            &PostV1SalesActsGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_acts_list</a>(request: PostV1SalesActsListRequest) -> Result&lt;PostV1SalesActsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_acts_list(
            &PostV1SalesActsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1SalesActsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1SalesActsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_acts_pdf</a>(request: PostV1SalesActsPdfRequest) -> Result&lt;PostV1SalesActsPdfResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_acts_pdf(
            &PostV1SalesActsPdfRequest {
                id: "id".to_string(),
                locale: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**locale:** `Option<PostV1SalesActsPdfRequestLocale>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_operation_types_create</a>(request: PostV1OperationTypesCreateRequest) -> Result&lt;PostV1OperationTypesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1operation_types_create(
            &PostV1OperationTypesCreateRequest {
                code: "code".to_string(),
                name: "name".to_string(),
                invoice_type: None,
                payer_partner_id: None,
                debit_account_code: None,
                credit_account_code: None,
                vat_account_code: None,
                expense_account_code: None,
                advance_account_code: None,
                income_account_code: None,
                is_purchase: None,
                is_sale: None,
                is_write_off: None,
                is_internal_movement: None,
                is_purchase_return: None,
                is_sales_return: None,
                is_consignment: None,
                is_production: None,
                is_asset_in: None,
                is_asset_out: None,
                is_cash_register_sale: None,
                include_in_vat_register: None,
                include_in_saft: None,
                is_active: None,
                sort_order: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**invoice_type:** `Option<Option<PostV1OperationTypesCreateRequestInvoiceType>>` 
    
</dd>
</dl>

<dl>
<dd>

**payer_partner_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**debit_account_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**credit_account_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_account_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**expense_account_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**advance_account_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**income_account_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**is_purchase:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_sale:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_write_off:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_internal_movement:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_purchase_return:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_sales_return:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_consignment:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_production:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_asset_in:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_asset_out:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_cash_register_sale:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**include_in_vat_register:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**include_in_saft:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**sort_order:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_operation_types_update</a>(request: PostV1OperationTypesUpdateRequest) -> Result&lt;PostV1OperationTypesUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1operation_types_update(
            &PostV1OperationTypesUpdateRequest {
                id: "id".to_string(),
                code: None,
                name: None,
                invoice_type: None,
                payer_partner_id: None,
                debit_account_code: None,
                credit_account_code: None,
                vat_account_code: None,
                expense_account_code: None,
                advance_account_code: None,
                income_account_code: None,
                is_purchase: None,
                is_sale: None,
                is_write_off: None,
                is_internal_movement: None,
                is_purchase_return: None,
                is_sales_return: None,
                is_consignment: None,
                is_production: None,
                is_asset_in: None,
                is_asset_out: None,
                is_cash_register_sale: None,
                include_in_vat_register: None,
                include_in_saft: None,
                is_active: None,
                sort_order: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**invoice_type:** `Option<Option<PostV1OperationTypesUpdateRequestInvoiceType>>` 
    
</dd>
</dl>

<dl>
<dd>

**payer_partner_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**debit_account_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**credit_account_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_account_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**expense_account_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**advance_account_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**income_account_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**is_purchase:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_sale:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_write_off:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_internal_movement:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_purchase_return:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_sales_return:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_consignment:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_production:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_asset_in:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_asset_out:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_cash_register_sale:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**include_in_vat_register:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**include_in_saft:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**sort_order:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_operation_types_get</a>(request: PostV1OperationTypesGetRequest) -> Result&lt;PostV1OperationTypesGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1operation_types_get(
            &PostV1OperationTypesGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_operation_types_delete</a>(request: PostV1OperationTypesDeleteRequest) -> Result&lt;PostV1OperationTypesDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1operation_types_delete(
            &PostV1OperationTypesDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_operation_types_list</a>(request: PostV1OperationTypesListRequest) -> Result&lt;PostV1OperationTypesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1operation_types_list(
            &PostV1OperationTypesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1OperationTypesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1OperationTypesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_document_series_create</a>(request: PostV1DocumentSeriesCreateRequest) -> Result&lt;PostV1DocumentSeriesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1document_series_create(
            &PostV1DocumentSeriesCreateRequest {
                prefix: "prefix".to_string(),
                document_type: None,
                name: None,
                label: None,
                operation_type_id: None,
                number_length: None,
                next_number: None,
                allocated_from: None,
                allocated_to: None,
                warehouse_id: None,
                print_series: None,
                is_default: None,
                is_active: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**document_type:** `Option<PostV1DocumentSeriesCreateRequestDocumentType>` 
    
</dd>
</dl>

<dl>
<dd>

**prefix:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**label:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**operation_type_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**number_length:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**next_number:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**allocated_from:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**allocated_to:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**warehouse_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**print_series:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_default:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_document_series_update</a>(request: PostV1DocumentSeriesUpdateRequest) -> Result&lt;PostV1DocumentSeriesUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1document_series_update(
            &PostV1DocumentSeriesUpdateRequest {
                id: "id".to_string(),
                document_type: None,
                prefix: None,
                name: None,
                label: None,
                operation_type_id: None,
                number_length: None,
                next_number: None,
                allocated_from: None,
                allocated_to: None,
                warehouse_id: None,
                print_series: None,
                is_default: None,
                is_active: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**document_type:** `Option<PostV1DocumentSeriesUpdateRequestDocumentType>` 
    
</dd>
</dl>

<dl>
<dd>

**prefix:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**label:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**operation_type_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**number_length:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**next_number:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**allocated_from:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**allocated_to:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**warehouse_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**print_series:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_default:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_document_series_get</a>(request: PostV1DocumentSeriesGetRequest) -> Result&lt;PostV1DocumentSeriesGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1document_series_get(
            &PostV1DocumentSeriesGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_document_series_delete</a>(request: PostV1DocumentSeriesDeleteRequest) -> Result&lt;PostV1DocumentSeriesDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1document_series_delete(
            &PostV1DocumentSeriesDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_document_series_list</a>(request: PostV1DocumentSeriesListRequest) -> Result&lt;PostV1DocumentSeriesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1document_series_list(
            &PostV1DocumentSeriesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1DocumentSeriesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1DocumentSeriesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_recognition_compute</a>(request: PostV1SalesRecognitionComputeRequest) -> Result&lt;PostV1SalesRecognitionComputeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_recognition_compute(
            &PostV1SalesRecognitionComputeRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**as_of_date:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_recognition_run</a>(request: PostV1SalesRecognitionRunRequest) -> Result&lt;PostV1SalesRecognitionRunResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_recognition_run(
            &PostV1SalesRecognitionRunRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**as_of_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**posting_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**schedule_ids:** `Option<Vec<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_recognition_progress</a>(request: PostV1SalesRecognitionProgressRequest) -> Result&lt;PostV1SalesRecognitionProgressResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_recognition_progress(
            &PostV1SalesRecognitionProgressRequest {
                invoice_line_id: "invoiceLineId".to_string(),
                percent_complete: "percentComplete".to_string(),
                date: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**invoice_line_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**percent_complete:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_recognition_modify</a>(request: PostV1SalesRecognitionModifyRequest) -> Result&lt;PostV1SalesRecognitionModifyResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Apply an IFRS 15 contract modification to a deferred invoice line. Prospective: cancel the pending schedule and respread the unrecognized remainder over the new terms. Cumulative catch-up (ratable only): recompute revenue as if the new terms applied from the start and post the difference immediately.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_recognition_modify(
            &PostV1SalesRecognitionModifyRequest {
                invoice_line_id: "invoiceLineId".to_string(),
                approach: PostV1SalesRecognitionModifyRequestApproach::Prospective,
                date: None,
                new_end_date: None,
                new_milestones: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**invoice_line_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**approach:** `PostV1SalesRecognitionModifyRequestApproach` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**new_end_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**new_milestones:** `Option<Vec<PostV1SalesRecognitionModifyRequestNewMilestonesItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_recognition_runs_list</a>(request: PostV1SalesRecognitionRunsListRequest) -> Result&lt;PostV1SalesRecognitionRunsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_recognition_runs_list(
            &PostV1SalesRecognitionRunsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1SalesRecognitionRunsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1SalesRecognitionRunsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_recognition_summary</a>(request: PostV1SalesRecognitionSummaryRequest) -> Result&lt;PostV1SalesRecognitionSummaryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_recognition_summary(
            &PostV1SalesRecognitionSummaryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**invoice_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_refund_liability_list</a>(request: PostV1SalesRefundLiabilityListRequest) -> Result&lt;PostV1SalesRefundLiabilityListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_refund_liability_list(
            &PostV1SalesRefundLiabilityListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1SalesRefundLiabilityListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1SalesRefundLiabilityListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">post_v1_sales_refund_liability_true_up</a>(request: PostV1SalesRefundLiabilityTrueUpRequest) -> Result&lt;PostV1SalesRefundLiabilityTrueUpResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .sales
        .post_v1sales_refund_liability_true_up(
            &PostV1SalesRefundLiabilityTrueUpRequest {
                invoice_id: "invoiceId".to_string(),
                estimated_total: "estimatedTotal".to_string(),
                date: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**invoice_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**estimated_total:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Purchases
<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">post_v1_purchases_invoices_create</a>(request: PostV1PurchasesInvoicesCreateRequest) -> Result&lt;PostV1PurchasesInvoicesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .purchases
        .post_v1purchases_invoices_create(
            &PostV1PurchasesInvoicesCreateRequest {
                partner_id: "partnerId".to_string(),
                document_number: "documentNumber".to_string(),
                document_date: "documentDate".to_string(),
                lines: vec![PostV1PurchasesInvoicesCreateRequestLinesItem {
                    ..Default::default()
                }],
                r#type: None,
                due_date: None,
                currency: None,
                credited_invoice_id: None,
                purchase_order_id: None,
                operation_type_id: None,
                notes: None,
                intrastat_transport_mode: None,
                intrastat_delivery_terms: None,
                intrastat_region: None,
                intrastat_nature_of_transaction: None,
                einvoice_number: None,
                document_ref: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**partner_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<PostV1PurchasesInvoicesCreateRequestType>` 
    
</dd>
</dl>

<dl>
<dd>

**document_number:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**document_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**due_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**credited_invoice_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**purchase_order_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**operation_type_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**intrastat_transport_mode:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**intrastat_delivery_terms:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**intrastat_region:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**intrastat_nature_of_transaction:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**einvoice_number:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**document_ref:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Vec<PostV1PurchasesInvoicesCreateRequestLinesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">post_v1_purchases_invoices_get</a>(request: PostV1PurchasesInvoicesGetRequest) -> Result&lt;PostV1PurchasesInvoicesGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .purchases
        .post_v1purchases_invoices_get(
            &PostV1PurchasesInvoicesGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">post_v1_purchases_invoices_update</a>(request: PostV1PurchasesInvoicesUpdateRequest) -> Result&lt;PostV1PurchasesInvoicesUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .purchases
        .post_v1purchases_invoices_update(
            &PostV1PurchasesInvoicesUpdateRequest {
                id: "id".to_string(),
                partner_id: None,
                document_number: None,
                document_date: None,
                due_date: None,
                currency: None,
                purchase_order_id: None,
                operation_type_id: None,
                notes: None,
                intrastat_transport_mode: None,
                intrastat_delivery_terms: None,
                intrastat_region: None,
                intrastat_nature_of_transaction: None,
                einvoice_number: None,
                lines: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**partner_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**document_number:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**document_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**due_date:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**purchase_order_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**operation_type_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**intrastat_transport_mode:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**intrastat_delivery_terms:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**intrastat_region:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**intrastat_nature_of_transaction:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**einvoice_number:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Option<Vec<PostV1PurchasesInvoicesUpdateRequestLinesItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">post_v1_purchases_invoices_delete</a>(request: PostV1PurchasesInvoicesDeleteRequest) -> Result&lt;PostV1PurchasesInvoicesDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .purchases
        .post_v1purchases_invoices_delete(
            &PostV1PurchasesInvoicesDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">post_v1_purchases_invoices_register</a>(request: PostV1PurchasesInvoicesRegisterRequest) -> Result&lt;PostV1PurchasesInvoicesRegisterResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .purchases
        .post_v1purchases_invoices_register(
            &PostV1PurchasesInvoicesRegisterRequest {
                id: "id".to_string(),
                registration_date: None,
                warehouse_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**registration_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**warehouse_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">post_v1_purchases_invoices_list</a>(request: PostV1PurchasesInvoicesListRequest) -> Result&lt;PostV1PurchasesInvoicesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .purchases
        .post_v1purchases_invoices_list(
            &PostV1PurchasesInvoicesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1PurchasesInvoicesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1PurchasesInvoicesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">post_v1_purchases_orders_create</a>(request: PostV1PurchasesOrdersCreateRequest) -> Result&lt;PostV1PurchasesOrdersCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .purchases
        .post_v1purchases_orders_create(
            &PostV1PurchasesOrdersCreateRequest {
                partner_id: "partnerId".to_string(),
                order_date: "orderDate".to_string(),
                lines: vec![PostV1PurchasesOrdersCreateRequestLinesItem {
                    ..Default::default()
                }],
                order_number: None,
                expected_date: None,
                warehouse_id: None,
                currency: None,
                notes: None,
                document_ref: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**partner_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**order_number:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**order_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**expected_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**warehouse_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**document_ref:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Vec<PostV1PurchasesOrdersCreateRequestLinesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">post_v1_purchases_orders_update</a>(request: PostV1PurchasesOrdersUpdateRequest) -> Result&lt;PostV1PurchasesOrdersUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .purchases
        .post_v1purchases_orders_update(
            &PostV1PurchasesOrdersUpdateRequest {
                id: "id".to_string(),
                partner_id: None,
                order_date: None,
                expected_date: None,
                warehouse_id: None,
                currency: None,
                notes: None,
                lines: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**partner_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**order_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**expected_date:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**warehouse_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Option<Vec<PostV1PurchasesOrdersUpdateRequestLinesItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">post_v1_purchases_orders_get</a>(request: PostV1PurchasesOrdersGetRequest) -> Result&lt;PostV1PurchasesOrdersGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .purchases
        .post_v1purchases_orders_get(
            &PostV1PurchasesOrdersGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">post_v1_purchases_orders_list</a>(request: PostV1PurchasesOrdersListRequest) -> Result&lt;PostV1PurchasesOrdersListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .purchases
        .post_v1purchases_orders_list(
            &PostV1PurchasesOrdersListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1PurchasesOrdersListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1PurchasesOrdersListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">post_v1_purchases_orders_submit</a>(request: PostV1PurchasesOrdersSubmitRequest) -> Result&lt;PostV1PurchasesOrdersSubmitResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .purchases
        .post_v1purchases_orders_submit(
            &PostV1PurchasesOrdersSubmitRequest {
                id: "id".to_string(),
                reason: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**reason:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">post_v1_purchases_orders_approve</a>(request: PostV1PurchasesOrdersApproveRequest) -> Result&lt;PostV1PurchasesOrdersApproveResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .purchases
        .post_v1purchases_orders_approve(
            &PostV1PurchasesOrdersApproveRequest {
                id: "id".to_string(),
                reason: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**reason:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">post_v1_purchases_orders_reject</a>(request: PostV1PurchasesOrdersRejectRequest) -> Result&lt;PostV1PurchasesOrdersRejectResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .purchases
        .post_v1purchases_orders_reject(
            &PostV1PurchasesOrdersRejectRequest {
                id: "id".to_string(),
                reason: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**reason:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">post_v1_purchases_orders_cancel</a>(request: PostV1PurchasesOrdersCancelRequest) -> Result&lt;PostV1PurchasesOrdersCancelResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .purchases
        .post_v1purchases_orders_cancel(
            &PostV1PurchasesOrdersCancelRequest {
                id: "id".to_string(),
                reason: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**reason:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">post_v1_purchases_orders_close</a>(request: PostV1PurchasesOrdersCloseRequest) -> Result&lt;PostV1PurchasesOrdersCloseResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .purchases
        .post_v1purchases_orders_close(
            &PostV1PurchasesOrdersCloseRequest {
                id: "id".to_string(),
                reason: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**reason:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">post_v1_purchases_orders_delete</a>(request: PostV1PurchasesOrdersDeleteRequest) -> Result&lt;PostV1PurchasesOrdersDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .purchases
        .post_v1purchases_orders_delete(
            &PostV1PurchasesOrdersDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">post_v1_purchases_receipts_create</a>(request: PostV1PurchasesReceiptsCreateRequest) -> Result&lt;PostV1PurchasesReceiptsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .purchases
        .post_v1purchases_receipts_create(
            &PostV1PurchasesReceiptsCreateRequest {
                order_id: "orderId".to_string(),
                receipt_date: "receiptDate".to_string(),
                lines: vec![PostV1PurchasesReceiptsCreateRequestLinesItem {
                    order_line_id: "orderLineId".to_string(),
                    quantity: "quantity".to_string(),
                    ..Default::default()
                }],
                warehouse_id: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**order_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**receipt_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**warehouse_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Vec<PostV1PurchasesReceiptsCreateRequestLinesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">post_v1_purchases_receipts_get</a>(request: PostV1PurchasesReceiptsGetRequest) -> Result&lt;PostV1PurchasesReceiptsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .purchases
        .post_v1purchases_receipts_get(
            &PostV1PurchasesReceiptsGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">post_v1_purchases_receipts_list</a>(request: PostV1PurchasesReceiptsListRequest) -> Result&lt;PostV1PurchasesReceiptsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .purchases
        .post_v1purchases_receipts_list(
            &PostV1PurchasesReceiptsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1PurchasesReceiptsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1PurchasesReceiptsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">post_v1_purchases_invoices_match</a>(request: PostV1PurchasesInvoicesMatchRequest) -> Result&lt;PostV1PurchasesInvoicesMatchResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .purchases
        .post_v1purchases_invoices_match(
            &PostV1PurchasesInvoicesMatchRequest {
                invoice_id: "invoiceId".to_string(),
                price_tolerance_percent: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**invoice_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**price_tolerance_percent:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Capture
<details><summary><code>client.capture.<a href="/src/api/resources/capture/client.rs">post_v1_capture_settings_get</a>(request: PostV1CaptureSettingsGetRequest) -> Result&lt;PostV1CaptureSettingsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .capture
        .post_v1capture_settings_get(
            &PostV1CaptureSettingsGetRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.capture.<a href="/src/api/resources/capture/client.rs">post_v1_capture_settings_update</a>(request: PostV1CaptureSettingsUpdateRequest) -> Result&lt;PostV1CaptureSettingsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .capture
        .post_v1capture_settings_update(
            &PostV1CaptureSettingsUpdateRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**intake_enabled:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**capture_auto_extract:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.capture.<a href="/src/api/resources/capture/client.rs">post_v1_capture_settings_regenerate_intake</a>(request: PostV1CaptureSettingsRegenerateIntakeRequest) -> Result&lt;PostV1CaptureSettingsRegenerateIntakeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .capture
        .post_v1capture_settings_regenerate_intake(
            &PostV1CaptureSettingsRegenerateIntakeRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.capture.<a href="/src/api/resources/capture/client.rs">receive_an_inbound_email_with_supplier_documents_attached_postmark_style_or_generic_json</a>(request: PostV1CaptureInboundEmailRequest) -> Result&lt;PostV1CaptureInboundEmailResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .capture
        .receive_an_inbound_email_with_supplier_documents_attached_postmark_style_or_generic_json(
            &PostV1CaptureInboundEmailRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**postmark_to:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**to_full:** `Option<Vec<PostV1CaptureInboundEmailRequestToFullItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**postmark_from:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**postmark_subject:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**postmark_attachments:** `Option<Vec<PostV1CaptureInboundEmailRequestAttachmentsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**to:** `Option<PostV1CaptureInboundEmailRequestTo>` 
    
</dd>
</dl>

<dl>
<dd>

**from:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**subject:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**attachments:** `Option<Vec<PostV1CaptureInboundEmailRequestAttachmentsItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.capture.<a href="/src/api/resources/capture/client.rs">read_a_vendor_bill_or_receipt_and_return_an_editable_purchase_invoice_draft</a>(request: PostV1CaptureDocumentsUploadRequest) -> Result&lt;PostV1CaptureDocumentsUploadResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .capture
        .read_a_vendor_bill_or_receipt_and_return_an_editable_purchase_invoice_draft(
            &PostV1CaptureDocumentsUploadRequest {
                file_name: "fileName".to_string(),
                mime_type: "mimeType".to_string(),
                content: "content".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**file_name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**mime_type:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**content:** `String` — Base64-encoded scan, photo or PDF of the supplier document
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.capture.<a href="/src/api/resources/capture/client.rs">re_read_a_stored_capture_replacing_the_previous_draft</a>(request: PostV1CaptureDocumentsExtractRequest) -> Result&lt;PostV1CaptureDocumentsExtractResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .capture
        .re_read_a_stored_capture_replacing_the_previous_draft(
            &PostV1CaptureDocumentsExtractRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.capture.<a href="/src/api/resources/capture/client.rs">post_v1_capture_documents_get</a>(request: PostV1CaptureDocumentsGetRequest) -> Result&lt;PostV1CaptureDocumentsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .capture
        .post_v1capture_documents_get(
            &PostV1CaptureDocumentsGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.capture.<a href="/src/api/resources/capture/client.rs">post_v1_capture_documents_list</a>(request: PostV1CaptureDocumentsListRequest) -> Result&lt;PostV1CaptureDocumentsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .capture
        .post_v1capture_documents_list(
            &PostV1CaptureDocumentsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1CaptureDocumentsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1CaptureDocumentsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.capture.<a href="/src/api/resources/capture/client.rs">post_v1_capture_documents_delete</a>(request: PostV1CaptureDocumentsDeleteRequest) -> Result&lt;PostV1CaptureDocumentsDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .capture
        .post_v1capture_documents_delete(
            &PostV1CaptureDocumentsDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.capture.<a href="/src/api/resources/capture/client.rs">save_the_reviewed_draft_as_a_purchase_invoice_and_attach_the_original_document</a>(request: PostV1CaptureDocumentsConfirmRequest) -> Result&lt;PostV1CaptureDocumentsConfirmResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .capture
        .save_the_reviewed_draft_as_a_purchase_invoice_and_attach_the_original_document(
            &PostV1CaptureDocumentsConfirmRequest {
                id: "id".to_string(),
                document_number: "documentNumber".to_string(),
                document_date: "documentDate".to_string(),
                lines: vec![PostV1CaptureDocumentsConfirmRequestLinesItem {
                    ..Default::default()
                }],
                partner_id: None,
                new_supplier: None,
                due_date: None,
                currency: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**partner_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**new_supplier:** `Option<PostV1CaptureDocumentsConfirmRequestNewSupplier>` 
    
</dd>
</dl>

<dl>
<dd>

**document_number:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**document_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**due_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Vec<PostV1CaptureDocumentsConfirmRequestLinesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Declarations
<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_lt_intrastat_compute</a>(request: PostV1DeclarationsLtIntrastatComputeRequest) -> Result&lt;PostV1DeclarationsLtIntrastatComputeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_lt_intrastat_compute(
            &PostV1DeclarationsLtIntrastatComputeRequest {
                year: 1000000,
                month: 1000000,
                flow: PostV1DeclarationsLtIntrastatComputeRequestFlow::Arrivals,
                transaction_nature: None,
                delivery_terms: None,
                transport_mode: None,
                region_code: None,
                statistical_value_required: None,
                preparation_time_hours: None,
                preparation_time_minutes: None,
                persist: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**flow:** `PostV1DeclarationsLtIntrastatComputeRequestFlow` 
    
</dd>
</dl>

<dl>
<dd>

**transaction_nature:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**delivery_terms:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**transport_mode:** `Option<PostV1DeclarationsLtIntrastatComputeRequestTransportMode>` 
    
</dd>
</dl>

<dl>
<dd>

**region_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**statistical_value_required:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**preparation_time_hours:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**preparation_time_minutes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**persist:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_lt_ivaz_generate</a>(request: PostV1DeclarationsLtIvazGenerateRequest) -> Result&lt;PostV1DeclarationsLtIvazGenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_lt_ivaz_generate(
            &PostV1DeclarationsLtIvazGenerateRequest {
                waybill_ids: vec!["waybillIds".to_string()],
                persist: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**waybill_ids:** `Vec<String>` 
    
</dd>
</dl>

<dl>
<dd>

**persist:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_lt_intrastat_obligation</a>(request: PostV1DeclarationsLtIntrastatObligationRequest) -> Result&lt;PostV1DeclarationsLtIntrastatObligationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_lt_intrastat_obligation(
            &PostV1DeclarationsLtIntrastatObligationRequest { year: 1000000 },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_lt_isaf_generate</a>(request: PostV1DeclarationsLtIsafGenerateRequest) -> Result&lt;PostV1DeclarationsLtIsafGenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_lt_isaf_generate(
            &PostV1DeclarationsLtIsafGenerateRequest {
                year: 1000000,
                month: 1000000,
                data_type: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**data_type:** `Option<PostV1DeclarationsLtIsafGenerateRequestDataType>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_lt_fr0600_compute</a>(request: PostV1DeclarationsLtFr0600ComputeRequest) -> Result&lt;PostV1DeclarationsLtFr0600ComputeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_lt_fr0600compute(
            &PostV1DeclarationsLtFr0600ComputeRequest {
                year: 1000000,
                month: 1000000,
                months: None,
                deduction_percent: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**months:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**deduction_percent:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_lt_gpm313_compute</a>(request: PostV1DeclarationsLtGpm313ComputeRequest) -> Result&lt;PostV1DeclarationsLtGpm313ComputeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_lt_gpm313compute(
            &PostV1DeclarationsLtGpm313ComputeRequest {
                year: 1000000,
                month: 1000000,
                payout_timing: None,
                payment_day: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**payout_timing:** `Option<PostV1DeclarationsLtGpm313ComputeRequestPayoutTiming>` 
    
</dd>
</dl>

<dl>
<dd>

**payment_day:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_lt_sam_compute</a>(request: PostV1DeclarationsLtSamComputeRequest) -> Result&lt;PostV1DeclarationsLtSamComputeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_lt_sam_compute(
            &PostV1DeclarationsLtSamComputeRequest {
                year: 1000000,
                month: 1000000,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_lt_sd_generate</a>(request: PostV1DeclarationsLtSdGenerateRequest) -> Result&lt;PostV1DeclarationsLtSdGenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_lt_sd_generate(
            &PostV1DeclarationsLtSdGenerateRequest {
                r#type: PostV1DeclarationsLtSdGenerateRequestType::OneSd,
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**type_:** `PostV1DeclarationsLtSdGenerateRequestType` 
    
</dd>
</dl>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_lt_saft_generate</a>(request: PostV1DeclarationsLtSaftGenerateRequest) -> Result&lt;PostV1DeclarationsLtSaftGenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_lt_saft_generate(
            &PostV1DeclarationsLtSaftGenerateRequest {
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
                data_type: None,
                persist: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**data_type:** `Option<PostV1DeclarationsLtSaftGenerateRequestDataType>` 
    
</dd>
</dl>

<dl>
<dd>

**persist:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_lt_ivaz_amend</a>(request: PostV1DeclarationsLtIvazAmendRequest) -> Result&lt;PostV1DeclarationsLtIvazAmendResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_lt_ivaz_amend(
            &PostV1DeclarationsLtIvazAmendRequest {
                waybill_ids: vec!["waybillIds".to_string()],
                persist: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**waybill_ids:** `Vec<String>` 
    
</dd>
</dl>

<dl>
<dd>

**persist:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_lt_ivaz_cancel</a>(request: PostV1DeclarationsLtIvazCancelRequest) -> Result&lt;PostV1DeclarationsLtIvazCancelResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_lt_ivaz_cancel(
            &PostV1DeclarationsLtIvazCancelRequest {
                entries: vec![PostV1DeclarationsLtIvazCancelRequestEntriesItem {
                    waybill_id: "waybillId".to_string(),
                    reason: PostV1DeclarationsLtIvazCancelRequestEntriesItemReason::One,
                    additional_info: None,
                }],
                persist: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entries:** `Vec<PostV1DeclarationsLtIvazCancelRequestEntriesItem>` 
    
</dd>
</dl>

<dl>
<dd>

**persist:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_lt_fr0564_compute</a>(request: PostV1DeclarationsLtFr0564ComputeRequest) -> Result&lt;PostV1DeclarationsLtFr0564ComputeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_lt_fr0564compute(
            &PostV1DeclarationsLtFr0564ComputeRequest {
                year: 1000000,
                month: 1000000,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_lt_gpm312_compute</a>(request: PostV1DeclarationsLtGpm312ComputeRequest) -> Result&lt;PostV1DeclarationsLtGpm312ComputeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_lt_gpm312compute(
            &PostV1DeclarationsLtGpm312ComputeRequest {
                year: 1000000,
                payout_timing: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**payout_timing:** `Option<PostV1DeclarationsLtGpm312ComputeRequestPayoutTiming>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_lt_pln204_compute</a>(request: PostV1DeclarationsLtPln204ComputeRequest) -> Result&lt;PostV1DeclarationsLtPln204ComputeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_lt_pln204compute(
            &PostV1DeclarationsLtPln204ComputeRequest { year: 1000000 },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_eu_oss_compute</a>(request: PostV1DeclarationsEuOssComputeRequest) -> Result&lt;PostV1DeclarationsEuOssComputeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_eu_oss_compute(
            &PostV1DeclarationsEuOssComputeRequest {
                year: 1000000,
                quarter: 1000000,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**quarter:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_eu_ioss_compute</a>(request: PostV1DeclarationsEuIossComputeRequest) -> Result&lt;PostV1DeclarationsEuIossComputeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_eu_ioss_compute(
            &PostV1DeclarationsEuIossComputeRequest {
                year: 1000000,
                month: 1000000,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_eu_distance_sales_threshold_get</a>(request: PostV1DeclarationsEuDistanceSalesThresholdGetRequest) -> Result&lt;PostV1DeclarationsEuDistanceSalesThresholdGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_eu_distance_sales_threshold_get(
            &PostV1DeclarationsEuDistanceSalesThresholdGetRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**date:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_eu_union_turnover_get</a>(request: PostV1DeclarationsEuUnionTurnoverGetRequest) -> Result&lt;PostV1DeclarationsEuUnionTurnoverGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_eu_union_turnover_get(
            &PostV1DeclarationsEuUnionTurnoverGetRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**date:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_eu_sme_cross_border_report_compute</a>(request: PostV1DeclarationsEuSmeCrossBorderReportComputeRequest) -> Result&lt;PostV1DeclarationsEuSmeCrossBorderReportComputeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_eu_sme_cross_border_report_compute(
            &PostV1DeclarationsEuSmeCrossBorderReportComputeRequest {
                year: 1000000,
                quarter: 1000000,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**quarter:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_eu_sme_thresholds_list</a>(request: PostV1DeclarationsEuSmeThresholdsListRequest) -> Result&lt;PostV1DeclarationsEuSmeThresholdsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_eu_sme_thresholds_list(
            &PostV1DeclarationsEuSmeThresholdsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_eu_sme_threshold_get</a>(request: PostV1DeclarationsEuSmeThresholdGetRequest) -> Result&lt;PostV1DeclarationsEuSmeThresholdGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_eu_sme_threshold_get(
            &PostV1DeclarationsEuSmeThresholdGetRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**date:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_eu_vat_return_packs_list</a>(request: PostV1DeclarationsEuVatReturnPacksListRequest) -> Result&lt;PostV1DeclarationsEuVatReturnPacksListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_eu_vat_return_packs_list(
            &PostV1DeclarationsEuVatReturnPacksListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_eu_vat_return_compute</a>(request: PostV1DeclarationsEuVatReturnComputeRequest) -> Result&lt;PostV1DeclarationsEuVatReturnComputeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_eu_vat_return_compute(
            &PostV1DeclarationsEuVatReturnComputeRequest {
                country_code: "countryCode".to_string(),
                year: 1000000,
                month: 1000000,
                months: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**country_code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**months:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_pl_jpk_v7_m_generate</a>(request: PostV1DeclarationsPlJpkV7MGenerateRequest) -> Result&lt;PostV1DeclarationsPlJpkV7MGenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Generate the Polish JPK_V7M(3) file (VAT declaration with evidence) for a month, per the MF schema in force since February 2026. Amounts must already be in PLN; rows are marked BFK until a KSeF integration supplies invoice numbers. Review the warnings before submitting via e-dokumenty.mf.gov.pl.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_pl_jpk_v7m_generate(
            &PostV1DeclarationsPlJpkV7MGenerateRequest {
                year: 1000000,
                month: 1000000,
                kod_urzedu: "kodUrzedu".to_string(),
                email: "email".to_string(),
                cel_zlozenia: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**kod_urzedu:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**email:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**cel_zlozenia:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_pl_vat_ue_generate</a>(request: PostV1DeclarationsPlVatUeGenerateRequest) -> Result&lt;PostV1DeclarationsPlVatUeGenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Build the rows of the Polish recapitulative statement VAT-UE for a month: section C intra-Community supplies of goods, section D intra-Community acquisitions, section E services taxed where the customer is established. Amounts are full złoty per counterparty. The VAT-UE(5) file itself goes out from the EU sales list deadline in the calendar.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_pl_vat_ue_generate(
            &PostV1DeclarationsPlVatUeGenerateRequest {
                year: 1000000,
                month: 1000000,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_pl_intrastat_generate</a>(request: PostV1DeclarationsPlIntrastatGenerateRequest) -> Result&lt;PostV1DeclarationsPlIntrastatGenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Build the rows of the Polish INTRASTAT declaration for a month, arrivals or dispatches, grouped by CN code, partner country, country of origin, partner VAT number, nature of transaction, transport and delivery terms. Values are whole złoty converted at the invoice rate; credit notes with goods lines are returns (code 21). Goods without a CN code are left out and named in the warnings. The IST message itself goes out from the Intrastat deadline in the calendar.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_pl_intrastat_generate(
            &PostV1DeclarationsPlIntrastatGenerateRequest {
                year: 1000000,
                month: 1000000,
                flow: PostV1DeclarationsPlIntrastatGenerateRequestFlow::Arrivals,
                transaction_nature: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**flow:** `PostV1DeclarationsPlIntrastatGenerateRequestFlow` 
    
</dd>
</dl>

<dl>
<dd>

**transaction_nature:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_pl_ksef_received_list</a>(request: PostV1DeclarationsPlKsefReceivedListRequest) -> Result&lt;PostV1DeclarationsPlKsefReceivedListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

List the invoices KSeF holds for this company as the buyer, for a window of acquisition timestamps. Each row carries the KSeF number and, when the document number matches a registered purchase invoice, the invoice it belongs to.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_pl_ksef_received_list(
            &PostV1DeclarationsPlKsefReceivedListRequest {
                from: DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z").unwrap(),
                to: DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z").unwrap(),
                page_size: None,
                page_offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_offset:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_pl_ksef_received_fetch</a>(request: PostV1DeclarationsPlKsefReceivedFetchRequest) -> Result&lt;PostV1DeclarationsPlKsefReceivedFetchResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Read one invoice out of KSeF by its national number. With a purchase invoice given, the KSeF number is written onto that invoice, which is what makes the purchase row of JPK_V7M carry NrKSeF instead of the BFK marker.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_pl_ksef_received_fetch(
            &PostV1DeclarationsPlKsefReceivedFetchRequest {
                ksef_number: "ksefNumber".to_string(),
                purchase_invoice_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**ksef_number:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**purchase_invoice_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_pl_ksef_receipt</a>(request: PostV1DeclarationsPlKsefReceiptRequest) -> Result&lt;PostV1DeclarationsPlKsefReceiptResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

The UPO for a KSeF session. KSeF issues one receipt per session rather than per invoice, so the session reference number from the send is what identifies it.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_pl_ksef_receipt(
            &PostV1DeclarationsPlKsefReceiptRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**session_reference_number:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">tax_adjustments_recorded_for_a_tax_year</a>(request: PostV1DeclarationsTaxAdjustmentsListRequest) -> Result&lt;PostV1DeclarationsTaxAdjustmentsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

The differences between the accounting result and the taxable profit: non-deductible expenses, income added to or left out of the tax base, extra deductible expenses, donations, losses carried forward, reliefs and tax credits. The annual corporate income tax return is built from them.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .tax_adjustments_recorded_for_a_tax_year(
            &PostV1DeclarationsTaxAdjustmentsListRequest { year: 1000000 },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">record_a_tax_adjustment_for_a_tax_year</a>(request: PostV1DeclarationsTaxAdjustmentsCreateRequest) -> Result&lt;PostV1DeclarationsTaxAdjustmentsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .record_a_tax_adjustment_for_a_tax_year(
            &PostV1DeclarationsTaxAdjustmentsCreateRequest {
                year: 1000000,
                kind: PostV1DeclarationsTaxAdjustmentsCreateRequestKind::NonDeductible,
                amount: "amount".to_string(),
                description: "description".to_string(),
                code: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**kind:** `PostV1DeclarationsTaxAdjustmentsCreateRequestKind` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**amount:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">change_a_recorded_tax_adjustment</a>(request: PostV1DeclarationsTaxAdjustmentsUpdateRequest) -> Result&lt;PostV1DeclarationsTaxAdjustmentsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .change_a_recorded_tax_adjustment(
            &PostV1DeclarationsTaxAdjustmentsUpdateRequest {
                id: "id".to_string(),
                kind: None,
                code: None,
                amount: None,
                description: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**kind:** `Option<PostV1DeclarationsTaxAdjustmentsUpdateRequestKind>` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**amount:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">remove_a_recorded_tax_adjustment</a>(request: PostV1DeclarationsTaxAdjustmentsDeleteRequest) -> Result&lt;PostV1DeclarationsTaxAdjustmentsDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .remove_a_recorded_tax_adjustment(
            &PostV1DeclarationsTaxAdjustmentsDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">payments_already_made_towards_a_tax_of_a_year</a>(request: PostV1DeclarationsTaxPaymentsListRequest) -> Result&lt;PostV1DeclarationsTaxPaymentsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

What the company has paid the administration towards a tax before the return is filed: payments on account, tax withheld at source by others, a final settlement, and a refund received. Returns report these on their own lines, so the amount they ask for is the balance.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .payments_already_made_towards_a_tax_of_a_year(
            &PostV1DeclarationsTaxPaymentsListRequest {
                tax: PostV1DeclarationsTaxPaymentsListRequestTax::CorporateIncomeTax,
                year: 1000000,
                month: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**tax:** `PostV1DeclarationsTaxPaymentsListRequestTax` 
    
</dd>
</dl>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">record_a_payment_made_towards_a_tax</a>(request: PostV1DeclarationsTaxPaymentsCreateRequest) -> Result&lt;PostV1DeclarationsTaxPaymentsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .record_a_payment_made_towards_a_tax(
            &PostV1DeclarationsTaxPaymentsCreateRequest {
                tax: PostV1DeclarationsTaxPaymentsCreateRequestTax::CorporateIncomeTax,
                year: 1000000,
                kind: PostV1DeclarationsTaxPaymentsCreateRequestKind::Advance,
                amount: "amount".to_string(),
                paid_on: "paidOn".to_string(),
                description: "description".to_string(),
                month: None,
                reference: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**tax:** `PostV1DeclarationsTaxPaymentsCreateRequestTax` 
    
</dd>
</dl>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**kind:** `PostV1DeclarationsTaxPaymentsCreateRequestKind` 
    
</dd>
</dl>

<dl>
<dd>

**amount:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**paid_on:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**reference:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">change_a_recorded_tax_payment</a>(request: PostV1DeclarationsTaxPaymentsUpdateRequest) -> Result&lt;PostV1DeclarationsTaxPaymentsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .change_a_recorded_tax_payment(
            &PostV1DeclarationsTaxPaymentsUpdateRequest {
                id: "id".to_string(),
                kind: None,
                amount: None,
                paid_on: None,
                reference: None,
                description: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**kind:** `Option<PostV1DeclarationsTaxPaymentsUpdateRequestKind>` 
    
</dd>
</dl>

<dl>
<dd>

**amount:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**paid_on:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**reference:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">remove_a_recorded_tax_payment</a>(request: PostV1DeclarationsTaxPaymentsDeleteRequest) -> Result&lt;PostV1DeclarationsTaxPaymentsDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .remove_a_recorded_tax_payment(
            &PostV1DeclarationsTaxPaymentsDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">adoption_and_signing_facts_of_the_annual_accounts_of_a_year</a>(request: PostV1DeclarationsAnnualAccountsGetRequest) -> Result&lt;PostV1DeclarationsAnnualAccountsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Whether the general meeting adopted the annual accounts and on which date, the date the accounts were prepared, and which directors signed them. The annual accounts filed with the trade register are built from these facts.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .adoption_and_signing_facts_of_the_annual_accounts_of_a_year(
            &PostV1DeclarationsAnnualAccountsGetRequest { year: 1000000 },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">record_the_adoption_and_preparation_of_the_annual_accounts_of_a_year</a>(request: PostV1DeclarationsAnnualAccountsSetRequest) -> Result&lt;PostV1DeclarationsAnnualAccountsSetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .record_the_adoption_and_preparation_of_the_annual_accounts_of_a_year(
            &PostV1DeclarationsAnnualAccountsSetRequest {
                year: 1000000,
                adopted: true,
                date_of_preparation: "dateOfPreparation".to_string(),
                adoption_date: None,
                audited: None,
                audit_report_qualified: None,
                auditor_not_elected: None,
                notes_text: None,
                management_report_text: None,
                auditor_report_text: None,
                auditor_report_date: None,
                result_to_reserves: None,
                result_to_loss_compensation: None,
                result_to_remainder: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**adopted:** `bool` 
    
</dd>
</dl>

<dl>
<dd>

**adoption_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**date_of_preparation:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**audited:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**audit_report_qualified:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**auditor_not_elected:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**notes_text:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**management_report_text:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**auditor_report_text:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**auditor_report_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**result_to_reserves:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**result_to_loss_compensation:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**result_to_remainder:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">record_whether_a_director_signed_the_annual_accounts_of_a_year</a>(request: PostV1DeclarationsAnnualAccountsSignaturesCreateRequest) -> Result&lt;PostV1DeclarationsAnnualAccountsSignaturesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.declarations.record_whether_a_director_signed_the_annual_accounts_of_a_year(&PostV1DeclarationsAnnualAccountsSignaturesCreateRequest {
        year: 1000000,
        director_name: "directorName".to_string(),
        director_type: PostV1DeclarationsAnnualAccountsSignaturesCreateRequestDirectorType::ManagingCurrent,
        signed: true,
        signed_on: None,
        signed_at: None,
        reason_not_signed: None
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**director_name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**director_type:** `PostV1DeclarationsAnnualAccountsSignaturesCreateRequestDirectorType` 
    
</dd>
</dl>

<dl>
<dd>

**signed:** `bool` 
    
</dd>
</dl>

<dl>
<dd>

**signed_on:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**signed_at:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**reason_not_signed:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">change_a_recorded_director_signature</a>(request: PostV1DeclarationsAnnualAccountsSignaturesUpdateRequest) -> Result&lt;PostV1DeclarationsAnnualAccountsSignaturesUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.declarations.change_a_recorded_director_signature(&PostV1DeclarationsAnnualAccountsSignaturesUpdateRequest {
        id: "id".to_string(),
        director_name: "directorName".to_string(),
        director_type: PostV1DeclarationsAnnualAccountsSignaturesUpdateRequestDirectorType::ManagingCurrent,
        signed: true,
        signed_on: None,
        signed_at: None,
        reason_not_signed: None
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**director_name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**director_type:** `PostV1DeclarationsAnnualAccountsSignaturesUpdateRequestDirectorType` 
    
</dd>
</dl>

<dl>
<dd>

**signed:** `bool` 
    
</dd>
</dl>

<dl>
<dd>

**signed_on:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**signed_at:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**reason_not_signed:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">remove_a_recorded_director_signature</a>(request: PostV1DeclarationsAnnualAccountsSignaturesDeleteRequest) -> Result&lt;PostV1DeclarationsAnnualAccountsSignaturesDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .remove_a_recorded_director_signature(
            &PostV1DeclarationsAnnualAccountsSignaturesDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">record_a_decision_to_distribute_profit_a_dividend_an_interim_dividend_or_a_payment_treated_as_one</a>(request: PostV1DeclarationsAnnualAccountsDistributionsCreateRequest) -> Result&lt;PostV1DeclarationsAnnualAccountsDistributionsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.declarations.record_a_decision_to_distribute_profit_a_dividend_an_interim_dividend_or_a_payment_treated_as_one(&PostV1DeclarationsAnnualAccountsDistributionsCreateRequest {
        year: 1000000,
        decided_on: "decidedOn".to_string(),
        kind: PostV1DeclarationsAnnualAccountsDistributionsCreateRequestKind::Dividend,
        amount: "amount".to_string(),
        description: None
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**decided_on:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**kind:** `PostV1DeclarationsAnnualAccountsDistributionsCreateRequestKind` 
    
</dd>
</dl>

<dl>
<dd>

**amount:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">change_a_recorded_profit_distribution</a>(request: PostV1DeclarationsAnnualAccountsDistributionsUpdateRequest) -> Result&lt;PostV1DeclarationsAnnualAccountsDistributionsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .change_a_recorded_profit_distribution(
            &PostV1DeclarationsAnnualAccountsDistributionsUpdateRequest {
                id: "id".to_string(),
                decided_on: "decidedOn".to_string(),
                kind: PostV1DeclarationsAnnualAccountsDistributionsUpdateRequestKind::Dividend,
                amount: "amount".to_string(),
                description: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**decided_on:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**kind:** `PostV1DeclarationsAnnualAccountsDistributionsUpdateRequestKind` 
    
</dd>
</dl>

<dl>
<dd>

**amount:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">remove_a_recorded_profit_distribution</a>(request: PostV1DeclarationsAnnualAccountsDistributionsDeleteRequest) -> Result&lt;PostV1DeclarationsAnnualAccountsDistributionsDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .remove_a_recorded_profit_distribution(
            &PostV1DeclarationsAnnualAccountsDistributionsDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">attach_an_uploaded_document_to_the_annual_accounts_of_a_year</a>(request: PostV1DeclarationsAnnualAccountsAttachmentsAddRequest) -> Result&lt;PostV1DeclarationsAnnualAccountsAttachmentsAddResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Links a file uploaded through files/upload (its storageKey) to the annual accounts of the year as the notes, the management report, the auditor statement, the profit appropriation resolution, the approval certificate, the general data sheet, the full report as a pdf, or another document. Deposits that must carry these documents take them from here.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .attach_an_uploaded_document_to_the_annual_accounts_of_a_year(
            &PostV1DeclarationsAnnualAccountsAttachmentsAddRequest {
                year: 1000000,
                kind: PostV1DeclarationsAnnualAccountsAttachmentsAddRequestKind::FullReport,
                r#ref: "ref".to_string(),
                name: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**kind:** `PostV1DeclarationsAnnualAccountsAttachmentsAddRequestKind` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**ref_:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">remove_a_document_attached_to_the_annual_accounts_and_delete_its_file</a>(request: PostV1DeclarationsAnnualAccountsAttachmentsDeleteRequest) -> Result&lt;PostV1DeclarationsAnnualAccountsAttachmentsDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .remove_a_document_attached_to_the_annual_accounts_and_delete_its_file(
            &PostV1DeclarationsAnnualAccountsAttachmentsDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_cy_td4_generate</a>(request: PostV1DeclarationsCyTd4GenerateRequest) -> Result&lt;PostV1DeclarationsCyTd4GenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Compute the company income tax return TD4 of a tax year from the ledger and the recorded tax adjustments: the accounting profit, the add-backs, deductions, capital allowances and losses brought forward, the chargeable income, the corporation tax at the rate of the year and the double tax relief, as the fields the company keys into TAXISnet or Tax For All. The Tax Department publishes no upload layout for the TD4; the XML is a working file.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_cy_td4generate(
            &PostV1DeclarationsCyTd4GenerateRequest { year: 1000000 },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_cy_he32_generate</a>(request: PostV1DeclarationsCyHe32GenerateRequest) -> Result&lt;PostV1DeclarationsCyHe32GenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Build the annual return HE32 of a year: the figures the Registrar’s e-filing screens ask for (company number, registered office, made-up-to date, share capital, register of members, directors and secretary, annual general meeting date, the accounts summary), the working file, and the printed form HE32(I) filled in as a PDF for signing and for keying into the Registrar’s system, which takes the return only through its own screens.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_cy_he32generate(
            &PostV1DeclarationsCyHe32GenerateRequest { year: 1000000 },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_de_returns_generate</a>(request: PostV1DeclarationsDeReturnsGenerateRequest) -> Result&lt;PostV1DeclarationsDeReturnsGenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Build one of the German returns that ELSTER accepts only through a licensed ERiC transmission (E-Bilanz, Körperschaftsteuer, Gewerbesteuer with its Zerlegungserklärung, annual VAT return, Lohnsteuer-Anmeldung, Lohnsteuerbescheinigung) for the company to send through its own ELSTER-capable program. The period is the year, or YYYY-MM for the monthly Lohnsteuer-Anmeldung.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_de_returns_generate(
            &PostV1DeclarationsDeReturnsGenerateRequest {
                rule_key: PostV1DeclarationsDeReturnsGenerateRequestRuleKey::DeEBilanz,
                period: "period".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**rule_key:** `PostV1DeclarationsDeReturnsGenerateRequestRuleKey` 
    
</dd>
</dl>

<dl>
<dd>

**period:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_de_return_facts_get</a>(request: PostV1DeclarationsDeReturnFactsGetRequest) -> Result&lt;PostV1DeclarationsDeReturnFactsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

The facts of one year that the German annual returns (Körperschaftsteuer, Gewerbesteuer, Umsatzsteuererklärung) need and the ledger does not hold: changes of shareholders, contracts with shareholders, the tax contribution account, loss carry-back, the donation carry-forward, the business premises with the municipalities for the apportionment of the trade tax, the land values or property tax and the participations for the trade tax additions and reductions, the foreign income per country for the Anlage AESt, the date of leaving the small-business scheme and the Anlage UN answers of a company seated abroad. A key that is absent has not been answered.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_de_return_facts_get(
            &PostV1DeclarationsDeReturnFactsGetRequest { year: 1000000 },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_de_return_facts_set</a>(request: PostV1DeclarationsDeReturnFactsSetRequest) -> Result&lt;PostV1DeclarationsDeReturnFactsSetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Replace the facts of one year for the German annual returns. The returns built afterwards read them; a key left out stays unanswered.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_de_return_facts_set(
            &PostV1DeclarationsDeReturnFactsSetRequest {
                year: 1000000,
                facts: PostV1DeclarationsDeReturnFactsSetRequestFacts {
                    ..Default::default()
                },
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**facts:** `PostV1DeclarationsDeReturnFactsSetRequestFacts` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_de_deuev_generate</a>(request: PostV1DeclarationsDeDeuevGenerateRequest) -> Result&lt;PostV1DeclarationsDeDeuevGenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Build the DEÜV notifications of a month (Anmeldung for every start, Abmeldung for every leaving, in December the Jahresmeldung for everyone employed on 31 December) as DSME records with the DBME, DBNA, DBGB and DBAN blocks of Anlage 4 in force from 2026, from the approved payroll runs and the employee record, for the company's own transmission channel.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_de_deuev_generate(
            &PostV1DeclarationsDeDeuevGenerateRequest {
                year: 1000000,
                month: 1000000,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_de_beitragsnachweis_generate</a>(request: PostV1DeclarationsDeBeitragsnachweisGenerateRequest) -> Result&lt;PostV1DeclarationsDeBeitragsnachweisGenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Build the monthly contribution statement to the health insurers (Beitragsnachweis) from the payroll run: one fixed-length record BW02 per insurer, in the record layout in force from 2026, ready for the company's own transmission channel.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_de_beitragsnachweis_generate(
            &PostV1DeclarationsDeBeitragsnachweisGenerateRequest {
                year: 1000000,
                month: 1000000,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_dk_selskabsskat_generate</a>(request: PostV1DeclarationsDkSelskabsskatGenerateRequest) -> Result&lt;PostV1DeclarationsDkSelskabsskatGenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Compute the oplysningsskema for selskaber (selskabsselvangivelsen) of an income year from the ledger and the recorded tax adjustments: accounting result before tax, tax adjustments, losses carried forward, taxable income, the 22 % corporation tax, reliefs and the balance, as the rubrikker the company keys into TastSelv Selskabsskat (DIAS). Skatteforvaltningen publishes no file format for the return; the XML is a working file.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_dk_selskabsskat_generate(
            &PostV1DeclarationsDkSelskabsskatGenerateRequest { year: 1000000 },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_ee_employment_register_send</a>(request: PostV1DeclarationsEeEmploymentRegisterSendRequest) -> Result&lt;PostV1DeclarationsEeEmploymentRegisterSendResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Send one employment register (töötamise register) entry for an employment contract to e-MTA over X-tee: the start of work, or its end with the reason recorded on the contract.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_ee_employment_register_send(
            &PostV1DeclarationsEeEmploymentRegisterSendRequest {
                contract_id: "contractId".to_string(),
                event: PostV1DeclarationsEeEmploymentRegisterSendRequestEvent::Start,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**contract_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**event:** `PostV1DeclarationsEeEmploymentRegisterSendRequestEvent` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_es_verifactu_declaracion_responsable</a>(request: PostV1DeclarationsEsVerifactuDeclaracionResponsableRequest) -> Result&lt;PostV1DeclarationsEsVerifactuDeclaracionResponsableResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Nordlet's declaración responsable for its VERI*FACTU invoicing system (Orden HAC/1177/2024, art. 15), as a PDF and as plain text.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_es_verifactu_declaracion_responsable(
            &PostV1DeclarationsEsVerifactuDeclaracionResponsableRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_ie_ct1_generate</a>(request: PostV1DeclarationsIeCt1GenerateRequest) -> Result&lt;PostV1DeclarationsIeCt1GenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Build the Form CT1 of an accounting year as the ROS version 26 XML and the accompanying financial statements as inline XBRL on the FRS 102 Irish Extension 2026 taxonomy Revenue accepts, both from the ledger, the recorded tax adjustments, the annual accounts record and the officers, for upload through the company’s own ROS account. Says whether the company is above the iXBRL deferral limits (balance sheet total €4.4 million, turnover €8.8 million, 50 employees).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_ie_ct1generate(
            &PostV1DeclarationsIeCt1GenerateRequest { year: 1000000 },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_ie_b1_generate</a>(request: PostV1DeclarationsIeB1GenerateRequest) -> Result&lt;PostV1DeclarationsIeB1GenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Build the working paper for the Form B1 annual return of a financial year — company details, registered office, directors and secretary from Settings → Officers, the members from Settings → Shareholders, the issued share capital and the figures of the financial statements — in the order the CORE screens ask for them. The CRO publishes no file format for the B1, so it is keyed into CORE.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_ie_b1generate(
            &PostV1DeclarationsIeB1GenerateRequest { year: 1000000 },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_it_sdi_purchase_send</a>(request: PostV1DeclarationsItSdiPurchaseSendRequest) -> Result&lt;PostV1DeclarationsItSdiPurchaseSendResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Build the TD16-TD19 integration document for a registered purchase invoice and send it to the Sistema di Interscambio. Since July 2022 a purchase from a supplier established abroad is reported this way instead of the esterometro. The Italian VAT rate to self-assess is a judgement about the supply: pass vatRatePercent unless the purchase lines already carry it, otherwise the request is refused rather than guessed.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_it_sdi_purchase_send(
            &PostV1DeclarationsItSdiPurchaseSendRequest {
                purchase_invoice_id: "purchaseInvoiceId".to_string(),
                vat_rate_percent: None,
                tipo_documento: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**purchase_invoice_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**vat_rate_percent:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**tipo_documento:** `Option<PostV1DeclarationsItSdiPurchaseSendRequestTipoDocumento>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_it_sdi_purchase_preview</a>(request: PostV1DeclarationsItSdiPurchasePreviewRequest) -> Result&lt;PostV1DeclarationsItSdiPurchasePreviewResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Render the TD16-TD19 integration document for a registered purchase invoice without sending it, so the rate and the document type can be checked first.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_it_sdi_purchase_preview(
            &PostV1DeclarationsItSdiPurchasePreviewRequest {
                purchase_invoice_id: "purchaseInvoiceId".to_string(),
                vat_rate_percent: None,
                tipo_documento: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**purchase_invoice_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**vat_rate_percent:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**tipo_documento:** `Option<PostV1DeclarationsItSdiPurchasePreviewRequestTipoDocumento>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_lt_saft_send</a>(request: PostV1DeclarationsLtSaftSendRequest) -> Result&lt;PostV1DeclarationsLtSaftSendResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Upload the SAF-T file to i.SAF-T over the iSAFTUploaderService web service and start its processing. The submission itself is confirmed separately, because after confirmation the file can no longer be corrected.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_lt_saft_send(
            &PostV1DeclarationsLtSaftSendRequest {
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
                data_type: None,
                confirm: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**data_type:** `Option<PostV1DeclarationsLtSaftSendRequestDataType>` 
    
</dd>
</dl>

<dl>
<dd>

**confirm:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_lt_sd_ffdata</a>(request: PostV1DeclarationsLtSdFfdataRequest) -> Result&lt;PostV1DeclarationsLtSdFfdataResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Render the Sodra 1-SD or 2-SD notice for the contracts starting or ending in the range as an .ffdata document for EDAS.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_lt_sd_ffdata(
            &PostV1DeclarationsLtSdFfdataRequest {
                r#type: PostV1DeclarationsLtSdFfdataRequestType::OneSd,
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
                manager_full_name: None,
                preparator_details: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**type_:** `PostV1DeclarationsLtSdFfdataRequestType` 
    
</dd>
</dl>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**manager_full_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**preparator_details:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_lt_pln204_ffdata</a>(request: PostV1DeclarationsLtPln204FfdataRequest) -> Result&lt;PostV1DeclarationsLtPln204FfdataResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Render the annual corporate income tax return PLN204 as an .ffdata document, including the PLN204S and PLN204Z annexes, from the ledger and the tax adjustments recorded for that year.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_lt_pln204ffdata(
            &PostV1DeclarationsLtPln204FfdataRequest { year: 1000000 },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_mt_company_tax_generate</a>(request: PostV1DeclarationsMtCompanyTaxGenerateRequest) -> Result&lt;PostV1DeclarationsMtCompanyTaxGenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Compute the company income tax return and self-assessment of a year of assessment from the ledger and the recorded tax adjustments: the accounting profit before tax, the add-backs and deductions, the approved donations, capital allowances and losses carried forward, the chargeable income, the 35 % charge, the relief against the tax and the allocation of the distributable profit to the five tax accounts. The Malta Tax and Customs Administration issues the return as a personalised spreadsheet to the registered tax practitioner and publishes no layout, so the XML is a working file and the figures are keyed into that spreadsheet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_mt_company_tax_generate(
            &PostV1DeclarationsMtCompanyTaxGenerateRequest { year: 1000000 },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_mt_annual_return_generate</a>(request: PostV1DeclarationsMtAnnualReturnGenerateRequest) -> Result&lt;PostV1DeclarationsMtAnnualReturnGenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Build the annual return of a year: the company number, registered office and made-up-to date, the share capital, the register of members, the directors and the company secretary and the accounts summary, as the figures the Malta Business Registry asks for on its own screens, plus the printed Annual Return Form of the Seventh Schedule filled in as a PDF for signing.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_mt_annual_return_generate(
            &PostV1DeclarationsMtAnnualReturnGenerateRequest { year: 1000000 },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_pl_jpk_fa_generate</a>(request: PostV1DeclarationsPlJpkFaGenerateRequest) -> Result&lt;PostV1DeclarationsPlJpkFaGenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Generate JPK_FA(4), the on-demand structure with every sales invoice issued in a period, its VAT bases per rate and one row per invoice line. Filed only when the tax office asks for it.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_pl_jpk_fa_generate(
            &PostV1DeclarationsPlJpkFaGenerateRequest {
                date_from: "dateFrom".to_string(),
                date_to: "dateTo".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**date_from:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**date_to:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_pl_jpk_kr_generate</a>(request: PostV1DeclarationsPlJpkKrGenerateRequest) -> Result&lt;PostV1DeclarationsPlJpkKrGenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Generate JPK_KR(1), the on-demand structure with the chart of accounts and its opening balances and turnover, the journal and the double entries behind it. Filed only when the tax office asks for it.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_pl_jpk_kr_generate(
            &PostV1DeclarationsPlJpkKrGenerateRequest {
                date_from: "dateFrom".to_string(),
                date_to: "dateTo".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**date_from:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**date_to:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_pl_jpk_mag_generate</a>(request: PostV1DeclarationsPlJpkMagGenerateRequest) -> Result&lt;PostV1DeclarationsPlJpkMagGenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Generate JPK_MAG(2), the on-demand structure with the warehouse documents of one warehouse: goods received from outside (PZ) or internally (PW) and issued to a customer (WZ) or internally (RW). Filed only when the tax office asks for it.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_pl_jpk_mag_generate(
            &PostV1DeclarationsPlJpkMagGenerateRequest {
                date_from: "dateFrom".to_string(),
                date_to: "dateTo".to_string(),
                warehouse_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**date_from:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**date_to:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**warehouse_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_pl_pit11_generate</a>(request: PostV1DeclarationsPlPit11GenerateRequest) -> Result&lt;PostV1DeclarationsPlPit11GenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Generate PIT-11(29) for every person on the payroll of one year: the pay, the deductible costs, the advance withheld and the social and health contributions taken off it. One document per person, because that is how the form is filed.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_pl_pit11generate(
            &PostV1DeclarationsPlPit11GenerateRequest { year: 1000000 },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_pl_cit8_generate</a>(request: PostV1DeclarationsPlCit8GenerateRequest) -> Result&lt;PostV1DeclarationsPlCit8GenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Generate CIT-8(34), the annual corporate income tax return, from the ledger of the year and the recorded tax adjustments. The tax office code and the small-taxpayer setting come from the e-Deklaracje compliance settings, the seat address from the JPK gateway settings. Names the annexes the figures would need, which are not produced.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_pl_cit8generate(
            &PostV1DeclarationsPlCit8GenerateRequest { year: 1000000 },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_pl_zus_dra_compute</a>(request: PostV1DeclarationsPlZusDraComputeRequest) -> Result&lt;PostV1DeclarationsPlZusDraComputeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Compute the monthly ZUS DRA settlement from the payroll run of one month: the pension, disability, sickness, accident and health insurance contributions and the Labour Fund, Solidarity Fund and guaranteed benefits fund charges, each split between the insured person and the payer. The amounts are carried into Płatnik or ePłatnik by hand.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_pl_zus_dra_compute(
            &PostV1DeclarationsPlZusDraComputeRequest {
                year: 1000000,
                month: 1000000,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_pl_zus_dra_kedu</a>(request: PostV1DeclarationsPlZusDraKeduRequest) -> Result&lt;PostV1DeclarationsPlZusDraKeduResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Build the KEDU file for one month: the ZUS DRA settlement and one ZUS RCA report per person on the payroll, in the schema kedu_5_4 that Płatnik and ePłatnik import. The payer REGON, short name and declaration deadline code come from the ZUS compliance settings; the insurance title code and working time of each person from the employee record.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_pl_zus_dra_kedu(
            &PostV1DeclarationsPlZusDraKeduRequest {
                year: 1000000,
                month: 1000000,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_pl_zus_dra_pdf</a>(request: PostV1DeclarationsPlZusDraPdfRequest) -> Result&lt;PostV1DeclarationsPlZusDraPdfResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Fill the published ZUS DRA form for one month and return it as a PDF. The amounts, the payer identity and the deadline code are the same ones the KEDU file carries; blocks the payroll does not hold (paid benefits, bridging pensions, income declaration of a self-paying person) stay empty.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_pl_zus_dra_pdf(
            &PostV1DeclarationsPlZusDraPdfRequest {
                year: 1000000,
                month: 1000000,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_ro_etransport_build</a>(request: PostV1DeclarationsRoEtransportBuildRequest) -> Result&lt;PostV1DeclarationsRoEtransportBuildResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Build the RO e-Transport declaration for an issued waybill: goods with their tariff codes and masses, the commercial partner, the route and the vehicle. The XML follows the ANAF eTransport v2 schema and is kept as a file on the waybill. Anything listed in blockers has to be filled in before /etransport/send will accept it.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_ro_etransport_build(
            &PostV1DeclarationsRoEtransportBuildRequest {
                waybill_id: "waybillId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**waybill_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_ro_etransport_submit</a>(request: PostV1DeclarationsRoEtransportSubmitRequest) -> Result&lt;PostV1DeclarationsRoEtransportSubmitResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Hand the RO e-Transport declaration for an issued waybill to ANAF under the SPV OAuth token in compliance settings, and return the upload index the UIT is read back with. Answers 422 while any field the ANAF validator requires is still missing.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_ro_etransport_submit(
            &PostV1DeclarationsRoEtransportSubmitRequest {
                waybill_id: "waybillId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**waybill_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_ro_etransport_status</a>(request: PostV1DeclarationsRoEtransportStatusRequest) -> Result&lt;PostV1DeclarationsRoEtransportStatusResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Read the outcome of an e-Transport declaration from ANAF by its upload index, under the SPV OAuth token in compliance settings. Returns the UIT code once the declaration validates.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_ro_etransport_status(
            &PostV1DeclarationsRoEtransportStatusRequest {
                reference: "reference".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**reference:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_li_lohndeklaration_generate</a>(request: PostV1DeclarationsLiLohndeklarationGenerateRequest) -> Result&lt;PostV1DeclarationsLiLohndeklarationGenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Build the annual wage declaration (Lohndeklaration) to the AHV-IV-FAK from the approved payroll runs of the year as the CSV that AHVeasy imports under Lohndeklaration → CSV-Import der Lohndaten: one row per employee with the 18 columns of the AHVeasy template, the AHV-liable wage and the ALV wage.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_li_lohndeklaration_generate(
            &PostV1DeclarationsLiLohndeklarationGenerateRequest { year: 1000000 },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_li_lohnlisten_generate</a>(request: PostV1DeclarationsLiLohnlistenGenerateRequest) -> Result&lt;PostV1DeclarationsLiLohnlistenGenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Build the annual wage list (Lohnliste) of a Liechtenstein employer from the approved payroll runs of the year as the XLSX file the tax administration's eLohnausweis / eLohnlisten application imports: one row per employee with PEID, name, birth date, address, gross wage, wage tax withheld and the settlement period.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_li_lohnlisten_generate(
            &PostV1DeclarationsLiLohnlistenGenerateRequest { year: 1000000 },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_configs_list</a>(request: PostV1DeclarationsConfigsListRequest) -> Result&lt;PostV1DeclarationsConfigsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_configs_list(
            &PostV1DeclarationsConfigsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_configs_update</a>(request: PostV1DeclarationsConfigsUpdateRequest) -> Result&lt;PostV1DeclarationsConfigsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_configs_update(
            &PostV1DeclarationsConfigsUpdateRequest {
                system: "system".to_string(),
                config: HashMap::from([("key".to_string(), "value".to_string())]),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**system:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**config:** `std::collections::HashMap<String, String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">store_the_certificate_or_private_key_a_filing_system_authenticates_with</a>(request: PostV1DeclarationsCertificatesUploadRequest) -> Result&lt;PostV1DeclarationsCertificatesUploadResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .store_the_certificate_or_private_key_a_filing_system_authenticates_with(
            &PostV1DeclarationsCertificatesUploadRequest {
                system: "system".to_string(),
                file_name: "fileName".to_string(),
                content: "content".to_string(),
                passphrase: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**system:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**file_name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**content:** `String` — Base64-encoded PEM or PKCS#12 file
    
</dd>
</dl>

<dl>
<dd>

**passphrase:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_certificates_list</a>(request: PostV1DeclarationsCertificatesListRequest) -> Result&lt;PostV1DeclarationsCertificatesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_certificates_list(
            &PostV1DeclarationsCertificatesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_certificates_delete</a>(request: PostV1DeclarationsCertificatesDeleteRequest) -> Result&lt;PostV1DeclarationsCertificatesDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_certificates_delete(
            &PostV1DeclarationsCertificatesDeleteRequest {
                system: "system".to_string(),
                field_key: PostV1DeclarationsCertificatesDeleteRequestFieldKey::Certificate,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**system:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**field_key:** `PostV1DeclarationsCertificatesDeleteRequestFieldKey` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">which_deadlines_nordlet_can_file_by_itself_for_this_company_and_which_are_switched_on</a>(request: PostV1DeclarationsAutomationListRequest) -> Result&lt;PostV1DeclarationsAutomationListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .which_deadlines_nordlet_can_file_by_itself_for_this_company_and_which_are_switched_on(
            &PostV1DeclarationsAutomationListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_automation_update</a>(request: PostV1DeclarationsAutomationUpdateRequest) -> Result&lt;PostV1DeclarationsAutomationUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_automation_update(
            &PostV1DeclarationsAutomationUpdateRequest {
                rule_key: "ruleKey".to_string(),
                enabled: true,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**rule_key:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**enabled:** `bool` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">send_a_filing_whose_delivery_failed_once_more_with_the_bytes_that_were_generated</a>(request: PostV1DeclarationsSubmissionsRetryRequest) -> Result&lt;PostV1DeclarationsSubmissionsRetryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .send_a_filing_whose_delivery_failed_once_more_with_the_bytes_that_were_generated(
            &PostV1DeclarationsSubmissionsRetryRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_submissions_create</a>(request: PostV1DeclarationsSubmissionsCreateRequest) -> Result&lt;PostV1DeclarationsSubmissionsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_submissions_create(
            &PostV1DeclarationsSubmissionsCreateRequest {
                obligation: PostV1DeclarationsSubmissionsCreateRequestObligation::LtIsaf,
                year: 1000000,
                month: 1000000,
                data_type: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**obligation:** `PostV1DeclarationsSubmissionsCreateRequestObligation` 
    
</dd>
</dl>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**data_type:** `Option<PostV1DeclarationsSubmissionsCreateRequestDataType>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_submissions_mark</a>(request: PostV1DeclarationsSubmissionsMarkRequest) -> Result&lt;PostV1DeclarationsSubmissionsMarkResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_submissions_mark(
            &PostV1DeclarationsSubmissionsMarkRequest {
                id: "id".to_string(),
                status: PostV1DeclarationsSubmissionsMarkRequestStatus::Submitted,
                external_ref: None,
                message: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `PostV1DeclarationsSubmissionsMarkRequestStatus` 
    
</dd>
</dl>

<dl>
<dd>

**external_ref:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**message:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">post_v1_declarations_submissions_list</a>(request: PostV1DeclarationsSubmissionsListRequest) -> Result&lt;PostV1DeclarationsSubmissionsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .declarations
        .post_v1declarations_submissions_list(
            &PostV1DeclarationsSubmissionsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1DeclarationsSubmissionsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1DeclarationsSubmissionsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Ledger
<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_accounts_list</a>(request: PostV1LedgerAccountsListRequest) -> Result&lt;PostV1LedgerAccountsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_accounts_list(
            &PostV1LedgerAccountsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1LedgerAccountsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1LedgerAccountsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_accounts_create</a>(request: PostV1LedgerAccountsCreateRequest) -> Result&lt;PostV1LedgerAccountsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_accounts_create(
            &PostV1LedgerAccountsCreateRequest {
                code: "code".to_string(),
                name: "name".to_string(),
                r#type: PostV1LedgerAccountsCreateRequestType::Asset,
                translations: None,
                parent_id: None,
                is_postable: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**translations:** `Option<std::collections::HashMap<String, PostV1LedgerAccountsCreateRequestTranslationsValue>>` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `PostV1LedgerAccountsCreateRequestType` 
    
</dd>
</dl>

<dl>
<dd>

**parent_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_postable:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_accounts_update</a>(request: PostV1LedgerAccountsUpdateRequest) -> Result&lt;PostV1LedgerAccountsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_accounts_update(
            &PostV1LedgerAccountsUpdateRequest {
                id: "id".to_string(),
                name: None,
                translations: None,
                parent_id: None,
                is_postable: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**translations:** `Option<Option<std::collections::HashMap<String, Option<PostV1LedgerAccountsUpdateRequestTranslationsValue>>>>` 
    
</dd>
</dl>

<dl>
<dd>

**parent_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**is_postable:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_accounts_apply_template</a>(request: PostV1LedgerAccountsApplyTemplateRequest) -> Result&lt;PostV1LedgerAccountsApplyTemplateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_accounts_apply_template(
            &PostV1LedgerAccountsApplyTemplateRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">move_a_company_that_has_posted_nothing_yet_to_the_chart_of_accounts_of_its_country</a>(request: PostV1LedgerAccountsSwitchChartRequest) -> Result&lt;PostV1LedgerAccountsSwitchChartResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Replaces the seeded chart with the chart template of the company country (the Romanian general chart for a company registered in Romania, the Lithuanian standard chart otherwise) and switches the posting defaults with it. Answers 409 when the company already uses that chart, has journal entries, holds accounts created by hand, or has settings that name an account the new chart does not have.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .move_a_company_that_has_posted_nothing_yet_to_the_chart_of_accounts_of_its_country(
            &PostV1LedgerAccountsSwitchChartRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_periods_list</a>(request: PostV1LedgerPeriodsListRequest) -> Result&lt;PostV1LedgerPeriodsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_periods_list(
            &PostV1LedgerPeriodsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1LedgerPeriodsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1LedgerPeriodsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_periods_lock</a>(request: PostV1LedgerPeriodsLockRequest) -> Result&lt;PostV1LedgerPeriodsLockResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_periods_lock(
            &PostV1LedgerPeriodsLockRequest {
                year: 1000000,
                month: 1000000,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_periods_unlock</a>(request: PostV1LedgerPeriodsUnlockRequest) -> Result&lt;PostV1LedgerPeriodsUnlockResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_periods_unlock(
            &PostV1LedgerPeriodsUnlockRequest {
                year: 1000000,
                month: 1000000,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_journal_transactions_list</a>(request: PostV1LedgerJournalTransactionsListRequest) -> Result&lt;PostV1LedgerJournalTransactionsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_journal_transactions_list(
            &PostV1LedgerJournalTransactionsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1LedgerJournalTransactionsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1LedgerJournalTransactionsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_cost_centers_create</a>(request: PostV1LedgerCostCentersCreateRequest) -> Result&lt;PostV1LedgerCostCentersCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_cost_centers_create(
            &PostV1LedgerCostCentersCreateRequest {
                code: "code".to_string(),
                name: "name".to_string(),
                group_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**group_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_cost_centers_update</a>(request: PostV1LedgerCostCentersUpdateRequest) -> Result&lt;PostV1LedgerCostCentersUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_cost_centers_update(
            &PostV1LedgerCostCentersUpdateRequest {
                id: "id".to_string(),
                name: None,
                is_active: None,
                group_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**group_id:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_cost_centers_list</a>(request: PostV1LedgerCostCentersListRequest) -> Result&lt;PostV1LedgerCostCentersListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_cost_centers_list(
            &PostV1LedgerCostCentersListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1LedgerCostCentersListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1LedgerCostCentersListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_cost_center_groups_create</a>(request: PostV1LedgerCostCenterGroupsCreateRequest) -> Result&lt;PostV1LedgerCostCenterGroupsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_cost_center_groups_create(
            &PostV1LedgerCostCenterGroupsCreateRequest {
                code: "code".to_string(),
                name: "name".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_cost_center_groups_update</a>(request: PostV1LedgerCostCenterGroupsUpdateRequest) -> Result&lt;PostV1LedgerCostCenterGroupsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_cost_center_groups_update(
            &PostV1LedgerCostCenterGroupsUpdateRequest {
                id: "id".to_string(),
                code: None,
                name: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_cost_center_groups_delete</a>(request: PostV1LedgerCostCenterGroupsDeleteRequest) -> Result&lt;PostV1LedgerCostCenterGroupsDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_cost_center_groups_delete(
            &PostV1LedgerCostCenterGroupsDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_cost_center_groups_list</a>(request: PostV1LedgerCostCenterGroupsListRequest) -> Result&lt;PostV1LedgerCostCenterGroupsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_cost_center_groups_list(
            &PostV1LedgerCostCenterGroupsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1LedgerCostCenterGroupsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1LedgerCostCenterGroupsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_posting_rules_list</a>(request: PostV1LedgerPostingRulesListRequest) -> Result&lt;PostV1LedgerPostingRulesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_posting_rules_list(
            &PostV1LedgerPostingRulesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_posting_rules_update</a>(request: PostV1LedgerPostingRulesUpdateRequest) -> Result&lt;PostV1LedgerPostingRulesUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_posting_rules_update(
            &PostV1LedgerPostingRulesUpdateRequest {
                rules: vec![PostV1LedgerPostingRulesUpdateRequestRulesItem {
                    key: PostV1LedgerPostingRulesUpdateRequestRulesItemKey::SalesReceivable,
                    account_code: None,
                }],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**rules:** `Vec<PostV1LedgerPostingRulesUpdateRequestRulesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_owners_create</a>(request: PostV1LedgerOwnersCreateRequest) -> Result&lt;PostV1LedgerOwnersCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_owners_create(
            &PostV1LedgerOwnersCreateRequest {
                name: "name".to_string(),
                code: None,
                equity_account_code: None,
                shares_quantity: None,
                shares_amount: None,
                shares_type: None,
                shares_acquisition_date: None,
                withholding_tax_percent: None,
                partner_liability: None,
                special_balance_required: None,
                supplementary_balance_required: None,
                address: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**equity_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**shares_quantity:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**shares_amount:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**shares_type:** `Option<PostV1LedgerOwnersCreateRequestSharesType>` 
    
</dd>
</dl>

<dl>
<dd>

**shares_acquisition_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**withholding_tax_percent:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**partner_liability:** `Option<Option<PostV1LedgerOwnersCreateRequestPartnerLiability>>` 
    
</dd>
</dl>

<dl>
<dd>

**special_balance_required:** `Option<Option<bool>>` 
    
</dd>
</dl>

<dl>
<dd>

**supplementary_balance_required:** `Option<Option<bool>>` 
    
</dd>
</dl>

<dl>
<dd>

**address:** `Option<PostV1LedgerOwnersCreateRequestAddress>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_owners_update</a>(request: PostV1LedgerOwnersUpdateRequest) -> Result&lt;PostV1LedgerOwnersUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_owners_update(
            &PostV1LedgerOwnersUpdateRequest {
                id: "id".to_string(),
                name: None,
                code: None,
                equity_account_code: None,
                shares_quantity: None,
                shares_amount: None,
                shares_type: None,
                shares_acquisition_date: None,
                withholding_tax_percent: None,
                partner_liability: None,
                special_balance_required: None,
                supplementary_balance_required: None,
                address: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**equity_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**shares_quantity:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**shares_amount:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**shares_type:** `Option<Option<PostV1LedgerOwnersUpdateRequestSharesType>>` 
    
</dd>
</dl>

<dl>
<dd>

**shares_acquisition_date:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**withholding_tax_percent:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**partner_liability:** `Option<Option<PostV1LedgerOwnersUpdateRequestPartnerLiability>>` 
    
</dd>
</dl>

<dl>
<dd>

**special_balance_required:** `Option<Option<bool>>` 
    
</dd>
</dl>

<dl>
<dd>

**supplementary_balance_required:** `Option<Option<bool>>` 
    
</dd>
</dl>

<dl>
<dd>

**address:** `Option<Option<PostV1LedgerOwnersUpdateRequestAddress>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_owners_delete</a>(request: PostV1LedgerOwnersDeleteRequest) -> Result&lt;PostV1LedgerOwnersDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_owners_delete(
            &PostV1LedgerOwnersDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_owners_list</a>(request: PostV1LedgerOwnersListRequest) -> Result&lt;PostV1LedgerOwnersListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_owners_list(
            &PostV1LedgerOwnersListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1LedgerOwnersListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1LedgerOwnersListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_journal_transactions_get</a>(request: PostV1LedgerJournalTransactionsGetRequest) -> Result&lt;PostV1LedgerJournalTransactionsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_journal_transactions_get(
            &PostV1LedgerJournalTransactionsGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">post_v1_ledger_journal_transactions_create</a>(request: PostV1LedgerJournalTransactionsCreateRequest) -> Result&lt;PostV1LedgerJournalTransactionsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .post_v1ledger_journal_transactions_create(
            &PostV1LedgerJournalTransactionsCreateRequest {
                date: "date".to_string(),
                entries: vec![PostV1LedgerJournalTransactionsCreateRequestEntriesItem {
                    account_code: "accountCode".to_string(),
                    ..Default::default()
                }],
                description: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**entries:** `Vec<PostV1LedgerJournalTransactionsCreateRequestEntriesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">national_statement_layouts_available_to_the_company</a>(request: PostV1LedgerStatementRowsSchemesRequest) -> Result&lt;PostV1LedgerStatementRowsSchemesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

The rows or codes of each return or registry deposit of the company country that are filled from account balances. Accounts fall into a row by the layout defaults for the standard chart of accounts unless mapped under Settings → Statement rows.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .national_statement_layouts_available_to_the_company(
            &PostV1LedgerStatementRowsSchemesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">accounts_placed_on_the_rows_of_a_statement_layout_with_the_row_totals_of_a_period</a>(request: PostV1LedgerStatementRowsListRequest) -> Result&lt;PostV1LedgerStatementRowsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .accounts_placed_on_the_rows_of_a_statement_layout_with_the_row_totals_of_a_period(
            &PostV1LedgerStatementRowsListRequest {
                scheme: "scheme".to_string(),
                from_date: None,
                to_date: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**scheme:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**from_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">map_an_account_or_an_account_code_prefix_to_a_row_of_a_statement_layout</a>(request: PostV1LedgerStatementRowsSetRequest) -> Result&lt;PostV1LedgerStatementRowsSetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

A mapping on a code prefix covers every account whose code starts with it; the longest matching prefix wins. An empty rowCode removes the mapping so the layout default applies again.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .map_an_account_or_an_account_code_prefix_to_a_row_of_a_statement_layout(
            &PostV1LedgerStatementRowsSetRequest {
                scheme: "scheme".to_string(),
                account_code: "accountCode".to_string(),
                row_code: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**scheme:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**account_code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**row_code:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">officers_of_the_company</a>(request: PostV1OfficersListRequest) -> Result&lt;PostV1OfficersListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Directors, board members, the company secretary, representatives and liquidators, with their personal identifier, appointment and resignation dates and whether they sign the annual accounts. Annual returns and registry deposits are built from this register.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .officers_of_the_company(
            &PostV1OfficersListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">record_an_officer_of_the_company</a>(request: PostV1OfficersCreateRequest) -> Result&lt;PostV1OfficersCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .record_an_officer_of_the_company(
            &PostV1OfficersCreateRequest {
                name: "name".to_string(),
                role: PostV1OfficersCreateRequestRole::Director,
                personal_code: None,
                birth_date: None,
                appointed_on: None,
                power_notary: None,
                resigned_on: None,
                signs_accounts: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**role:** `PostV1OfficersCreateRequestRole` 
    
</dd>
</dl>

<dl>
<dd>

**personal_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**birth_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**appointed_on:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**power_notary:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**resigned_on:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**signs_accounts:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">change_a_recorded_officer</a>(request: PostV1OfficersUpdateRequest) -> Result&lt;PostV1OfficersUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .change_a_recorded_officer(
            &PostV1OfficersUpdateRequest {
                id: "id".to_string(),
                name: "name".to_string(),
                role: PostV1OfficersUpdateRequestRole::Director,
                personal_code: None,
                birth_date: None,
                appointed_on: None,
                power_notary: None,
                resigned_on: None,
                signs_accounts: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**role:** `PostV1OfficersUpdateRequestRole` 
    
</dd>
</dl>

<dl>
<dd>

**personal_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**birth_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**appointed_on:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**power_notary:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**resigned_on:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**signs_accounts:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">remove_a_recorded_officer</a>(request: PostV1OfficersDeleteRequest) -> Result&lt;PostV1OfficersDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ledger
        .remove_a_recorded_officer(
            &PostV1OfficersDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Migration
<details><summary><code>client.migration.<a href="/src/api/resources/migration/client.rs">check_a_historical_books_package_without_writing_anything</a>(request: PostV1MigrationBooksValidateRequest) -> Result&lt;PostV1MigrationBooksValidateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Runs every check the import runs (accounts, partners, balances, open invoices, assets, stock) and returns the same summary and warnings, then rolls everything back. Nothing is stored.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .migration
        .check_a_historical_books_package_without_writing_anything(
            &PostV1MigrationBooksValidateRequest {
                cutover_date: "cutoverDate".to_string(),
                source: None,
                accounts: None,
                partners: None,
                items: None,
                opening_balances: None,
                journal: None,
                open_receivables: None,
                open_payables: None,
                asset_groups: None,
                fixed_assets: None,
                stock: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**cutover_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**source:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**accounts:** `Option<Vec<PostV1MigrationBooksValidateRequestAccountsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**partners:** `Option<Vec<PostV1MigrationBooksValidateRequestPartnersItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**items:** `Option<Vec<PostV1MigrationBooksValidateRequestItemsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**opening_balances:** `Option<PostV1MigrationBooksValidateRequestOpeningBalances>` 
    
</dd>
</dl>

<dl>
<dd>

**journal:** `Option<Vec<PostV1MigrationBooksValidateRequestJournalItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**open_receivables:** `Option<Vec<PostV1MigrationBooksValidateRequestOpenReceivablesItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**open_payables:** `Option<Vec<PostV1MigrationBooksValidateRequestOpenPayablesItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**asset_groups:** `Option<Vec<PostV1MigrationBooksValidateRequestAssetGroupsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**fixed_assets:** `Option<Vec<PostV1MigrationBooksValidateRequestFixedAssetsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**stock:** `Option<Vec<PostV1MigrationBooksValidateRequestStockItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.migration.<a href="/src/api/resources/migration/client.rs">import_historical_books_from_a_previous_accounting_system</a>(request: PostV1MigrationBooksImportRequest) -> Result&lt;PostV1MigrationBooksImportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Brings a company over from another system in one call: chart of accounts, partners, items, opening balances (or the full journal history), open customer and supplier invoices, fixed assets with their accumulated depreciation, and stock on hand. The whole package is written in one database transaction — if any row fails, nothing is stored.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .migration
        .import_historical_books_from_a_previous_accounting_system(
            &PostV1MigrationBooksImportRequest {
                cutover_date: "cutoverDate".to_string(),
                source: None,
                accounts: None,
                partners: None,
                items: None,
                opening_balances: None,
                journal: None,
                open_receivables: None,
                open_payables: None,
                asset_groups: None,
                fixed_assets: None,
                stock: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**cutover_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**source:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**accounts:** `Option<Vec<PostV1MigrationBooksImportRequestAccountsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**partners:** `Option<Vec<PostV1MigrationBooksImportRequestPartnersItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**items:** `Option<Vec<PostV1MigrationBooksImportRequestItemsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**opening_balances:** `Option<PostV1MigrationBooksImportRequestOpeningBalances>` 
    
</dd>
</dl>

<dl>
<dd>

**journal:** `Option<Vec<PostV1MigrationBooksImportRequestJournalItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**open_receivables:** `Option<Vec<PostV1MigrationBooksImportRequestOpenReceivablesItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**open_payables:** `Option<Vec<PostV1MigrationBooksImportRequestOpenPayablesItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**asset_groups:** `Option<Vec<PostV1MigrationBooksImportRequestAssetGroupsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**fixed_assets:** `Option<Vec<PostV1MigrationBooksImportRequestFixedAssetsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**stock:** `Option<Vec<PostV1MigrationBooksImportRequestStockItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Assets
<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">post_v1_assets_groups_create</a>(request: PostV1AssetsGroupsCreateRequest) -> Result&lt;PostV1AssetsGroupsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .assets
        .post_v1assets_groups_create(
            &PostV1AssetsGroupsCreateRequest {
                code: "code".to_string(),
                name: "name".to_string(),
                asset_account_code: "assetAccountCode".to_string(),
                depreciation_account_code: "depreciationAccountCode".to_string(),
                default_useful_life_months: None,
                expense_account_code: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**default_useful_life_months:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**asset_account_code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**depreciation_account_code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**expense_account_code:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">post_v1_assets_groups_list</a>(request: PostV1AssetsGroupsListRequest) -> Result&lt;PostV1AssetsGroupsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .assets
        .post_v1assets_groups_list(
            &PostV1AssetsGroupsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1AssetsGroupsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1AssetsGroupsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">post_v1_assets_assets_create</a>(request: PostV1AssetsAssetsCreateRequest) -> Result&lt;PostV1AssetsAssetsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .assets
        .post_v1assets_assets_create(
            &PostV1AssetsAssetsCreateRequest {
                group_id: "groupId".to_string(),
                code: "code".to_string(),
                name: "name".to_string(),
                acquisition_date: "acquisitionDate".to_string(),
                acquisition_cost: "acquisitionCost".to_string(),
                depreciation_start_date: None,
                salvage_value: None,
                useful_life_months: None,
                notes: None,
                documents: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**group_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**acquisition_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**depreciation_start_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**acquisition_cost:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**salvage_value:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**useful_life_months:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**documents:** `Option<Vec<PostV1AssetsAssetsCreateRequestDocumentsItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">post_v1_assets_assets_update</a>(request: PostV1AssetsAssetsUpdateRequest) -> Result&lt;PostV1AssetsAssetsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .assets
        .post_v1assets_assets_update(
            &PostV1AssetsAssetsUpdateRequest {
                id: "id".to_string(),
                group_id: None,
                code: None,
                name: None,
                acquisition_date: None,
                depreciation_start_date: None,
                acquisition_cost: None,
                salvage_value: None,
                useful_life_months: None,
                notes: None,
                documents: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**group_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**acquisition_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**depreciation_start_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**acquisition_cost:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**salvage_value:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**useful_life_months:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**documents:** `Option<Vec<PostV1AssetsAssetsUpdateRequestDocumentsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">post_v1_assets_assets_input_vat</a>(request: PostV1AssetsAssetsInputVatRequest) -> Result&lt;PostV1AssetsAssetsInputVatResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Record the input VAT facts of a capital good that the annual VAT return needs for the adjustment of the deduction over the adjustment period (Article 187 of the VAT Directive, § 15a UStG): the input VAT on the acquisition, the date of first use, the share of use for deductible turnover at first use, whether it is land or a building (ten-year period instead of five), and every later year in which the share changed or the good was sold or withdrawn. Allowed also after depreciation has been posted.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.assets.post_v1assets_assets_input_vat(&PostV1AssetsAssetsInputVatRequest {
        id: "id".to_string(),
        input_vat_real_estate: true,
        input_vat_use_changes: vec![PostV1AssetsAssetsInputVatRequestInputVatUseChangesItem {
            year: 1000000,
            percent: "percent".to_string(),
            reason: PostV1AssetsAssetsInputVatRequestInputVatUseChangesItemReason::UseChange
        }],
        input_vat_amount: None,
        input_vat_first_use_date: None,
        input_vat_deductible_percent: None
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**input_vat_amount:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**input_vat_first_use_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**input_vat_deductible_percent:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**input_vat_real_estate:** `bool` 
    
</dd>
</dl>

<dl>
<dd>

**input_vat_use_changes:** `Vec<PostV1AssetsAssetsInputVatRequestInputVatUseChangesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">post_v1_assets_assets_get</a>(request: PostV1AssetsAssetsGetRequest) -> Result&lt;PostV1AssetsAssetsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .assets
        .post_v1assets_assets_get(
            &PostV1AssetsAssetsGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">post_v1_assets_assets_list</a>(request: PostV1AssetsAssetsListRequest) -> Result&lt;PostV1AssetsAssetsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .assets
        .post_v1assets_assets_list(
            &PostV1AssetsAssetsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1AssetsAssetsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1AssetsAssetsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">post_v1_assets_assets_modernize</a>(request: PostV1AssetsAssetsModernizeRequest) -> Result&lt;PostV1AssetsAssetsModernizeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .assets
        .post_v1assets_assets_modernize(
            &PostV1AssetsAssetsModernizeRequest {
                id: "id".to_string(),
                date: "date".to_string(),
                amount: "amount".to_string(),
                added_life_months: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**amount:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**added_life_months:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">post_v1_assets_depreciation_preview</a>(request: PostV1AssetsDepreciationPreviewRequest) -> Result&lt;PostV1AssetsDepreciationPreviewResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .assets
        .post_v1assets_depreciation_preview(
            &PostV1AssetsDepreciationPreviewRequest {
                year: 1000000,
                month: 1000000,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">post_v1_assets_depreciation_post</a>(request: PostV1AssetsDepreciationPostRequest) -> Result&lt;PostV1AssetsDepreciationPostResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .assets
        .post_v1assets_depreciation_post(
            &PostV1AssetsDepreciationPostRequest {
                year: 1000000,
                month: 1000000,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Hr
<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_positions_create</a>(request: PostV1HrPositionsCreateRequest) -> Result&lt;PostV1HrPositionsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_positions_create(
            &PostV1HrPositionsCreateRequest {
                name: "name".to_string(),
                code: None,
                translations: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**translations:** `Option<std::collections::HashMap<String, PostV1HrPositionsCreateRequestTranslationsValue>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_positions_update</a>(request: PostV1HrPositionsUpdateRequest) -> Result&lt;PostV1HrPositionsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_positions_update(
            &PostV1HrPositionsUpdateRequest {
                id: "id".to_string(),
                code: None,
                name: None,
                translations: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**translations:** `Option<Option<std::collections::HashMap<String, Option<PostV1HrPositionsUpdateRequestTranslationsValue>>>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_positions_list</a>(request: PostV1HrPositionsListRequest) -> Result&lt;PostV1HrPositionsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_positions_list(
            &PostV1HrPositionsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1HrPositionsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1HrPositionsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_employees_create</a>(request: PostV1HrEmployeesCreateRequest) -> Result&lt;PostV1HrEmployeesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_employees_create(
            &PostV1HrEmployeesCreateRequest {
                first_name: "firstName".to_string(),
                last_name: "lastName".to_string(),
                code: None,
                personal_code: None,
                birth_date: None,
                email: None,
                phone: None,
                address: None,
                iban: None,
                social_insurance_no: None,
                social_insurance_start: None,
                hire_date: None,
                apply_allowance: None,
                allowance_override: None,
                pension_accumulation: None,
                payroll_options: None,
                notes: None,
                attributes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**first_name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**last_name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**personal_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**birth_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**phone:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**address:** `Option<PostV1HrEmployeesCreateRequestAddress>` 
    
</dd>
</dl>

<dl>
<dd>

**iban:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**social_insurance_no:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**social_insurance_start:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**hire_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**apply_allowance:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**allowance_override:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**pension_accumulation:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**payroll_options:** `Option<std::collections::HashMap<String, String>>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**attributes:** `Option<Vec<PostV1HrEmployeesCreateRequestAttributesItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_employees_update</a>(request: PostV1HrEmployeesUpdateRequest) -> Result&lt;PostV1HrEmployeesUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_employees_update(
            &PostV1HrEmployeesUpdateRequest {
                id: "id".to_string(),
                code: None,
                first_name: None,
                last_name: None,
                personal_code: None,
                birth_date: None,
                email: None,
                phone: None,
                address: None,
                iban: None,
                social_insurance_no: None,
                social_insurance_start: None,
                hire_date: None,
                apply_allowance: None,
                allowance_override: None,
                pension_accumulation: None,
                payroll_options: None,
                notes: None,
                attributes: None,
                termination_date: None,
                status: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**first_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**last_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**personal_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**birth_date:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**phone:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**address:** `Option<Option<PostV1HrEmployeesUpdateRequestAddress>>` 
    
</dd>
</dl>

<dl>
<dd>

**iban:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**social_insurance_no:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**social_insurance_start:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**hire_date:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**apply_allowance:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**allowance_override:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**pension_accumulation:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**payroll_options:** `Option<std::collections::HashMap<String, String>>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**attributes:** `Option<Vec<PostV1HrEmployeesUpdateRequestAttributesItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**termination_date:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<PostV1HrEmployeesUpdateRequestStatus>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_employees_get</a>(request: PostV1HrEmployeesGetRequest) -> Result&lt;PostV1HrEmployeesGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_employees_get(
            &PostV1HrEmployeesGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">extra_employee_details_the_country_of_the_company_asks_for</a>(request: PostV1HrEmployeesFieldsRequest) -> Result&lt;PostV1HrEmployeesFieldsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Attributes a filing of the company country needs about a person that the shared employee record does not carry, such as the sex and place of birth an Italian income certificate asks for. Their values are kept in the payrollOptions of the employee.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .extra_employee_details_the_country_of_the_company_asks_for(
            &PostV1HrEmployeesFieldsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_employees_list</a>(request: PostV1HrEmployeesListRequest) -> Result&lt;PostV1HrEmployeesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_employees_list(
            &PostV1HrEmployeesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1HrEmployeesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1HrEmployeesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_employees_delete</a>(request: PostV1HrEmployeesDeleteRequest) -> Result&lt;PostV1HrEmployeesDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_employees_delete(
            &PostV1HrEmployeesDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">blank_an_employees_personal_data_and_hide_the_record</a>(request: PostV1HrEmployeesAnonymizeRequest) -> Result&lt;PostV1HrEmployeesAnonymizeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Replaces the name with a placeholder and removes personal code, birth date, contact details, address, bank account, social-insurance number, notes and sick-leave reasons. Payroll and contract rows stay linked to the record for the statutory retention period.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .blank_an_employees_personal_data_and_hide_the_record(
            &PostV1HrEmployeesAnonymizeRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_contracts_create</a>(request: PostV1HrContractsCreateRequest) -> Result&lt;PostV1HrContractsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_contracts_create(
            &PostV1HrContractsCreateRequest {
                employee_id: "employeeId".to_string(),
                start_date: "startDate".to_string(),
                base_salary: "baseSalary".to_string(),
                position_id: None,
                department_id: None,
                schedule_id: None,
                agreement_id: None,
                contract_no: None,
                r#type: None,
                end_date: None,
                salary_type: None,
                work_hours: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**employee_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**position_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**department_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**schedule_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**agreement_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**contract_no:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<PostV1HrContractsCreateRequestType>` 
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**base_salary:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**salary_type:** `Option<PostV1HrContractsCreateRequestSalaryType>` 
    
</dd>
</dl>

<dl>
<dd>

**work_hours:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_contracts_end</a>(request: PostV1HrContractsEndRequest) -> Result&lt;PostV1HrContractsEndResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_contracts_end(
            &PostV1HrContractsEndRequest {
                id: "id".to_string(),
                end_date: "endDate".to_string(),
                end_reason: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**end_reason:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_contracts_list</a>(request: PostV1HrContractsListRequest) -> Result&lt;PostV1HrContractsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_contracts_list(
            &PostV1HrContractsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1HrContractsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1HrContractsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_leave_balances_set</a>(request: PostV1HrLeaveBalancesSetRequest) -> Result&lt;PostV1HrLeaveBalancesSetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_leave_balances_set(
            &PostV1HrLeaveBalancesSetRequest {
                employee_id: "employeeId".to_string(),
                year: 1000000,
                entitled_days: "entitledDays".to_string(),
                used_days: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**employee_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**entitled_days:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**used_days:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_leave_balances_list</a>(request: PostV1HrLeaveBalancesListRequest) -> Result&lt;PostV1HrLeaveBalancesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_leave_balances_list(
            &PostV1HrLeaveBalancesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**employee_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**year:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_incapacity_certificates_create</a>(request: PostV1HrIncapacityCertificatesCreateRequest) -> Result&lt;PostV1HrIncapacityCertificatesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_incapacity_certificates_create(
            &PostV1HrIncapacityCertificatesCreateRequest {
                employee_id: "employeeId".to_string(),
                number: "number".to_string(),
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
                series: None,
                reason: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**employee_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**series:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**number:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**reason:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_incapacity_certificates_list</a>(request: PostV1HrIncapacityCertificatesListRequest) -> Result&lt;PostV1HrIncapacityCertificatesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_incapacity_certificates_list(
            &PostV1HrIncapacityCertificatesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1HrIncapacityCertificatesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1HrIncapacityCertificatesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_employees_records_create</a>(request: PostV1HrEmployeesRecordsCreateRequest) -> Result&lt;PostV1HrEmployeesRecordsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_employees_records_create(
            &PostV1HrEmployeesRecordsCreateRequest {
                employee_id: "employeeId".to_string(),
                r#type: PostV1HrEmployeesRecordsCreateRequestType::Education,
                title: "title".to_string(),
                institution: None,
                issued_at: None,
                valid_until: None,
                file_id: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**employee_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `PostV1HrEmployeesRecordsCreateRequestType` 
    
</dd>
</dl>

<dl>
<dd>

**title:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**institution:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**issued_at:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**valid_until:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_employees_records_update</a>(request: PostV1HrEmployeesRecordsUpdateRequest) -> Result&lt;PostV1HrEmployeesRecordsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_employees_records_update(
            &PostV1HrEmployeesRecordsUpdateRequest {
                id: "id".to_string(),
                r#type: None,
                title: None,
                institution: None,
                issued_at: None,
                valid_until: None,
                file_id: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<PostV1HrEmployeesRecordsUpdateRequestType>` 
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**institution:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**issued_at:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**valid_until:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_employees_records_delete</a>(request: PostV1HrEmployeesRecordsDeleteRequest) -> Result&lt;PostV1HrEmployeesRecordsDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_employees_records_delete(
            &PostV1HrEmployeesRecordsDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_employees_records_list</a>(request: PostV1HrEmployeesRecordsListRequest) -> Result&lt;PostV1HrEmployeesRecordsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_employees_records_list(
            &PostV1HrEmployeesRecordsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1HrEmployeesRecordsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1HrEmployeesRecordsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_employees_attachments_list</a>(request: PostV1HrEmployeesAttachmentsListRequest) -> Result&lt;PostV1HrEmployeesAttachmentsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_employees_attachments_list(
            &PostV1HrEmployeesAttachmentsListRequest {
                employee_id: "employeeId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**employee_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_timesheets_generate</a>(request: PostV1HrTimesheetsGenerateRequest) -> Result&lt;PostV1HrTimesheetsGenerateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_timesheets_generate(
            &PostV1HrTimesheetsGenerateRequest {
                year: 1000000,
                month: 1000000,
                employee_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**employee_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_timesheets_upsert</a>(request: PostV1HrTimesheetsUpsertRequest) -> Result&lt;PostV1HrTimesheetsUpsertResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_timesheets_upsert(
            &PostV1HrTimesheetsUpsertRequest {
                employee_id: "employeeId".to_string(),
                year: 1000000,
                month: 1000000,
                days: vec![PostV1HrTimesheetsUpsertRequestDaysItem {
                    day: 1000000,
                    hours: "hours".to_string(),
                    r#type: PostV1HrTimesheetsUpsertRequestDaysItemType::Work,
                }],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**employee_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**days:** `Vec<PostV1HrTimesheetsUpsertRequestDaysItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_timesheets_get</a>(request: PostV1HrTimesheetsGetRequest) -> Result&lt;PostV1HrTimesheetsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_timesheets_get(
            &PostV1HrTimesheetsGetRequest {
                employee_id: "employeeId".to_string(),
                year: 1000000,
                month: 1000000,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**employee_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_timesheets_list</a>(request: PostV1HrTimesheetsListRequest) -> Result&lt;PostV1HrTimesheetsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_timesheets_list(
            &PostV1HrTimesheetsListRequest {
                year: 1000000,
                month: 1000000,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">post_v1_hr_timesheets_delete</a>(request: PostV1HrTimesheetsDeleteRequest) -> Result&lt;PostV1HrTimesheetsDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hr
        .post_v1hr_timesheets_delete(
            &PostV1HrTimesheetsDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Fleet
<details><summary><code>client.fleet.<a href="/src/api/resources/fleet/client.rs">post_v1_fleet_vehicles_create</a>(request: PostV1FleetVehiclesCreateRequest) -> Result&lt;PostV1FleetVehiclesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .fleet
        .post_v1fleet_vehicles_create(
            &PostV1FleetVehiclesCreateRequest {
                plate_number: "plateNumber".to_string(),
                make: "make".to_string(),
                model: "model".to_string(),
                year: None,
                vin: None,
                fuel_type: None,
                acquisition_date: None,
                market_value: None,
                fixed_asset_id: None,
                technical_inspection_due: None,
                insurance_due: None,
                notes: None,
                documents: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**plate_number:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**make:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**model:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**year:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**vin:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**fuel_type:** `Option<PostV1FleetVehiclesCreateRequestFuelType>` 
    
</dd>
</dl>

<dl>
<dd>

**acquisition_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**market_value:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**fixed_asset_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**technical_inspection_due:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**insurance_due:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**documents:** `Option<Vec<PostV1FleetVehiclesCreateRequestDocumentsItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.fleet.<a href="/src/api/resources/fleet/client.rs">post_v1_fleet_vehicles_update</a>(request: PostV1FleetVehiclesUpdateRequest) -> Result&lt;PostV1FleetVehiclesUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .fleet
        .post_v1fleet_vehicles_update(
            &PostV1FleetVehiclesUpdateRequest {
                id: "id".to_string(),
                plate_number: None,
                make: None,
                model: None,
                year: None,
                vin: None,
                fuel_type: None,
                acquisition_date: None,
                market_value: None,
                fixed_asset_id: None,
                technical_inspection_due: None,
                insurance_due: None,
                status: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**plate_number:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**make:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**model:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**year:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**vin:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**fuel_type:** `Option<Option<PostV1FleetVehiclesUpdateRequestFuelType>>` 
    
</dd>
</dl>

<dl>
<dd>

**acquisition_date:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**market_value:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**fixed_asset_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**technical_inspection_due:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**insurance_due:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<PostV1FleetVehiclesUpdateRequestStatus>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.fleet.<a href="/src/api/resources/fleet/client.rs">post_v1_fleet_vehicles_get</a>(request: PostV1FleetVehiclesGetRequest) -> Result&lt;PostV1FleetVehiclesGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .fleet
        .post_v1fleet_vehicles_get(
            &PostV1FleetVehiclesGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.fleet.<a href="/src/api/resources/fleet/client.rs">post_v1_fleet_vehicles_list</a>(request: PostV1FleetVehiclesListRequest) -> Result&lt;PostV1FleetVehiclesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .fleet
        .post_v1fleet_vehicles_list(
            &PostV1FleetVehiclesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1FleetVehiclesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1FleetVehiclesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.fleet.<a href="/src/api/resources/fleet/client.rs">post_v1_fleet_assignments_create</a>(request: PostV1FleetAssignmentsCreateRequest) -> Result&lt;PostV1FleetAssignmentsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .fleet
        .post_v1fleet_assignments_create(
            &PostV1FleetAssignmentsCreateRequest {
                vehicle_id: "vehicleId".to_string(),
                employee_id: "employeeId".to_string(),
                from_date: "fromDate".to_string(),
                to_date: None,
                private_use: None,
                employer_pays_fuel: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**vehicle_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**employee_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**private_use:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**employer_pays_fuel:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.fleet.<a href="/src/api/resources/fleet/client.rs">post_v1_fleet_assignments_end</a>(request: PostV1FleetAssignmentsEndRequest) -> Result&lt;PostV1FleetAssignmentsEndResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .fleet
        .post_v1fleet_assignments_end(
            &PostV1FleetAssignmentsEndRequest {
                id: "id".to_string(),
                to_date: "toDate".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.fleet.<a href="/src/api/resources/fleet/client.rs">post_v1_fleet_assignments_list</a>(request: PostV1FleetAssignmentsListRequest) -> Result&lt;PostV1FleetAssignmentsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .fleet
        .post_v1fleet_assignments_list(
            &PostV1FleetAssignmentsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1FleetAssignmentsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1FleetAssignmentsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.fleet.<a href="/src/api/resources/fleet/client.rs">post_v1_fleet_natura_preview</a>(request: PostV1FleetNaturaPreviewRequest) -> Result&lt;PostV1FleetNaturaPreviewResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .fleet
        .post_v1fleet_natura_preview(
            &PostV1FleetNaturaPreviewRequest {
                year: 1000000,
                month: 1000000,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Payroll
<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">post_v1_payroll_departments_create</a>(request: PostV1PayrollDepartmentsCreateRequest) -> Result&lt;PostV1PayrollDepartmentsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .payroll
        .post_v1payroll_departments_create(
            &PostV1PayrollDepartmentsCreateRequest {
                code: "code".to_string(),
                name: "name".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">post_v1_payroll_departments_list</a>(request: PostV1PayrollDepartmentsListRequest) -> Result&lt;PostV1PayrollDepartmentsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .payroll
        .post_v1payroll_departments_list(
            &PostV1PayrollDepartmentsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">post_v1_payroll_schedules_create</a>(request: PostV1PayrollSchedulesCreateRequest) -> Result&lt;PostV1PayrollSchedulesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .payroll
        .post_v1payroll_schedules_create(
            &PostV1PayrollSchedulesCreateRequest {
                code: "code".to_string(),
                name: "name".to_string(),
                hours_per_week: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**hours_per_week:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">post_v1_payroll_schedules_list</a>(request: PostV1PayrollSchedulesListRequest) -> Result&lt;PostV1PayrollSchedulesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .payroll
        .post_v1payroll_schedules_list(
            &PostV1PayrollSchedulesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">calculate_one_employee_payment_under_the_rules_of_the_company_country</a>(request: PostV1PayrollCalcRequest) -> Result&lt;PostV1PayrollCalcResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .payroll
        .calculate_one_employee_payment_under_the_rules_of_the_company_country(
            &PostV1PayrollCalcRequest {
                taxable_base: "taxableBase".to_string(),
                date: "date".to_string(),
                apply_allowance: None,
                allowance_override: None,
                pension_accumulation: None,
                fixed_term: None,
                benefit_in_kind: None,
                options: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**taxable_base:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**apply_allowance:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**allowance_override:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**pension_accumulation:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**fixed_term:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**benefit_in_kind:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**options:** `Option<std::collections::HashMap<String, String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">post_v1_payroll_runs_create</a>(request: PostV1PayrollRunsCreateRequest) -> Result&lt;PostV1PayrollRunsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .payroll
        .post_v1payroll_runs_create(
            &PostV1PayrollRunsCreateRequest {
                year: 1000000,
                month: 1000000,
                include_natura: None,
                gross_overrides: None,
                lines: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**include_natura:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**gross_overrides:** `Option<Vec<PostV1PayrollRunsCreateRequestGrossOverridesItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Option<Vec<PostV1PayrollRunsCreateRequestLinesItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">post_v1_payroll_runs_get</a>(request: PostV1PayrollRunsGetRequest) -> Result&lt;PostV1PayrollRunsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .payroll
        .post_v1payroll_runs_get(
            &PostV1PayrollRunsGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">post_v1_payroll_runs_list</a>(request: PostV1PayrollRunsListRequest) -> Result&lt;PostV1PayrollRunsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .payroll
        .post_v1payroll_runs_list(
            &PostV1PayrollRunsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1PayrollRunsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1PayrollRunsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">record_the_time_a_person_worked_in_a_payroll_line</a>(request: PostV1PayrollLinesAttendanceRequest) -> Result&lt;PostV1PayrollLinesAttendanceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

The days and hours worked, the days on the register and the average hourly earnings that some countries report per employment. The Czech monthly employer report asks for all four. They can be set while the run is a draft.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .payroll
        .record_the_time_a_person_worked_in_a_payroll_line(
            &PostV1PayrollLinesAttendanceRequest {
                id: "id".to_string(),
                days_worked: None,
                hours_worked: None,
                registered_days: None,
                average_hourly_earnings: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**days_worked:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**hours_worked:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**registered_days:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**average_hourly_earnings:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">post_v1_payroll_runs_approve</a>(request: PostV1PayrollRunsApproveRequest) -> Result&lt;PostV1PayrollRunsApproveResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .payroll
        .post_v1payroll_runs_approve(
            &PostV1PayrollRunsApproveRequest {
                id: "id".to_string(),
                wage_account_code: None,
                employer_account_code: None,
                payable_account_code: None,
                gpm_account_code: None,
                sodra_account_code: None,
                employer_social_account_code: None,
                deduction_account_code: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**wage_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**employer_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**payable_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**gpm_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sodra_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**employer_social_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**deduction_account_code:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">post_v1_payroll_runs_cancel</a>(request: PostV1PayrollRunsCancelRequest) -> Result&lt;PostV1PayrollRunsCancelResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .payroll
        .post_v1payroll_runs_cancel(
            &PostV1PayrollRunsCancelRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">post_v1_payroll_payments_export</a>(request: PostV1PayrollPaymentsExportRequest) -> Result&lt;PostV1PayrollPaymentsExportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .payroll
        .post_v1payroll_payments_export(
            &PostV1PayrollPaymentsExportRequest {
                run_id: "runId".to_string(),
                bank_account_id: "bankAccountId".to_string(),
                execution_date: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**bank_account_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**execution_date:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Agreements
<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">post_v1_agreements_types_create</a>(request: PostV1AgreementsTypesCreateRequest) -> Result&lt;PostV1AgreementsTypesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .agreements
        .post_v1agreements_types_create(
            &PostV1AgreementsTypesCreateRequest {
                code: "code".to_string(),
                name: "name".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">post_v1_agreements_types_list</a>(request: PostV1AgreementsTypesListRequest) -> Result&lt;PostV1AgreementsTypesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .agreements
        .post_v1agreements_types_list(
            &PostV1AgreementsTypesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1AgreementsTypesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1AgreementsTypesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">post_v1_agreements_agreements_create</a>(request: PostV1AgreementsAgreementsCreateRequest) -> Result&lt;PostV1AgreementsAgreementsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .agreements
        .post_v1agreements_agreements_create(
            &PostV1AgreementsAgreementsCreateRequest {
                number: "number".to_string(),
                start_date: "startDate".to_string(),
                type_id: None,
                kind: None,
                partner_id: None,
                employee_id: None,
                bank_account_id: None,
                name: None,
                end_date: None,
                auto_renew: None,
                value: None,
                billing_period: None,
                currency: None,
                status: None,
                notes: None,
                document_ref: None,
                items: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**type_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**kind:** `Option<PostV1AgreementsAgreementsCreateRequestKind>` 
    
</dd>
</dl>

<dl>
<dd>

**partner_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**employee_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**bank_account_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**number:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**auto_renew:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**value:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**billing_period:** `Option<PostV1AgreementsAgreementsCreateRequestBillingPeriod>` 
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<PostV1AgreementsAgreementsCreateRequestStatus>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**document_ref:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**items:** `Option<Vec<PostV1AgreementsAgreementsCreateRequestItemsItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">post_v1_agreements_agreements_get</a>(request: PostV1AgreementsAgreementsGetRequest) -> Result&lt;PostV1AgreementsAgreementsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .agreements
        .post_v1agreements_agreements_get(
            &PostV1AgreementsAgreementsGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">post_v1_agreements_agreements_update</a>(request: PostV1AgreementsAgreementsUpdateRequest) -> Result&lt;PostV1AgreementsAgreementsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .agreements
        .post_v1agreements_agreements_update(
            &PostV1AgreementsAgreementsUpdateRequest {
                id: "id".to_string(),
                type_id: None,
                kind: None,
                name: None,
                end_date: None,
                auto_renew: None,
                value: None,
                billing_period: None,
                status: None,
                notes: None,
                document_ref: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**type_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**kind:** `Option<PostV1AgreementsAgreementsUpdateRequestKind>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**auto_renew:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**value:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**billing_period:** `Option<Option<PostV1AgreementsAgreementsUpdateRequestBillingPeriod>>` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<PostV1AgreementsAgreementsUpdateRequestStatus>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**document_ref:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">post_v1_agreements_agreements_delete</a>(request: PostV1AgreementsAgreementsDeleteRequest) -> Result&lt;PostV1AgreementsAgreementsDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .agreements
        .post_v1agreements_agreements_delete(
            &PostV1AgreementsAgreementsDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">post_v1_agreements_agreements_list</a>(request: PostV1AgreementsAgreementsListRequest) -> Result&lt;PostV1AgreementsAgreementsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .agreements
        .post_v1agreements_agreements_list(
            &PostV1AgreementsAgreementsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1AgreementsAgreementsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1AgreementsAgreementsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">post_v1_agreements_agreements_generate_invoice</a>(request: PostV1AgreementsAgreementsGenerateInvoiceRequest) -> Result&lt;PostV1AgreementsAgreementsGenerateInvoiceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .agreements
        .post_v1agreements_agreements_generate_invoice(
            &PostV1AgreementsAgreementsGenerateInvoiceRequest {
                id: "id".to_string(),
                as_of_date: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**as_of_date:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">post_v1_agreements_agreements_billing_run</a>(request: PostV1AgreementsAgreementsBillingRunRequest) -> Result&lt;PostV1AgreementsAgreementsBillingRunResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .agreements
        .post_v1agreements_agreements_billing_run(
            &PostV1AgreementsAgreementsBillingRunRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**as_of_date:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">post_v1_agreements_insurance_policies_create</a>(request: PostV1AgreementsInsurancePoliciesCreateRequest) -> Result&lt;PostV1AgreementsInsurancePoliciesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .agreements
        .post_v1agreements_insurance_policies_create(
            &PostV1AgreementsInsurancePoliciesCreateRequest {
                policy_number: "policyNumber".to_string(),
                insured_object: "insuredObject".to_string(),
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
                insurer_partner_id: None,
                premium: None,
                currency: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**insurer_partner_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**policy_number:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**insured_object:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**premium:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">post_v1_agreements_insurance_policies_list</a>(request: PostV1AgreementsInsurancePoliciesListRequest) -> Result&lt;PostV1AgreementsInsurancePoliciesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .agreements
        .post_v1agreements_insurance_policies_list(
            &PostV1AgreementsInsurancePoliciesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1AgreementsInsurancePoliciesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1AgreementsInsurancePoliciesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">post_v1_agreements_insurance_policies_delete</a>(request: PostV1AgreementsInsurancePoliciesDeleteRequest) -> Result&lt;PostV1AgreementsInsurancePoliciesDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .agreements
        .post_v1agreements_insurance_policies_delete(
            &PostV1AgreementsInsurancePoliciesDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Inventory
<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">post_v1_inventory_settings_get</a>(request: PostV1InventorySettingsGetRequest) -> Result&lt;PostV1InventorySettingsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .inventory
        .post_v1inventory_settings_get(
            &PostV1InventorySettingsGetRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">post_v1_inventory_settings_update</a>(request: PostV1InventorySettingsUpdateRequest) -> Result&lt;PostV1InventorySettingsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .inventory
        .post_v1inventory_settings_update(
            &PostV1InventorySettingsUpdateRequest {
                negative_stock_policy:
                    PostV1InventorySettingsUpdateRequestNegativeStockPolicy::Reject,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**negative_stock_policy:** `PostV1InventorySettingsUpdateRequestNegativeStockPolicy` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">post_v1_inventory_warehouses_create</a>(request: PostV1InventoryWarehousesCreateRequest) -> Result&lt;PostV1InventoryWarehousesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .inventory
        .post_v1inventory_warehouses_create(
            &PostV1InventoryWarehousesCreateRequest {
                code: "code".to_string(),
                name: "name".to_string(),
                is_default: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**is_default:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">post_v1_inventory_warehouses_list</a>(request: PostV1InventoryWarehousesListRequest) -> Result&lt;PostV1InventoryWarehousesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .inventory
        .post_v1inventory_warehouses_list(
            &PostV1InventoryWarehousesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1InventoryWarehousesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1InventoryWarehousesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">post_v1_inventory_stock_receive</a>(request: PostV1InventoryStockReceiveRequest) -> Result&lt;PostV1InventoryStockReceiveResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .inventory
        .post_v1inventory_stock_receive(
            &PostV1InventoryStockReceiveRequest {
                warehouse_id: "warehouseId".to_string(),
                item_id: "itemId".to_string(),
                date: "date".to_string(),
                quantity: "quantity".to_string(),
                unit_cost: "unitCost".to_string(),
                lot_number: None,
                expiry_date: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**warehouse_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**item_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**quantity:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**unit_cost:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**lot_number:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**expiry_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">post_v1_inventory_stock_write_off</a>(request: PostV1InventoryStockWriteOffRequest) -> Result&lt;PostV1InventoryStockWriteOffResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .inventory
        .post_v1inventory_stock_write_off(
            &PostV1InventoryStockWriteOffRequest {
                warehouse_id: "warehouseId".to_string(),
                item_id: "itemId".to_string(),
                date: "date".to_string(),
                quantity: "quantity".to_string(),
                lot_number: None,
                expense_account_code: None,
                inventory_account_code: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**warehouse_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**item_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**quantity:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**lot_number:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**expense_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**inventory_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">post_v1_inventory_stock_transfer</a>(request: PostV1InventoryStockTransferRequest) -> Result&lt;PostV1InventoryStockTransferResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .inventory
        .post_v1inventory_stock_transfer(
            &PostV1InventoryStockTransferRequest {
                from_warehouse_id: "fromWarehouseId".to_string(),
                to_warehouse_id: "toWarehouseId".to_string(),
                item_id: "itemId".to_string(),
                date: "date".to_string(),
                quantity: "quantity".to_string(),
                lot_number: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_warehouse_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_warehouse_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**item_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**quantity:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**lot_number:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">post_v1_inventory_stock_take</a>(request: PostV1InventoryStockTakeRequest) -> Result&lt;PostV1InventoryStockTakeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .inventory
        .post_v1inventory_stock_take(
            &PostV1InventoryStockTakeRequest {
                warehouse_id: "warehouseId".to_string(),
                date: "date".to_string(),
                lines: vec![PostV1InventoryStockTakeRequestLinesItem {
                    counted_qty: "countedQty".to_string(),
                    ..Default::default()
                }],
                expense_account_code: None,
                inventory_account_code: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**warehouse_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**expense_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**inventory_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Vec<PostV1InventoryStockTakeRequestLinesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">post_v1_inventory_stock_levels</a>(request: PostV1InventoryStockLevelsRequest) -> Result&lt;PostV1InventoryStockLevelsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .inventory
        .post_v1inventory_stock_levels(
            &PostV1InventoryStockLevelsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**warehouse_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**item_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">post_v1_inventory_stock_movements_list</a>(request: PostV1InventoryStockMovementsListRequest) -> Result&lt;PostV1InventoryStockMovementsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .inventory
        .post_v1inventory_stock_movements_list(
            &PostV1InventoryStockMovementsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1InventoryStockMovementsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1InventoryStockMovementsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">post_v1_inventory_lots_list</a>(request: PostV1InventoryLotsListRequest) -> Result&lt;PostV1InventoryLotsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .inventory
        .post_v1inventory_lots_list(
            &PostV1InventoryLotsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1InventoryLotsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1InventoryLotsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">post_v1_inventory_lots_get</a>(request: PostV1InventoryLotsGetRequest) -> Result&lt;PostV1InventoryLotsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .inventory
        .post_v1inventory_lots_get(
            &PostV1InventoryLotsGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">post_v1_inventory_lots_update</a>(request: PostV1InventoryLotsUpdateRequest) -> Result&lt;PostV1InventoryLotsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .inventory
        .post_v1inventory_lots_update(
            &PostV1InventoryLotsUpdateRequest {
                id: "id".to_string(),
                expiry_date: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**expiry_date:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">post_v1_inventory_landed_costs_create</a>(request: PostV1InventoryLandedCostsCreateRequest) -> Result&lt;PostV1InventoryLandedCostsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .inventory
        .post_v1inventory_landed_costs_create(
            &PostV1InventoryLandedCostsCreateRequest {
                date: "date".to_string(),
                amount: "amount".to_string(),
                method: None,
                goods_receipt_id: None,
                movement_ids: None,
                source_invoice_id: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**amount:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**method:** `Option<PostV1InventoryLandedCostsCreateRequestMethod>` 
    
</dd>
</dl>

<dl>
<dd>

**goods_receipt_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**movement_ids:** `Option<Vec<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**source_invoice_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">post_v1_inventory_landed_costs_get</a>(request: PostV1InventoryLandedCostsGetRequest) -> Result&lt;PostV1InventoryLandedCostsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .inventory
        .post_v1inventory_landed_costs_get(
            &PostV1InventoryLandedCostsGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">post_v1_inventory_landed_costs_list</a>(request: PostV1InventoryLandedCostsListRequest) -> Result&lt;PostV1InventoryLandedCostsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .inventory
        .post_v1inventory_landed_costs_list(
            &PostV1InventoryLandedCostsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1InventoryLandedCostsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1InventoryLandedCostsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">post_v1_inventory_reorder_rules_create</a>(request: PostV1InventoryReorderRulesCreateRequest) -> Result&lt;PostV1InventoryReorderRulesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .inventory
        .post_v1inventory_reorder_rules_create(
            &PostV1InventoryReorderRulesCreateRequest {
                item_id: "itemId".to_string(),
                min_qty: "minQty".to_string(),
                warehouse_id: None,
                reorder_qty: None,
                is_active: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**item_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**warehouse_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**min_qty:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**reorder_qty:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">post_v1_inventory_reorder_rules_update</a>(request: PostV1InventoryReorderRulesUpdateRequest) -> Result&lt;PostV1InventoryReorderRulesUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .inventory
        .post_v1inventory_reorder_rules_update(
            &PostV1InventoryReorderRulesUpdateRequest {
                id: "id".to_string(),
                min_qty: None,
                reorder_qty: None,
                is_active: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**min_qty:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**reorder_qty:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">post_v1_inventory_reorder_rules_delete</a>(request: PostV1InventoryReorderRulesDeleteRequest) -> Result&lt;PostV1InventoryReorderRulesDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .inventory
        .post_v1inventory_reorder_rules_delete(
            &PostV1InventoryReorderRulesDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">post_v1_inventory_reorder_rules_list</a>(request: PostV1InventoryReorderRulesListRequest) -> Result&lt;PostV1InventoryReorderRulesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .inventory
        .post_v1inventory_reorder_rules_list(
            &PostV1InventoryReorderRulesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1InventoryReorderRulesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1InventoryReorderRulesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">post_v1_inventory_reorder_rules_check</a>(request: PostV1InventoryReorderRulesCheckRequest) -> Result&lt;PostV1InventoryReorderRulesCheckResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .inventory
        .post_v1inventory_reorder_rules_check(
            &PostV1InventoryReorderRulesCheckRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Production
<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">post_v1_production_work_centers_create</a>(request: PostV1ProductionWorkCentersCreateRequest) -> Result&lt;PostV1ProductionWorkCentersCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .production
        .post_v1production_work_centers_create(
            &PostV1ProductionWorkCentersCreateRequest {
                code: "code".to_string(),
                name: "name".to_string(),
                cost_per_hour: None,
                cost_account_code: None,
                maintenance_interval_days: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**cost_per_hour:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**cost_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**maintenance_interval_days:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">post_v1_production_work_centers_update</a>(request: PostV1ProductionWorkCentersUpdateRequest) -> Result&lt;PostV1ProductionWorkCentersUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .production
        .post_v1production_work_centers_update(
            &PostV1ProductionWorkCentersUpdateRequest {
                id: "id".to_string(),
                code: None,
                name: None,
                cost_per_hour: None,
                cost_account_code: None,
                maintenance_interval_days: None,
                is_active: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**cost_per_hour:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**cost_account_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**maintenance_interval_days:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">post_v1_production_work_centers_list</a>(request: PostV1ProductionWorkCentersListRequest) -> Result&lt;PostV1ProductionWorkCentersListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .production
        .post_v1production_work_centers_list(
            &PostV1ProductionWorkCentersListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1ProductionWorkCentersListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1ProductionWorkCentersListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">post_v1_production_routings_create</a>(request: PostV1ProductionRoutingsCreateRequest) -> Result&lt;PostV1ProductionRoutingsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .production
        .post_v1production_routings_create(
            &PostV1ProductionRoutingsCreateRequest {
                code: "code".to_string(),
                name: "name".to_string(),
                operations: vec![PostV1ProductionRoutingsCreateRequestOperationsItem {
                    sequence: 1000000,
                    name: "name".to_string(),
                    work_center_id: "workCenterId".to_string(),
                    ..Default::default()
                }],
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**operations:** `Vec<PostV1ProductionRoutingsCreateRequestOperationsItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">post_v1_production_routings_get</a>(request: PostV1ProductionRoutingsGetRequest) -> Result&lt;PostV1ProductionRoutingsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .production
        .post_v1production_routings_get(
            &PostV1ProductionRoutingsGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">post_v1_production_routings_list</a>(request: PostV1ProductionRoutingsListRequest) -> Result&lt;PostV1ProductionRoutingsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .production
        .post_v1production_routings_list(
            &PostV1ProductionRoutingsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1ProductionRoutingsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1ProductionRoutingsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">post_v1_production_maintenance_create</a>(request: PostV1ProductionMaintenanceCreateRequest) -> Result&lt;PostV1ProductionMaintenanceCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .production
        .post_v1production_maintenance_create(
            &PostV1ProductionMaintenanceCreateRequest {
                work_center_id: "workCenterId".to_string(),
                r#type: PostV1ProductionMaintenanceCreateRequestType::Preventive,
                planned_date: "plannedDate".to_string(),
                description: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**work_center_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `PostV1ProductionMaintenanceCreateRequestType` 
    
</dd>
</dl>

<dl>
<dd>

**planned_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">post_v1_production_maintenance_complete</a>(request: PostV1ProductionMaintenanceCompleteRequest) -> Result&lt;PostV1ProductionMaintenanceCompleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .production
        .post_v1production_maintenance_complete(
            &PostV1ProductionMaintenanceCompleteRequest {
                id: "id".to_string(),
                completed_date: "completedDate".to_string(),
                downtime_hours: None,
                cost: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**completed_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**downtime_hours:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**cost:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">post_v1_production_maintenance_cancel</a>(request: PostV1ProductionMaintenanceCancelRequest) -> Result&lt;PostV1ProductionMaintenanceCancelResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .production
        .post_v1production_maintenance_cancel(
            &PostV1ProductionMaintenanceCancelRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">post_v1_production_maintenance_list</a>(request: PostV1ProductionMaintenanceListRequest) -> Result&lt;PostV1ProductionMaintenanceListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .production
        .post_v1production_maintenance_list(
            &PostV1ProductionMaintenanceListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1ProductionMaintenanceListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1ProductionMaintenanceListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">post_v1_production_boms_create</a>(request: PostV1ProductionBomsCreateRequest) -> Result&lt;PostV1ProductionBomsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .production
        .post_v1production_boms_create(
            &PostV1ProductionBomsCreateRequest {
                code: "code".to_string(),
                name: "name".to_string(),
                finished_item_id: "finishedItemId".to_string(),
                lines: vec![PostV1ProductionBomsCreateRequestLinesItem {
                    component_item_id: "componentItemId".to_string(),
                    quantity: "quantity".to_string(),
                    ..Default::default()
                }],
                output_quantity: None,
                routing_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**finished_item_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**output_quantity:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**routing_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Vec<PostV1ProductionBomsCreateRequestLinesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">post_v1_production_boms_get</a>(request: PostV1ProductionBomsGetRequest) -> Result&lt;PostV1ProductionBomsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .production
        .post_v1production_boms_get(
            &PostV1ProductionBomsGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">post_v1_production_boms_list</a>(request: PostV1ProductionBomsListRequest) -> Result&lt;PostV1ProductionBomsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .production
        .post_v1production_boms_list(
            &PostV1ProductionBomsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1ProductionBomsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1ProductionBomsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">post_v1_production_orders_create</a>(request: PostV1ProductionOrdersCreateRequest) -> Result&lt;PostV1ProductionOrdersCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .production
        .post_v1production_orders_create(
            &PostV1ProductionOrdersCreateRequest {
                bom_id: "bomId".to_string(),
                warehouse_id: "warehouseId".to_string(),
                quantity: "quantity".to_string(),
                date: "date".to_string(),
                r#type: None,
                routing_id: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**type_:** `Option<PostV1ProductionOrdersCreateRequestType>` 
    
</dd>
</dl>

<dl>
<dd>

**bom_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**warehouse_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**routing_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**quantity:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">post_v1_production_orders_record_operation</a>(request: PostV1ProductionOrdersRecordOperationRequest) -> Result&lt;PostV1ProductionOrdersRecordOperationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .production
        .post_v1production_orders_record_operation(
            &PostV1ProductionOrdersRecordOperationRequest {
                id: "id".to_string(),
                actual_minutes: "actualMinutes".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**actual_minutes:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">post_v1_production_quality_checks_add</a>(request: PostV1ProductionQualityChecksAddRequest) -> Result&lt;PostV1ProductionQualityChecksAddResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .production
        .post_v1production_quality_checks_add(
            &PostV1ProductionQualityChecksAddRequest {
                order_id: "orderId".to_string(),
                name: "name".to_string(),
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**order_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">post_v1_production_quality_checks_record</a>(request: PostV1ProductionQualityChecksRecordRequest) -> Result&lt;PostV1ProductionQualityChecksRecordResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .production
        .post_v1production_quality_checks_record(
            &PostV1ProductionQualityChecksRecordRequest {
                id: "id".to_string(),
                result: PostV1ProductionQualityChecksRecordRequestResult::Passed,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**result:** `PostV1ProductionQualityChecksRecordRequestResult` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">post_v1_production_quality_checks_list</a>(request: PostV1ProductionQualityChecksListRequest) -> Result&lt;PostV1ProductionQualityChecksListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .production
        .post_v1production_quality_checks_list(
            &PostV1ProductionQualityChecksListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1ProductionQualityChecksListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1ProductionQualityChecksListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">post_v1_production_orders_complete</a>(request: PostV1ProductionOrdersCompleteRequest) -> Result&lt;PostV1ProductionOrdersCompleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .production
        .post_v1production_orders_complete(
            &PostV1ProductionOrdersCompleteRequest {
                id: "id".to_string(),
                scrapped_quantity: None,
                components_account_code: None,
                finished_account_code: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**scrapped_quantity:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**components_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**finished_account_code:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">post_v1_production_orders_get</a>(request: PostV1ProductionOrdersGetRequest) -> Result&lt;PostV1ProductionOrdersGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .production
        .post_v1production_orders_get(
            &PostV1ProductionOrdersGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">post_v1_production_orders_list</a>(request: PostV1ProductionOrdersListRequest) -> Result&lt;PostV1ProductionOrdersListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .production
        .post_v1production_orders_list(
            &PostV1ProductionOrdersListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1ProductionOrdersListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1ProductionOrdersListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Ecommerce
<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">post_v1_ecommerce_orders_create</a>(request: PostV1EcommerceOrdersCreateRequest) -> Result&lt;PostV1EcommerceOrdersCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .post_v1ecommerce_orders_create(
            &PostV1EcommerceOrdersCreateRequest {
                lines: vec![PostV1EcommerceOrdersCreateRequestLinesItem {
                    description: "description".to_string(),
                    quantity: "quantity".to_string(),
                    unit_price_excl_vat: "unitPriceExclVat".to_string(),
                    ..Default::default()
                }],
                channel: None,
                external_ref: None,
                partner_id: None,
                partner: None,
                warehouse_id: None,
                currency: None,
                ship_to_country_code: None,
                marketplace: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**channel:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**external_ref:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**partner_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**partner:** `Option<PostV1EcommerceOrdersCreateRequestPartner>` 
    
</dd>
</dl>

<dl>
<dd>

**warehouse_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**ship_to_country_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**marketplace:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Vec<PostV1EcommerceOrdersCreateRequestLinesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">post_v1_ecommerce_orders_get</a>(request: PostV1EcommerceOrdersGetRequest) -> Result&lt;PostV1EcommerceOrdersGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .post_v1ecommerce_orders_get(
            &PostV1EcommerceOrdersGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">post_v1_ecommerce_orders_list</a>(request: PostV1EcommerceOrdersListRequest) -> Result&lt;PostV1EcommerceOrdersListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .post_v1ecommerce_orders_list(
            &PostV1EcommerceOrdersListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1EcommerceOrdersListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1EcommerceOrdersListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">post_v1_ecommerce_orders_reserve</a>(request: PostV1EcommerceOrdersReserveRequest) -> Result&lt;PostV1EcommerceOrdersReserveResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .post_v1ecommerce_orders_reserve(
            &PostV1EcommerceOrdersReserveRequest {
                id: "id".to_string(),
                warehouse_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**warehouse_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">post_v1_ecommerce_orders_fulfill</a>(request: PostV1EcommerceOrdersFulfillRequest) -> Result&lt;PostV1EcommerceOrdersFulfillResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .post_v1ecommerce_orders_fulfill(
            &PostV1EcommerceOrdersFulfillRequest {
                id: "id".to_string(),
                date: None,
                cogs_account_code: None,
                inventory_account_code: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**cogs_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**inventory_account_code:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">post_v1_ecommerce_orders_cancel</a>(request: PostV1EcommerceOrdersCancelRequest) -> Result&lt;PostV1EcommerceOrdersCancelResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .post_v1ecommerce_orders_cancel(
            &PostV1EcommerceOrdersCancelRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">post_v1_ecommerce_products_list</a>(request: PostV1EcommerceProductsListRequest) -> Result&lt;PostV1EcommerceProductsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .post_v1ecommerce_products_list(
            &PostV1EcommerceProductsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**warehouse_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**price_list_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**updated_since:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">post_v1_ecommerce_stock_list</a>(request: PostV1EcommerceStockListRequest) -> Result&lt;PostV1EcommerceStockListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .post_v1ecommerce_stock_list(
            &PostV1EcommerceStockListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**warehouse_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Cash
<details><summary><code>client.cash.<a href="/src/api/resources/cash/client.rs">post_v1_cash_orders_create</a>(request: PostV1CashOrdersCreateRequest) -> Result&lt;PostV1CashOrdersCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .cash
        .post_v1cash_orders_create(
            &PostV1CashOrdersCreateRequest {
                r#type: PostV1CashOrdersCreateRequestType::Receipt,
                date: "date".to_string(),
                amount: "amount".to_string(),
                purpose: "purpose".to_string(),
                counter_account_code: "counterAccountCode".to_string(),
                cash_account_code: None,
                series: None,
                partner_id: None,
                employee_id: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**type_:** `PostV1CashOrdersCreateRequestType` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**amount:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**purpose:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**counter_account_code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**cash_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**series:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**partner_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**employee_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.cash.<a href="/src/api/resources/cash/client.rs">post_v1_cash_orders_get</a>(request: PostV1CashOrdersGetRequest) -> Result&lt;PostV1CashOrdersGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .cash
        .post_v1cash_orders_get(
            &PostV1CashOrdersGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.cash.<a href="/src/api/resources/cash/client.rs">post_v1_cash_orders_list</a>(request: PostV1CashOrdersListRequest) -> Result&lt;PostV1CashOrdersListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .cash
        .post_v1cash_orders_list(
            &PostV1CashOrdersListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1CashOrdersListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1CashOrdersListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.cash.<a href="/src/api/resources/cash/client.rs">post_v1_cash_balance</a>(request: PostV1CashBalanceRequest) -> Result&lt;PostV1CashBalanceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .cash
        .post_v1cash_balance(
            &PostV1CashBalanceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**cash_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**as_of:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.cash.<a href="/src/api/resources/cash/client.rs">post_v1_cash_advance_holders_balances</a>(request: PostV1CashAdvanceHoldersBalancesRequest) -> Result&lt;PostV1CashAdvanceHoldersBalancesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .cash
        .post_v1cash_advance_holders_balances(
            &PostV1CashAdvanceHoldersBalancesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Projects
<details><summary><code>client.projects.<a href="/src/api/resources/projects/client.rs">post_v1_projects_create</a>(request: PostV1ProjectsCreateRequest) -> Result&lt;PostV1ProjectsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .projects
        .post_v1projects_create(
            &PostV1ProjectsCreateRequest {
                code: "code".to_string(),
                name: "name".to_string(),
                partner_id: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**partner_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.projects.<a href="/src/api/resources/projects/client.rs">post_v1_projects_update</a>(request: PostV1ProjectsUpdateRequest) -> Result&lt;PostV1ProjectsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .projects
        .post_v1projects_update(
            &PostV1ProjectsUpdateRequest {
                id: "id".to_string(),
                name: None,
                partner_id: None,
                status: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**partner_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<PostV1ProjectsUpdateRequestStatus>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.projects.<a href="/src/api/resources/projects/client.rs">post_v1_projects_get</a>(request: PostV1ProjectsGetRequest) -> Result&lt;PostV1ProjectsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .projects
        .post_v1projects_get(
            &PostV1ProjectsGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.projects.<a href="/src/api/resources/projects/client.rs">post_v1_projects_list</a>(request: PostV1ProjectsListRequest) -> Result&lt;PostV1ProjectsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .projects
        .post_v1projects_list(
            &PostV1ProjectsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1ProjectsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1ProjectsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.projects.<a href="/src/api/resources/projects/client.rs">post_v1_projects_time_entries_create</a>(request: PostV1ProjectsTimeEntriesCreateRequest) -> Result&lt;PostV1ProjectsTimeEntriesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .projects
        .post_v1projects_time_entries_create(
            &PostV1ProjectsTimeEntriesCreateRequest {
                project_id: "projectId".to_string(),
                date: "date".to_string(),
                hours: "hours".to_string(),
                employee_id: None,
                description: None,
                billable: None,
                hourly_rate: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**employee_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**hours:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**billable:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**hourly_rate:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.projects.<a href="/src/api/resources/projects/client.rs">post_v1_projects_time_entries_update</a>(request: PostV1ProjectsTimeEntriesUpdateRequest) -> Result&lt;PostV1ProjectsTimeEntriesUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .projects
        .post_v1projects_time_entries_update(
            &PostV1ProjectsTimeEntriesUpdateRequest {
                id: "id".to_string(),
                date: None,
                hours: None,
                description: None,
                billable: None,
                hourly_rate: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**hours:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**billable:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**hourly_rate:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.projects.<a href="/src/api/resources/projects/client.rs">post_v1_projects_time_entries_delete</a>(request: PostV1ProjectsTimeEntriesDeleteRequest) -> Result&lt;PostV1ProjectsTimeEntriesDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .projects
        .post_v1projects_time_entries_delete(
            &PostV1ProjectsTimeEntriesDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.projects.<a href="/src/api/resources/projects/client.rs">post_v1_projects_time_entries_list</a>(request: PostV1ProjectsTimeEntriesListRequest) -> Result&lt;PostV1ProjectsTimeEntriesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .projects
        .post_v1projects_time_entries_list(
            &PostV1ProjectsTimeEntriesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1ProjectsTimeEntriesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1ProjectsTimeEntriesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.projects.<a href="/src/api/resources/projects/client.rs">post_v1_projects_time_entries_bill</a>(request: PostV1ProjectsTimeEntriesBillRequest) -> Result&lt;PostV1ProjectsTimeEntriesBillResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .projects
        .post_v1projects_time_entries_bill(
            &PostV1ProjectsTimeEntriesBillRequest {
                project_id: "projectId".to_string(),
                partner_id: None,
                date_from: None,
                date_to: None,
                item_id: None,
                hourly_rate: None,
                vat_rate_percent: None,
                vat_classifier_code: None,
                issue_date: None,
                due_date: None,
                group_by: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**partner_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**date_from:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**date_to:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**item_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**hourly_rate:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_rate_percent:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_classifier_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**issue_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**due_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**group_by:** `Option<PostV1ProjectsTimeEntriesBillRequestGroupBy>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.projects.<a href="/src/api/resources/projects/client.rs">post_v1_projects_report</a>(request: PostV1ProjectsReportRequest) -> Result&lt;PostV1ProjectsReportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .projects
        .post_v1projects_report(
            &PostV1ProjectsReportRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**date_from:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**date_to:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Transport
<details><summary><code>client.transport.<a href="/src/api/resources/transport/client.rs">post_v1_transport_waybills_create</a>(request: PostV1TransportWaybillsCreateRequest) -> Result&lt;PostV1TransportWaybillsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .transport
        .post_v1transport_waybills_create(
            &PostV1TransportWaybillsCreateRequest {
                consignee_partner_id: "consigneePartnerId".to_string(),
                dispatch_at: DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z").unwrap(),
                load_address: "loadAddress".to_string(),
                unload_address: "unloadAddress".to_string(),
                transporter_partner_id: None,
                document_date: None,
                estimated_arrival_at: None,
                vehicle_plate: None,
                trailer_plate: None,
                driver_name: None,
                driver_surname: None,
                load_warehouse_id: None,
                value_eur: None,
                sale_invoice_id: None,
                notes: None,
                series: None,
                lines: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**consignee_partner_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**transporter_partner_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**document_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**dispatch_at:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**estimated_arrival_at:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**vehicle_plate:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**trailer_plate:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**driver_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**driver_surname:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**load_warehouse_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**load_address:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**unload_address:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**value_eur:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sale_invoice_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**series:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Option<Vec<PostV1TransportWaybillsCreateRequestLinesItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transport.<a href="/src/api/resources/transport/client.rs">post_v1_transport_waybills_update</a>(request: PostV1TransportWaybillsUpdateRequest) -> Result&lt;PostV1TransportWaybillsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .transport
        .post_v1transport_waybills_update(
            &PostV1TransportWaybillsUpdateRequest {
                id: "id".to_string(),
                consignee_partner_id: None,
                transporter_partner_id: None,
                document_date: None,
                dispatch_at: None,
                estimated_arrival_at: None,
                vehicle_plate: None,
                trailer_plate: None,
                driver_name: None,
                driver_surname: None,
                load_warehouse_id: None,
                load_address: None,
                unload_address: None,
                value_eur: None,
                sale_invoice_id: None,
                notes: None,
                series: None,
                lines: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**consignee_partner_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**transporter_partner_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**document_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**dispatch_at:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**estimated_arrival_at:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**vehicle_plate:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**trailer_plate:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**driver_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**driver_surname:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**load_warehouse_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**load_address:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**unload_address:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**value_eur:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**sale_invoice_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**series:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Option<Vec<PostV1TransportWaybillsUpdateRequestLinesItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transport.<a href="/src/api/resources/transport/client.rs">post_v1_transport_waybills_issue</a>(request: PostV1TransportWaybillsIssueRequest) -> Result&lt;PostV1TransportWaybillsIssueResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .transport
        .post_v1transport_waybills_issue(
            &PostV1TransportWaybillsIssueRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transport.<a href="/src/api/resources/transport/client.rs">post_v1_transport_waybills_cancel</a>(request: PostV1TransportWaybillsCancelRequest) -> Result&lt;PostV1TransportWaybillsCancelResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .transport
        .post_v1transport_waybills_cancel(
            &PostV1TransportWaybillsCancelRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transport.<a href="/src/api/resources/transport/client.rs">post_v1_transport_waybills_get</a>(request: PostV1TransportWaybillsGetRequest) -> Result&lt;PostV1TransportWaybillsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .transport
        .post_v1transport_waybills_get(
            &PostV1TransportWaybillsGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transport.<a href="/src/api/resources/transport/client.rs">post_v1_transport_waybills_list</a>(request: PostV1TransportWaybillsListRequest) -> Result&lt;PostV1TransportWaybillsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .transport
        .post_v1transport_waybills_list(
            &PostV1TransportWaybillsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1TransportWaybillsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1TransportWaybillsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Pos
<details><summary><code>client.pos.<a href="/src/api/resources/pos/client.rs">post_v1_pos_devices_create</a>(request: PostV1PosDevicesCreateRequest) -> Result&lt;PostV1PosDevicesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .pos
        .post_v1pos_devices_create(
            &PostV1PosDevicesCreateRequest {
                name: "name".to_string(),
                serial_number: "serialNumber".to_string(),
                model: None,
                registration_number: None,
                address: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**serial_number:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**model:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**registration_number:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**address:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pos.<a href="/src/api/resources/pos/client.rs">post_v1_pos_devices_update</a>(request: PostV1PosDevicesUpdateRequest) -> Result&lt;PostV1PosDevicesUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .pos
        .post_v1pos_devices_update(
            &PostV1PosDevicesUpdateRequest {
                id: "id".to_string(),
                is_active: None,
                name: None,
                serial_number: None,
                model: None,
                registration_number: None,
                address: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**serial_number:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**model:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**registration_number:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**address:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pos.<a href="/src/api/resources/pos/client.rs">post_v1_pos_devices_list</a>(request: PostV1PosDevicesListRequest) -> Result&lt;PostV1PosDevicesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .pos
        .post_v1pos_devices_list(
            &PostV1PosDevicesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1PosDevicesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1PosDevicesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pos.<a href="/src/api/resources/pos/client.rs">post_v1_pos_reports_create</a>(request: PostV1PosReportsCreateRequest) -> Result&lt;PostV1PosReportsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .pos
        .post_v1pos_reports_create(
            &PostV1PosReportsCreateRequest {
                report_number: "reportNumber".to_string(),
                date: "date".to_string(),
                vat_lines: vec![PostV1PosReportsCreateRequestVatLinesItem {
                    vat_rate_percent: "vatRatePercent".to_string(),
                    net_amount: "netAmount".to_string(),
                    vat_amount: "vatAmount".to_string(),
                    ..Default::default()
                }],
                device_id: None,
                warehouse_id: None,
                cash_amount: None,
                card_amount: None,
                item_lines: None,
                cash_account_code: None,
                card_account_code: None,
                revenue_account_code: None,
                vat_account_code: None,
                cogs_account_code: None,
                inventory_account_code: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**report_number:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**device_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**warehouse_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_lines:** `Vec<PostV1PosReportsCreateRequestVatLinesItem>` 
    
</dd>
</dl>

<dl>
<dd>

**cash_amount:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**card_amount:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**item_lines:** `Option<Vec<PostV1PosReportsCreateRequestItemLinesItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**cash_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**card_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**revenue_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**cogs_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**inventory_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pos.<a href="/src/api/resources/pos/client.rs">post_v1_pos_reports_get</a>(request: PostV1PosReportsGetRequest) -> Result&lt;PostV1PosReportsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .pos
        .post_v1pos_reports_get(
            &PostV1PosReportsGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pos.<a href="/src/api/resources/pos/client.rs">post_v1_pos_reports_list</a>(request: PostV1PosReportsListRequest) -> Result&lt;PostV1PosReportsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .pos
        .post_v1pos_reports_list(
            &PostV1PosReportsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1PosReportsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1PosReportsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Calendar
<details><summary><code>client.calendar.<a href="/src/api/resources/calendar/client.rs">post_v1_calendar_list</a>(request: PostV1CalendarListRequest) -> Result&lt;PostV1CalendarListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .calendar
        .post_v1calendar_list(
            &PostV1CalendarListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**to:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**include_done:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.calendar.<a href="/src/api/resources/calendar/client.rs">post_v1_calendar_get</a>(request: PostV1CalendarGetRequest) -> Result&lt;PostV1CalendarGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .calendar
        .post_v1calendar_get(
            &PostV1CalendarGetRequest {
                key: "key".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**key:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.calendar.<a href="/src/api/resources/calendar/client.rs">generate_the_filing_for_a_deadline_and_send_it_to_the_administration</a>(request: PostV1CalendarSubmitRequest) -> Result&lt;PostV1CalendarSubmitResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .calendar
        .generate_the_filing_for_a_deadline_and_send_it_to_the_administration(
            &PostV1CalendarSubmitRequest {
                key: "key".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**key:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.calendar.<a href="/src/api/resources/calendar/client.rs">generate_the_file_of_a_deadline_for_the_company_to_send_itself</a>(request: PostV1CalendarDownloadRequest) -> Result&lt;PostV1CalendarDownloadResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Builds the file of a deadline whose format Nordlet produces but whose administration takes it only through the company's own account or program. Nothing is sent and no filing is recorded.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .calendar
        .generate_the_file_of_a_deadline_for_the_company_to_send_itself(
            &PostV1CalendarDownloadRequest {
                key: "key".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**key:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.calendar.<a href="/src/api/resources/calendar/client.rs">post_v1_calendar_create</a>(request: PostV1CalendarCreateRequest) -> Result&lt;PostV1CalendarCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .calendar
        .post_v1calendar_create(
            &PostV1CalendarCreateRequest {
                title: "title".to_string(),
                due_date: "dueDate".to_string(),
                notes: None,
                done: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**title:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**due_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**done:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.calendar.<a href="/src/api/resources/calendar/client.rs">post_v1_calendar_update</a>(request: PostV1CalendarUpdateRequest) -> Result&lt;PostV1CalendarUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .calendar
        .post_v1calendar_update(
            &PostV1CalendarUpdateRequest {
                key: "key".to_string(),
                title: None,
                due_date: None,
                notes: None,
                done: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**key:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**due_date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**done:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.calendar.<a href="/src/api/resources/calendar/client.rs">post_v1_calendar_delete</a>(request: PostV1CalendarDeleteRequest) -> Result&lt;PostV1CalendarDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .calendar
        .post_v1calendar_delete(
            &PostV1CalendarDeleteRequest {
                key: "key".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**key:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Audit
<details><summary><code>client.audit.<a href="/src/api/resources/audit/client.rs">post_v1_audit_list</a>(request: PostV1AuditListRequest) -> Result&lt;PostV1AuditListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .audit
        .post_v1audit_list(
            &PostV1AuditListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1AuditListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1AuditListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Webhooks
<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">post_v1_webhooks_subscriptions_create</a>(request: PostV1WebhooksSubscriptionsCreateRequest) -> Result&lt;PostV1WebhooksSubscriptionsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .post_v1webhooks_subscriptions_create(
            &PostV1WebhooksSubscriptionsCreateRequest {
                url: "url".to_string(),
                events: vec!["events".to_string()],
                secret: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**url:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**events:** `Vec<String>` 
    
</dd>
</dl>

<dl>
<dd>

**secret:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">post_v1_webhooks_subscriptions_list</a>(request: PostV1WebhooksSubscriptionsListRequest) -> Result&lt;PostV1WebhooksSubscriptionsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .post_v1webhooks_subscriptions_list(
            &PostV1WebhooksSubscriptionsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1WebhooksSubscriptionsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1WebhooksSubscriptionsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">post_v1_webhooks_subscriptions_update</a>(request: PostV1WebhooksSubscriptionsUpdateRequest) -> Result&lt;PostV1WebhooksSubscriptionsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .post_v1webhooks_subscriptions_update(
            &PostV1WebhooksSubscriptionsUpdateRequest {
                id: "id".to_string(),
                url: None,
                events: None,
                is_active: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**events:** `Option<Vec<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">post_v1_webhooks_subscriptions_delete</a>(request: PostV1WebhooksSubscriptionsDeleteRequest) -> Result&lt;PostV1WebhooksSubscriptionsDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .post_v1webhooks_subscriptions_delete(
            &PostV1WebhooksSubscriptionsDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">post_v1_webhooks_deliveries_list</a>(request: PostV1WebhooksDeliveriesListRequest) -> Result&lt;PostV1WebhooksDeliveriesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .post_v1webhooks_deliveries_list(
            &PostV1WebhooksDeliveriesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1WebhooksDeliveriesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1WebhooksDeliveriesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">post_v1_webhooks_deliveries_redeliver</a>(request: PostV1WebhooksDeliveriesRedeliverRequest) -> Result&lt;PostV1WebhooksDeliveriesRedeliverResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .post_v1webhooks_deliveries_redeliver(
            &PostV1WebhooksDeliveriesRedeliverRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Bank
<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_accounts_create</a>(request: PostV1BankAccountsCreateRequest) -> Result&lt;PostV1BankAccountsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_accounts_create(
            &PostV1BankAccountsCreateRequest {
                name: "name".to_string(),
                iban: None,
                currency: None,
                account_code: None,
                document_ref: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**iban:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**document_ref:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_accounts_list</a>(request: PostV1BankAccountsListRequest) -> Result&lt;PostV1BankAccountsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_accounts_list(
            &PostV1BankAccountsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1BankAccountsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1BankAccountsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_accounts_update</a>(request: PostV1BankAccountsUpdateRequest) -> Result&lt;PostV1BankAccountsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_accounts_update(
            &PostV1BankAccountsUpdateRequest {
                id: "id".to_string(),
                name: None,
                iban: None,
                account_code: None,
                is_active: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**iban:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_transactions_import</a>(request: PostV1BankTransactionsImportRequest) -> Result&lt;PostV1BankTransactionsImportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_transactions_import(
            &PostV1BankTransactionsImportRequest {
                bank_account_id: "bankAccountId".to_string(),
                transactions: vec![PostV1BankTransactionsImportRequestTransactionsItem {
                    date: "date".to_string(),
                    amount: "amount".to_string(),
                    ..Default::default()
                }],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**bank_account_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**transactions:** `Vec<PostV1BankTransactionsImportRequestTransactionsItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_statements_import</a>(request: PostV1BankStatementsImportRequest) -> Result&lt;PostV1BankStatementsImportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_statements_import(
            &PostV1BankStatementsImportRequest {
                bank_account_id: "bankAccountId".to_string(),
                content: "content".to_string(),
                template_id: None,
                format: None,
                transfers_csv: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**bank_account_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**template_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**format:** `Option<PostV1BankStatementsImportRequestFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**content:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**transfers_csv:** `Option<String>` — Stripe transfers export (plain CSV or base64) used to post lender payouts and commissions
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_transactions_list</a>(request: PostV1BankTransactionsListRequest) -> Result&lt;PostV1BankTransactionsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_transactions_list(
            &PostV1BankTransactionsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1BankTransactionsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1BankTransactionsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_transactions_match</a>(request: PostV1BankTransactionsMatchRequest) -> Result&lt;PostV1BankTransactionsMatchResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_transactions_match(
            &PostV1BankTransactionsMatchRequest {
                transaction_id: "transactionId".to_string(),
                document_type: PostV1BankTransactionsMatchRequestDocumentType::SaleInvoice,
                document_id: "documentId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**transaction_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**document_type:** `PostV1BankTransactionsMatchRequestDocumentType` 
    
</dd>
</dl>

<dl>
<dd>

**document_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_transactions_record</a>(request: PostV1BankTransactionsRecordRequest) -> Result&lt;PostV1BankTransactionsRecordResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_transactions_record(
            &PostV1BankTransactionsRecordRequest {
                bank_account_id: "bankAccountId".to_string(),
                date: "date".to_string(),
                amount: "amount".to_string(),
                document_type: PostV1BankTransactionsRecordRequestDocumentType::SaleInvoice,
                document_id: "documentId".to_string(),
                description: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**bank_account_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**amount:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**document_type:** `PostV1BankTransactionsRecordRequestDocumentType` 
    
</dd>
</dl>

<dl>
<dd>

**document_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_payments_export</a>(request: PostV1BankPaymentsExportRequest) -> Result&lt;PostV1BankPaymentsExportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_payments_export(
            &PostV1BankPaymentsExportRequest {
                bank_account_id: "bankAccountId".to_string(),
                purchase_invoice_ids: vec!["purchaseInvoiceIds".to_string()],
                execution_date: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**bank_account_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**purchase_invoice_ids:** `Vec<String>` 
    
</dd>
</dl>

<dl>
<dd>

**execution_date:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">create_a_bank_import_template_fields_default_to_the_types_standard_field_list</a>(request: PostV1BankImportTemplatesCreateRequest) -> Result&lt;PostV1BankImportTemplatesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .create_a_bank_import_template_fields_default_to_the_types_standard_field_list(
            &PostV1BankImportTemplatesCreateRequest {
                name: "name".to_string(),
                r#type: PostV1BankImportTemplatesCreateRequestType::Stripe,
                fields: None,
                meta_fields: None,
                invoice_meta_field: None,
                invoice_vat_rate_percent: None,
                company_meta_field: None,
                invoice_item_id: None,
                advance_invoices: None,
                authorization_operation_type_id: None,
                payout_operation_type_id: None,
                commission_operation_type_id: None,
                lender_meta_field: None,
                partial_refund_label: None,
                full_refund_label: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `PostV1BankImportTemplatesCreateRequestType` 
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<Vec<PostV1BankImportTemplatesCreateRequestFieldsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**meta_fields:** `Option<Vec<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**invoice_meta_field:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**invoice_vat_rate_percent:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**company_meta_field:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**invoice_item_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**advance_invoices:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**authorization_operation_type_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**payout_operation_type_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**commission_operation_type_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**lender_meta_field:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**partial_refund_label:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**full_refund_label:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_import_templates_update</a>(request: PostV1BankImportTemplatesUpdateRequest) -> Result&lt;PostV1BankImportTemplatesUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_import_templates_update(
            &PostV1BankImportTemplatesUpdateRequest {
                id: "id".to_string(),
                name: None,
                r#type: None,
                fields: None,
                meta_fields: None,
                invoice_meta_field: None,
                invoice_vat_rate_percent: None,
                company_meta_field: None,
                invoice_item_id: None,
                advance_invoices: None,
                authorization_operation_type_id: None,
                payout_operation_type_id: None,
                commission_operation_type_id: None,
                lender_meta_field: None,
                partial_refund_label: None,
                full_refund_label: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<PostV1BankImportTemplatesUpdateRequestType>` 
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<Vec<PostV1BankImportTemplatesUpdateRequestFieldsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**meta_fields:** `Option<Vec<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**invoice_meta_field:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**invoice_vat_rate_percent:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**company_meta_field:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**invoice_item_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**advance_invoices:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**authorization_operation_type_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**payout_operation_type_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**commission_operation_type_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**lender_meta_field:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**partial_refund_label:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**full_refund_label:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_import_templates_delete</a>(request: PostV1BankImportTemplatesDeleteRequest) -> Result&lt;PostV1BankImportTemplatesDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_import_templates_delete(
            &PostV1BankImportTemplatesDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_import_templates_get</a>(request: PostV1BankImportTemplatesGetRequest) -> Result&lt;PostV1BankImportTemplatesGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_import_templates_get(
            &PostV1BankImportTemplatesGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_import_templates_list</a>(request: PostV1BankImportTemplatesListRequest) -> Result&lt;PostV1BankImportTemplatesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_import_templates_list(
            &PostV1BankImportTemplatesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1BankImportTemplatesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1BankImportTemplatesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_match_rules_create</a>(request: PostV1BankMatchRulesCreateRequest) -> Result&lt;PostV1BankMatchRulesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_match_rules_create(
            &PostV1BankMatchRulesCreateRequest {
                name: "name".to_string(),
                pattern: "pattern".to_string(),
                provider: None,
                payout_id_prefix: None,
                bank_account_id: None,
                date_window_days: None,
                is_active: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**provider:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**pattern:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**payout_id_prefix:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**bank_account_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**date_window_days:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_match_rules_update</a>(request: PostV1BankMatchRulesUpdateRequest) -> Result&lt;PostV1BankMatchRulesUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_match_rules_update(
            &PostV1BankMatchRulesUpdateRequest {
                id: "id".to_string(),
                name: None,
                provider: None,
                pattern: None,
                payout_id_prefix: None,
                bank_account_id: None,
                date_window_days: None,
                is_active: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**provider:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**pattern:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**payout_id_prefix:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**bank_account_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**date_window_days:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_match_rules_delete</a>(request: PostV1BankMatchRulesDeleteRequest) -> Result&lt;PostV1BankMatchRulesDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_match_rules_delete(
            &PostV1BankMatchRulesDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_match_rules_list</a>(request: PostV1BankMatchRulesListRequest) -> Result&lt;PostV1BankMatchRulesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_match_rules_list(
            &PostV1BankMatchRulesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_mandates_create</a>(request: PostV1BankMandatesCreateRequest) -> Result&lt;PostV1BankMandatesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_mandates_create(
            &PostV1BankMandatesCreateRequest {
                partner_id: "partnerId".to_string(),
                iban: "iban".to_string(),
                signature_date: "signatureDate".to_string(),
                bic: None,
                scheme: None,
                sequence_type: None,
                reference: None,
                debtor_name: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**partner_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**iban:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**bic:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**scheme:** `Option<PostV1BankMandatesCreateRequestScheme>` 
    
</dd>
</dl>

<dl>
<dd>

**sequence_type:** `Option<PostV1BankMandatesCreateRequestSequenceType>` 
    
</dd>
</dl>

<dl>
<dd>

**signature_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**reference:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**debtor_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_mandates_update</a>(request: PostV1BankMandatesUpdateRequest) -> Result&lt;PostV1BankMandatesUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_mandates_update(
            &PostV1BankMandatesUpdateRequest {
                id: "id".to_string(),
                bic: None,
                debtor_name: None,
                notes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**bic:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**debtor_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_mandates_cancel</a>(request: PostV1BankMandatesCancelRequest) -> Result&lt;PostV1BankMandatesCancelResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_mandates_cancel(
            &PostV1BankMandatesCancelRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_mandates_get</a>(request: PostV1BankMandatesGetRequest) -> Result&lt;PostV1BankMandatesGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_mandates_get(
            &PostV1BankMandatesGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_mandates_list</a>(request: PostV1BankMandatesListRequest) -> Result&lt;PostV1BankMandatesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_mandates_list(
            &PostV1BankMandatesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1BankMandatesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1BankMandatesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_direct_debits_export</a>(request: PostV1BankDirectDebitsExportRequest) -> Result&lt;PostV1BankDirectDebitsExportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_direct_debits_export(
            &PostV1BankDirectDebitsExportRequest {
                bank_account_id: "bankAccountId".to_string(),
                sale_invoice_ids: vec!["saleInvoiceIds".to_string()],
                collection_date: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**bank_account_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**sale_invoice_ids:** `Vec<String>` 
    
</dd>
</dl>

<dl>
<dd>

**collection_date:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_transactions_suggest_matches</a>(request: PostV1BankTransactionsSuggestMatchesRequest) -> Result&lt;PostV1BankTransactionsSuggestMatchesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_transactions_suggest_matches(
            &PostV1BankTransactionsSuggestMatchesRequest {
                transaction_id: "transactionId".to_string(),
                limit: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**transaction_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_settlements_import</a>(request: PostV1BankSettlementsImportRequest) -> Result&lt;PostV1BankSettlementsImportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_settlements_import(
            &PostV1BankSettlementsImportRequest {
                bank_account_id: "bankAccountId".to_string(),
                content: "content".to_string(),
                provider: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**bank_account_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**provider:** `Option<PostV1BankSettlementsImportRequestProvider>` 
    
</dd>
</dl>

<dl>
<dd>

**content:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_settlements_list</a>(request: PostV1BankSettlementsListRequest) -> Result&lt;PostV1BankSettlementsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_settlements_list(
            &PostV1BankSettlementsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1BankSettlementsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1BankSettlementsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_settlements_get</a>(request: PostV1BankSettlementsGetRequest) -> Result&lt;PostV1BankSettlementsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_settlements_get(
            &PostV1BankSettlementsGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_settlements_match</a>(request: PostV1BankSettlementsMatchRequest) -> Result&lt;PostV1BankSettlementsMatchResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_settlements_match(
            &PostV1BankSettlementsMatchRequest {
                line_id: "lineId".to_string(),
                invoice_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**line_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**invoice_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">set_what_the_marketplace_keeps_from_one_settlement_line_as_a_rate_or_as_an_amount</a>(request: PostV1BankSettlementsCommissionRequest) -> Result&lt;PostV1BankSettlementsCommissionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

A line with its own rate or amount is split with that value when the batch is posted. A line without one falls back to the commissionPercent given to the posting call, and without that the amount goes to the suspense account. Send both fields as null to clear the line back to the fallback.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .set_what_the_marketplace_keeps_from_one_settlement_line_as_a_rate_or_as_an_amount(
            &PostV1BankSettlementsCommissionRequest {
                line_id: "lineId".to_string(),
                commission_percent: None,
                commission_amount: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**line_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**commission_percent:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**commission_amount:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_settlements_link</a>(request: PostV1BankSettlementsLinkRequest) -> Result&lt;PostV1BankSettlementsLinkResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Attach the incoming bank-statement line that carries this payout to the settlement batch.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_settlements_link(
            &PostV1BankSettlementsLinkRequest {
                id: "id".to_string(),
                bank_transaction_id: "bankTransactionId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**bank_transaction_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_settlements_unlink</a>(request: PostV1BankSettlementsUnlinkRequest) -> Result&lt;PostV1BankSettlementsUnlinkResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Detach the bank-statement line from the settlement batch and return the line to unmatched.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_settlements_unlink(
            &PostV1BankSettlementsUnlinkRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_settlements_post</a>(request: PostV1BankSettlementsPostRequest) -> Result&lt;PostV1BankSettlementsPostResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_settlements_post(
            &PostV1BankSettlementsPostRequest {
                id: "id".to_string(),
                date: None,
                commission_percent: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**commission_percent:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">list_the_psd2_banks_asps_ps_available_to_connect</a>(request: PostV1BankFeedsBanksListRequest) -> Result&lt;PostV1BankFeedsBanksListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .list_the_psd2banks_asps_ps_available_to_connect(
            &PostV1BankFeedsBanksListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**country:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">begin_bank_authorization_redirect_the_user_to_the_returned_url</a>(request: PostV1BankFeedsConnectionsStartRequest) -> Result&lt;PostV1BankFeedsConnectionsStartResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .begin_bank_authorization_redirect_the_user_to_the_returned_url(
            &PostV1BankFeedsConnectionsStartRequest {
                aspsp_name: "aspspName".to_string(),
                aspsp_country: "aspspCountry".to_string(),
                psu_type: None,
                redirect_url: None,
                valid_for_days: None,
                language: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**aspsp_name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**aspsp_country:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**psu_type:** `Option<PostV1BankFeedsConnectionsStartRequestPsuType>` 
    
</dd>
</dl>

<dl>
<dd>

**redirect_url:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**valid_for_days:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**language:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">exchange_the_redirect_code_for_a_session_and_store_the_bank_accounts_it_exposes</a>(request: PostV1BankFeedsConnectionsCompleteRequest) -> Result&lt;PostV1BankFeedsConnectionsCompleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .exchange_the_redirect_code_for_a_session_and_store_the_bank_accounts_it_exposes(
            &PostV1BankFeedsConnectionsCompleteRequest {
                reference: "reference".to_string(),
                code: "code".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**reference:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_feeds_connections_get</a>(request: PostV1BankFeedsConnectionsGetRequest) -> Result&lt;PostV1BankFeedsConnectionsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_feeds_connections_get(
            &PostV1BankFeedsConnectionsGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">post_v1_bank_feeds_connections_list</a>(request: PostV1BankFeedsConnectionsListRequest) -> Result&lt;PostV1BankFeedsConnectionsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .post_v1bank_feeds_connections_list(
            &PostV1BankFeedsConnectionsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1BankFeedsConnectionsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1BankFeedsConnectionsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">revoke_the_consent_at_the_bank_and_drop_the_stored_connection</a>(request: PostV1BankFeedsConnectionsDeleteRequest) -> Result&lt;PostV1BankFeedsConnectionsDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .revoke_the_consent_at_the_bank_and_drop_the_stored_connection(
            &PostV1BankFeedsConnectionsDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">point_a_bank_feed_account_at_a_ledger_bank_account_so_its_transactions_can_be_synced</a>(request: PostV1BankFeedsAccountsLinkRequest) -> Result&lt;PostV1BankFeedsAccountsLinkResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .point_a_bank_feed_account_at_a_ledger_bank_account_so_its_transactions_can_be_synced(
            &PostV1BankFeedsAccountsLinkRequest {
                id: "id".to_string(),
                bank_account_id: None,
                create_bank_account: None,
                sync_from: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**bank_account_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**create_bank_account:** `Option<PostV1BankFeedsAccountsLinkRequestCreateBankAccount>` 
    
</dd>
</dl>

<dl>
<dd>

**sync_from:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">choose_the_import_template_applied_on_sync_and_how_often_the_account_is_synced_automatically</a>(request: PostV1BankFeedsAccountsConfigureRequest) -> Result&lt;PostV1BankFeedsAccountsConfigureResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.bank.choose_the_import_template_applied_on_sync_and_how_often_the_account_is_synced_automatically(&PostV1BankFeedsAccountsConfigureRequest {
        id: "id".to_string(),
        import_template_id: None,
        sync_schedule: None
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**import_template_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**sync_schedule:** `Option<PostV1BankFeedsAccountsConfigureRequestSyncSchedule>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">pull_new_transactions_from_the_bank_into_the_ledger_emits_bank_feed_synced</a>(request: PostV1BankFeedsSyncRequest) -> Result&lt;PostV1BankFeedsSyncResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bank
        .pull_new_transactions_from_the_bank_into_the_ledger_emits_bank_feed_synced(
            &PostV1BankFeedsSyncRequest {
                connection_id: "connectionId".to_string(),
                feed_account_id: None,
                date_from: None,
                date_to: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**connection_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**feed_account_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**date_from:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**date_to:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Files
<details><summary><code>client.files.<a href="/src/api/resources/files/client.rs">post_v1_files_upload</a>(request: PostV1FilesUploadRequest) -> Result&lt;PostV1FilesUploadResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .files
        .post_v1files_upload(
            &PostV1FilesUploadRequest {
                entity: "entity".to_string(),
                file_name: "fileName".to_string(),
                mime_type: "mimeType".to_string(),
                content: "content".to_string(),
                entity_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entity:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**entity_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**file_name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**mime_type:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**content:** `String` — Base64-encoded file content
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.files.<a href="/src/api/resources/files/client.rs">post_v1_files_get</a>(request: PostV1FilesGetRequest) -> Result&lt;PostV1FilesGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .files
        .post_v1files_get(
            &PostV1FilesGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.files.<a href="/src/api/resources/files/client.rs">post_v1_files_list</a>(request: PostV1FilesListRequest) -> Result&lt;PostV1FilesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .files
        .post_v1files_list(
            &PostV1FilesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1FilesListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1FilesListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.files.<a href="/src/api/resources/files/client.rs">post_v1_files_delete</a>(request: PostV1FilesDeleteRequest) -> Result&lt;PostV1FilesDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .files
        .post_v1files_delete(
            &PostV1FilesDeleteRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Reports
<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_trial_balance</a>(request: PostV1ReportsTrialBalanceRequest) -> Result&lt;PostV1ReportsTrialBalanceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_trial_balance(
            &PostV1ReportsTrialBalanceRequest {
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_size_category</a>(request: PostV1ReportsSizeCategoryRequest) -> Result&lt;PostV1ReportsSizeCategoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_size_category(&PostV1ReportsSizeCategoryRequest { year: 1000000 }, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**year:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_financial_statements</a>(request: PostV1ReportsFinancialStatementsRequest) -> Result&lt;PostV1ReportsFinancialStatementsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_financial_statements(
            &PostV1ReportsFinancialStatementsRequest {
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
                category: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**category:** `Option<PostV1ReportsFinancialStatementsRequestCategory>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_general_journal</a>(request: PostV1ReportsGeneralJournalRequest) -> Result&lt;PostV1ReportsGeneralJournalResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_general_journal(
            &PostV1ReportsGeneralJournalRequest {
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
                page: None,
                page_size: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_gl_detail</a>(request: PostV1ReportsGlDetailRequest) -> Result&lt;PostV1ReportsGlDetailResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_gl_detail(
            &PostV1ReportsGlDetailRequest {
                account_code: "accountCode".to_string(),
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_partner_balances</a>(request: PostV1ReportsPartnerBalancesRequest) -> Result&lt;PostV1ReportsPartnerBalancesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_partner_balances(
            &PostV1ReportsPartnerBalancesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_debt_aging</a>(request: PostV1ReportsDebtAgingRequest) -> Result&lt;PostV1ReportsDebtAgingResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_debt_aging(
            &PostV1ReportsDebtAgingRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**side:** `Option<PostV1ReportsDebtAgingRequestSide>` 
    
</dd>
</dl>

<dl>
<dd>

**as_of:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_monthly_summary</a>(request: PostV1ReportsMonthlySummaryRequest) -> Result&lt;PostV1ReportsMonthlySummaryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_monthly_summary(
            &PostV1ReportsMonthlySummaryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**months:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_stock_balance</a>(request: PostV1ReportsStockBalanceRequest) -> Result&lt;PostV1ReportsStockBalanceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_stock_balance(
            &PostV1ReportsStockBalanceRequest {
                as_of: "asOf".to_string(),
                warehouse_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**as_of:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**warehouse_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_stock_movement</a>(request: PostV1ReportsStockMovementRequest) -> Result&lt;PostV1ReportsStockMovementResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_stock_movement(
            &PostV1ReportsStockMovementRequest {
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
                warehouse_id: None,
                item_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**warehouse_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**item_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_vat_summary</a>(request: PostV1ReportsVatSummaryRequest) -> Result&lt;PostV1ReportsVatSummaryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_vat_summary(
            &PostV1ReportsVatSummaryRequest {
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
                side: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**side:** `Option<PostV1ReportsVatSummaryRequestSide>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_cash_flow</a>(request: PostV1ReportsCashFlowRequest) -> Result&lt;PostV1ReportsCashFlowResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_cash_flow(
            &PostV1ReportsCashFlowRequest {
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_stock_aging</a>(request: PostV1ReportsStockAgingRequest) -> Result&lt;PostV1ReportsStockAgingResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_stock_aging(
            &PostV1ReportsStockAgingRequest {
                as_of: "asOf".to_string(),
                warehouse_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**as_of:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**warehouse_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_stock_shortage</a>(request: PostV1ReportsStockShortageRequest) -> Result&lt;PostV1ReportsStockShortageResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_stock_shortage(
            &PostV1ReportsStockShortageRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**warehouse_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_sie</a>(request: PostV1ReportsSieRequest) -> Result&lt;PostV1ReportsSieResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export the ledger of one financial year as an SIE file (the Swedish standard accounting interchange format, specification 4B). The file carries the chart of accounts, the opening and closing balance of every balance sheet account and the turnover of every result account for the year and the year before it, and, when asked for, every posted voucher of the year with its lines. Cost centres travel as dimension 1 and projects as dimension 6. Services that build a Swedish annual report read this file.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_sie(
            &PostV1ReportsSieRequest {
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
                include_transactions: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**include_transactions:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_datev</a>(request: PostV1ReportsDatevRequest) -> Result&lt;PostV1ReportsDatevResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export the posted ledger of a period as a DATEV Buchungsstapel file (DATEV format, category 21, version 700). Every transaction becomes one or more bookings of an amount between an account and a contra account; a transaction with more than two lines is split into pairs whose totals match it. The file is semicolon separated and written in the Windows-1252 character set DATEV expects.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_datev(
            &PostV1ReportsDatevRequest {
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
                consultant_number: None,
                client_number: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**consultant_number:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**client_number:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_fec</a>(request: PostV1ReportsFecRequest) -> Result&lt;PostV1ReportsFecResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export the posted ledger of a period as a French FEC file (fichier des écritures comptables, order of 29 July 2013). One line per journal entry line, with the eighteen fields the order names, in their order, after a header line. Tab separated, UTF-8, comma as the decimal separator.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_fec(
            &PostV1ReportsFecRequest {
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_eu_purchases</a>(request: PostV1ReportsEuPurchasesRequest) -> Result&lt;PostV1ReportsEuPurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_eu_purchases(
            &PostV1ReportsEuPurchasesRequest {
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_vat_detail</a>(request: PostV1ReportsVatDetailRequest) -> Result&lt;PostV1ReportsVatDetailResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_vat_detail(
            &PostV1ReportsVatDetailRequest {
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
                side: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**side:** `Option<PostV1ReportsVatDetailRequestSide>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_pos_sales</a>(request: PostV1ReportsPosSalesRequest) -> Result&lt;PostV1ReportsPosSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_pos_sales(
            &PostV1ReportsPosSalesRequest {
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_online_sales</a>(request: PostV1ReportsOnlineSalesRequest) -> Result&lt;PostV1ReportsOnlineSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_online_sales(
            &PostV1ReportsOnlineSalesRequest {
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_oss</a>(request: PostV1ReportsOssRequest) -> Result&lt;PostV1ReportsOssResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_oss(
            &PostV1ReportsOssRequest {
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_advance_reconciliation</a>(request: PostV1ReportsAdvanceReconciliationRequest) -> Result&lt;PostV1ReportsAdvanceReconciliationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_advance_reconciliation(
            &PostV1ReportsAdvanceReconciliationRequest {
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_write_off_acts</a>(request: PostV1ReportsWriteOffActsRequest) -> Result&lt;PostV1ReportsWriteOffActsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_write_off_acts(
            &PostV1ReportsWriteOffActsRequest {
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
                warehouse_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**warehouse_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_cost_centers</a>(request: PostV1ReportsCostCentersRequest) -> Result&lt;PostV1ReportsCostCentersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_cost_centers(
            &PostV1ReportsCostCentersRequest {
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_cost_center_activity</a>(request: PostV1ReportsCostCenterActivityRequest) -> Result&lt;PostV1ReportsCostCenterActivityResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_cost_center_activity(
            &PostV1ReportsCostCenterActivityRequest {
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
                cost_center_id: "costCenterId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**cost_center_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_cost_center_items</a>(request: PostV1ReportsCostCenterItemsRequest) -> Result&lt;PostV1ReportsCostCenterItemsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_cost_center_items(
            &PostV1ReportsCostCenterItemsRequest {
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
                cost_center_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**cost_center_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_jobs_create</a>(request: PostV1ReportsJobsCreateRequest) -> Result&lt;PostV1ReportsJobsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_jobs_create(
            &PostV1ReportsJobsCreateRequest {
                report_type: "reportType".to_string(),
                params: None,
                formats: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**report_type:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**params:** `Option<std::collections::HashMap<String, serde_json::Value>>` 
    
</dd>
</dl>

<dl>
<dd>

**formats:** `Option<Vec<PostV1ReportsJobsCreateRequestFormatsItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_jobs_get</a>(request: PostV1ReportsJobsGetRequest) -> Result&lt;PostV1ReportsJobsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_jobs_get(
            &PostV1ReportsJobsGetRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">post_v1_reports_jobs_list</a>(request: PostV1ReportsJobsListRequest) -> Result&lt;PostV1ReportsJobsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .reports
        .post_v1reports_jobs_list(
            &PostV1ReportsJobsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<Vec<PostV1ReportsJobsListRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PostV1ReportsJobsListRequestFilterItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**totals:** `Option<Vec<String>>` — Numeric fields to sum over every row matching the filter (not only the current page)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Consolidation
<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">post_v1_consolidation_groups_create</a>(request: PostV1ConsolidationGroupsCreateRequest) -> Result&lt;PostV1ConsolidationGroupsCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .consolidation
        .post_v1consolidation_groups_create(
            &PostV1ConsolidationGroupsCreateRequest {
                name: "name".to_string(),
                presentation_currency: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**presentation_currency:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">post_v1_consolidation_groups_list</a>(request: PostV1ConsolidationGroupsListRequest) -> Result&lt;PostV1ConsolidationGroupsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .consolidation
        .post_v1consolidation_groups_list(
            &PostV1ConsolidationGroupsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">post_v1_consolidation_groups_get</a>(request: PostV1ConsolidationGroupsGetRequest) -> Result&lt;PostV1ConsolidationGroupsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .consolidation
        .post_v1consolidation_groups_get(
            &PostV1ConsolidationGroupsGetRequest {
                group_id: "groupId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**group_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">post_v1_consolidation_groups_update</a>(request: PostV1ConsolidationGroupsUpdateRequest) -> Result&lt;PostV1ConsolidationGroupsUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .consolidation
        .post_v1consolidation_groups_update(
            &PostV1ConsolidationGroupsUpdateRequest {
                group_id: "groupId".to_string(),
                name: None,
                presentation_currency: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**group_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**presentation_currency:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">post_v1_consolidation_groups_delete</a>(request: PostV1ConsolidationGroupsDeleteRequest) -> Result&lt;PostV1ConsolidationGroupsDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .consolidation
        .post_v1consolidation_groups_delete(
            &PostV1ConsolidationGroupsDeleteRequest {
                group_id: "groupId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**group_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">post_v1_consolidation_members_add</a>(request: PostV1ConsolidationMembersAddRequest) -> Result&lt;PostV1ConsolidationMembersAddResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .consolidation
        .post_v1consolidation_members_add(
            &PostV1ConsolidationMembersAddRequest {
                group_id: "groupId".to_string(),
                member_company_id: "memberCompanyId".to_string(),
                ownership_percent: None,
                method: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**group_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**member_company_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**ownership_percent:** `Option<f64>` 
    
</dd>
</dl>

<dl>
<dd>

**method:** `Option<PostV1ConsolidationMembersAddRequestMethod>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">post_v1_consolidation_members_remove</a>(request: PostV1ConsolidationMembersRemoveRequest) -> Result&lt;PostV1ConsolidationMembersRemoveResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .consolidation
        .post_v1consolidation_members_remove(
            &PostV1ConsolidationMembersRemoveRequest {
                group_id: "groupId".to_string(),
                member_company_id: "memberCompanyId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**group_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**member_company_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">post_v1_consolidation_intercompany_candidates</a>(request: PostV1ConsolidationIntercompanyCandidatesRequest) -> Result&lt;PostV1ConsolidationIntercompanyCandidatesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Partners in member companies that look like other members of the same group (matched on company code or VAT code), with any existing intercompany link. Confirming a candidate via intercompany/links/set enables invoice mirroring.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .consolidation
        .post_v1consolidation_intercompany_candidates(
            &PostV1ConsolidationIntercompanyCandidatesRequest {
                group_id: "groupId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**group_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">post_v1_consolidation_intercompany_links_set</a>(request: PostV1ConsolidationIntercompanyLinksSetRequest) -> Result&lt;PostV1ConsolidationIntercompanyLinksSetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Confirm that a partner record in one member company represents another member company of the group. Once links exist in both directions, issuing an intercompany sale invoice automatically creates the matching draft purchase invoice in the counterparty.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .consolidation
        .post_v1consolidation_intercompany_links_set(
            &PostV1ConsolidationIntercompanyLinksSetRequest {
                group_id: "groupId".to_string(),
                partner_id: "partnerId".to_string(),
                counterparty_company_id: "counterpartyCompanyId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**group_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**partner_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**counterparty_company_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">post_v1_consolidation_intercompany_links_list</a>(request: PostV1ConsolidationIntercompanyLinksListRequest) -> Result&lt;PostV1ConsolidationIntercompanyLinksListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .consolidation
        .post_v1consolidation_intercompany_links_list(
            &PostV1ConsolidationIntercompanyLinksListRequest {
                group_id: "groupId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**group_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">post_v1_consolidation_intercompany_links_remove</a>(request: PostV1ConsolidationIntercompanyLinksRemoveRequest) -> Result&lt;PostV1ConsolidationIntercompanyLinksRemoveResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .consolidation
        .post_v1consolidation_intercompany_links_remove(
            &PostV1ConsolidationIntercompanyLinksRemoveRequest {
                group_id: "groupId".to_string(),
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**group_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">post_v1_consolidation_intercompany_report</a>(request: PostV1ConsolidationIntercompanyReportRequest) -> Result&lt;PostV1ConsolidationIntercompanyReportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Intercompany reconciliation for a period: every issued intercompany sale invoice with its mirrored or manually recorded counterpart, unmatched documents on both sides, and per-currency totals with differences. Confirmed pairs are the basis for consolidation eliminations.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .consolidation
        .post_v1consolidation_intercompany_report(
            &PostV1ConsolidationIntercompanyReportRequest {
                group_id: "groupId".to_string(),
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**group_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">post_v1_consolidation_report</a>(request: PostV1ConsolidationReportRequest) -> Result&lt;PostV1ConsolidationReportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .consolidation
        .post_v1consolidation_report(
            &PostV1ConsolidationReportRequest {
                group_id: "groupId".to_string(),
                from_date: "fromDate".to_string(),
                to_date: "toDate".to_string(),
                category: None,
                eliminations: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**group_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**from_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**category:** `Option<PostV1ConsolidationReportRequestCategory>` 
    
</dd>
</dl>

<dl>
<dd>

**eliminations:** `Option<Vec<PostV1ConsolidationReportRequestEliminationsItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Public
<details><summary><code>client.public.<a href="/src/api/resources/public/client.rs">post_v1_public_integration_requests</a>(request: PostV1PublicIntegrationRequestsRequest) -> Result&lt;PostV1PublicIntegrationRequestsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .public
        .post_v1public_integration_requests(
            &PostV1PublicIntegrationRequestsRequest {
                integration: "integration".to_string(),
                name: "name".to_string(),
                email: "email".to_string(),
                company: None,
                details: None,
                website: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**integration:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**company:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**email:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**details:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**website:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.public.<a href="/src/api/resources/public/client.rs">get_v1_public_pay_token</a>(token: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .public
        .get_v1public_pay_token(&"token".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**token:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Billing
<details><summary><code>client.billing.<a href="/src/api/resources/billing/client.rs">post_v1_billing_account_get</a>(request: PostV1BillingAccountGetRequest) -> Result&lt;PostV1BillingAccountGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .billing
        .post_v1billing_account_get(
            &PostV1BillingAccountGetRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.billing.<a href="/src/api/resources/billing/client.rs">post_v1_billing_account_set_plan</a>(request: PostV1BillingAccountSetPlanRequest) -> Result&lt;PostV1BillingAccountSetPlanResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .billing
        .post_v1billing_account_set_plan(
            &PostV1BillingAccountSetPlanRequest {
                plan: PostV1BillingAccountSetPlanRequestPlan::Starter,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**plan:** `PostV1BillingAccountSetPlanRequestPlan` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.billing.<a href="/src/api/resources/billing/client.rs">post_v1_billing_topup_create</a>(request: PostV1BillingTopupCreateRequest) -> Result&lt;PostV1BillingTopupCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .billing
        .post_v1billing_topup_create(
            &PostV1BillingTopupCreateRequest {
                amount_cents: 1000000,
                locale: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**amount_cents:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**locale:** `Option<PostV1BillingTopupCreateRequestLocale>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.billing.<a href="/src/api/resources/billing/client.rs">post_v1_billing_portal_create</a>(request: PostV1BillingPortalCreateRequest) -> Result&lt;PostV1BillingPortalCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .billing
        .post_v1billing_portal_create(
            &PostV1BillingPortalCreateRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**locale:** `Option<PostV1BillingPortalCreateRequestLocale>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.billing.<a href="/src/api/resources/billing/client.rs">post_v1_billing_transactions_list</a>(request: PostV1BillingTransactionsListRequest) -> Result&lt;PostV1BillingTransactionsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .billing
        .post_v1billing_transactions_list(
            &PostV1BillingTransactionsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.billing.<a href="/src/api/resources/billing/client.rs">post_v1_billing_usage_list</a>(request: PostV1BillingUsageListRequest) -> Result&lt;PostV1BillingUsageListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .billing
        .post_v1billing_usage_list(
            &PostV1BillingUsageListRequest {
                from: "from".to_string(),
                to: "to".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**from:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**to:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Account
<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_login_link_request</a>(request: PostV1AccountLoginLinkRequestRequest) -> Result&lt;PostV1AccountLoginLinkRequestResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_login_link_request(
            &PostV1AccountLoginLinkRequestRequest {
                email: "email".to_string(),
                locale: None,
                accept_terms: None,
                accept_dpa: None,
                referral_code: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**email:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**locale:** `Option<PostV1AccountLoginLinkRequestRequestLocale>` 
    
</dd>
</dl>

<dl>
<dd>

**accept_terms:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**accept_dpa:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**referral_code:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_login_link_consume</a>(request: PostV1AccountLoginLinkConsumeRequest) -> Result&lt;PostV1AccountLoginLinkConsumeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_login_link_consume(
            &PostV1AccountLoginLinkConsumeRequest {
                token: "token".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**token:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_logout</a>(request: PostV1AccountLogoutRequest) -> Result&lt;PostV1AccountLogoutResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_logout(
            &PostV1AccountLogoutRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_me</a>(request: PostV1AccountMeRequest) -> Result&lt;PostV1AccountMeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_me(
            &PostV1AccountMeRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_members_list</a>(request: PostV1AccountMembersListRequest) -> Result&lt;PostV1AccountMembersListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_members_list(
            &PostV1AccountMembersListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_members_set_role</a>(request: PostV1AccountMembersSetRoleRequest) -> Result&lt;PostV1AccountMembersSetRoleResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_members_set_role(
            &PostV1AccountMembersSetRoleRequest {
                user_id: "userId".to_string(),
                role: PostV1AccountMembersSetRoleRequestRole::Admin,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**user_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**role:** `PostV1AccountMembersSetRoleRequestRole` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_members_transfer_ownership</a>(request: PostV1AccountMembersTransferOwnershipRequest) -> Result&lt;PostV1AccountMembersTransferOwnershipResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_members_transfer_ownership(
            &PostV1AccountMembersTransferOwnershipRequest {
                user_id: "userId".to_string(),
                move_payer: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**user_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**move_payer:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_members_remove</a>(request: PostV1AccountMembersRemoveRequest) -> Result&lt;PostV1AccountMembersRemoveResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_members_remove(
            &PostV1AccountMembersRemoveRequest {
                user_id: "userId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**user_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_invites_create</a>(request: PostV1AccountInvitesCreateRequest) -> Result&lt;PostV1AccountInvitesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_invites_create(
            &PostV1AccountInvitesCreateRequest {
                email: "email".to_string(),
                role: PostV1AccountInvitesCreateRequestRole::Admin,
                locale: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**email:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**role:** `PostV1AccountInvitesCreateRequestRole` 
    
</dd>
</dl>

<dl>
<dd>

**locale:** `Option<PostV1AccountInvitesCreateRequestLocale>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_invites_list</a>(request: PostV1AccountInvitesListRequest) -> Result&lt;PostV1AccountInvitesListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_invites_list(
            &PostV1AccountInvitesListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_invites_revoke</a>(request: PostV1AccountInvitesRevokeRequest) -> Result&lt;PostV1AccountInvitesRevokeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_invites_revoke(
            &PostV1AccountInvitesRevokeRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_invites_get</a>(request: PostV1AccountInvitesGetRequest) -> Result&lt;PostV1AccountInvitesGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_invites_get(
            &PostV1AccountInvitesGetRequest {
                token: "token".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**token:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_invites_accept</a>(request: PostV1AccountInvitesAcceptRequest) -> Result&lt;PostV1AccountInvitesAcceptResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_invites_accept(
            &PostV1AccountInvitesAcceptRequest {
                token: "token".to_string(),
                name: None,
                locale: None,
                accept_terms: None,
                accept_dpa: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**token:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**locale:** `Option<PostV1AccountInvitesAcceptRequestLocale>` 
    
</dd>
</dl>

<dl>
<dd>

**accept_terms:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**accept_dpa:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_locale_set</a>(request: PostV1AccountLocaleSetRequest) -> Result&lt;PostV1AccountLocaleSetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_locale_set(
            &PostV1AccountLocaleSetRequest {
                locale: PostV1AccountLocaleSetRequestLocale::En,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**locale:** `PostV1AccountLocaleSetRequestLocale` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_companies_create</a>(request: PostV1AccountCompaniesCreateRequest) -> Result&lt;PostV1AccountCompaniesCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_companies_create(
            &PostV1AccountCompaniesCreateRequest {
                name: "name".to_string(),
                code: None,
                vat_code: None,
                sme_exemption_number: None,
                is_vat_payer: None,
                vat_period: None,
                fiscal_year_end_month: None,
                time_zone: None,
                filing_options: None,
                address: None,
                email: None,
                phone: None,
                iban: None,
                bank_name: None,
                peppol_id: None,
                sepa_creditor_id: None,
                default_invoice_currency: None,
                legal_form: None,
                registry_name: None,
                incorporated_on: None,
                share_capital: None,
                accounts_kept_by: None,
                bookkeeper_name: None,
                auditor_name: None,
                auditor_registration_number: None,
                audit_required: None,
                country_code: None,
                is_sandbox: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sme_exemption_number:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**is_vat_payer:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_period:** `Option<PostV1AccountCompaniesCreateRequestVatPeriod>` 
    
</dd>
</dl>

<dl>
<dd>

**fiscal_year_end_month:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**time_zone:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**filing_options:** `Option<std::collections::HashMap<String, String>>` 
    
</dd>
</dl>

<dl>
<dd>

**address:** `Option<PostV1AccountCompaniesCreateRequestAddress>` 
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**phone:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**iban:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**bank_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**peppol_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sepa_creditor_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**default_invoice_currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**legal_form:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**registry_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**incorporated_on:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**share_capital:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**accounts_kept_by:** `Option<PostV1AccountCompaniesCreateRequestAccountsKeptBy>` 
    
</dd>
</dl>

<dl>
<dd>

**bookkeeper_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**auditor_name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**auditor_registration_number:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**audit_required:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**country_code:** `Option<PostV1AccountCompaniesCreateRequestCountryCode>` — Jurisdiction the company is registered in (immutable after creation)
    
</dd>
</dl>

<dl>
<dd>

**is_sandbox:** `Option<bool>` — Sandbox companies hold test data and are purged immediately on delete (immutable after creation)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_companies_select</a>(request: PostV1AccountCompaniesSelectRequest) -> Result&lt;PostV1AccountCompaniesSelectResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_companies_select(
            &PostV1AccountCompaniesSelectRequest {
                company_id: "companyId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**company_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_companies_profile</a>(request: PostV1AccountCompaniesProfileRequest) -> Result&lt;PostV1AccountCompaniesProfileResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_companies_profile(
            &PostV1AccountCompaniesProfileRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_companies_update</a>(request: PostV1AccountCompaniesUpdateRequest) -> Result&lt;PostV1AccountCompaniesUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_companies_update(
            &PostV1AccountCompaniesUpdateRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**sme_exemption_number:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**is_vat_payer:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_period:** `Option<Option<PostV1AccountCompaniesUpdateRequestVatPeriod>>` 
    
</dd>
</dl>

<dl>
<dd>

**fiscal_year_end_month:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**time_zone:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**filing_options:** `Option<Option<std::collections::HashMap<String, Option<String>>>>` 
    
</dd>
</dl>

<dl>
<dd>

**address:** `Option<PostV1AccountCompaniesUpdateRequestAddress>` 
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**phone:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**iban:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**bank_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**peppol_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**sepa_creditor_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**default_invoice_currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**legal_form:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**registry_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**incorporated_on:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**share_capital:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**accounts_kept_by:** `Option<Option<PostV1AccountCompaniesUpdateRequestAccountsKeptBy>>` 
    
</dd>
</dl>

<dl>
<dd>

**bookkeeper_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**auditor_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**auditor_registration_number:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**audit_required:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**logo:** `Option<PostV1AccountCompaniesUpdateRequestLogo>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_companies_archive</a>(request: PostV1AccountCompaniesArchiveRequest) -> Result&lt;PostV1AccountCompaniesArchiveResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_companies_archive(
            &PostV1AccountCompaniesArchiveRequest {
                company_id: "companyId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**company_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_companies_delete</a>(request: PostV1AccountCompaniesDeleteRequest) -> Result&lt;PostV1AccountCompaniesDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_companies_delete(
            &PostV1AccountCompaniesDeleteRequest {
                company_id: "companyId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**company_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_companies_activate</a>(request: PostV1AccountCompaniesActivateRequest) -> Result&lt;PostV1AccountCompaniesActivateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_companies_activate(
            &PostV1AccountCompaniesActivateRequest {
                company_id: "companyId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**company_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_api_keys_create</a>(request: PostV1AccountApiKeysCreateRequest) -> Result&lt;PostV1AccountApiKeysCreateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_api_keys_create(
            &PostV1AccountAPIKeysCreateRequest {
                name: "name".to_string(),
                scopes: None,
                expires_in_days: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**scopes:** `Option<Vec<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**expires_in_days:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_api_keys_list</a>(request: PostV1AccountApiKeysListRequest) -> Result&lt;PostV1AccountApiKeysListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_api_keys_list(
            &PostV1AccountAPIKeysListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">issue_a_replacement_for_an_api_key_and_set_the_old_one_to_stop_working_after_a_short_overlap</a>(request: PostV1AccountApiKeysRotateRequest) -> Result&lt;PostV1AccountApiKeysRotateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.account.issue_a_replacement_for_an_api_key_and_set_the_old_one_to_stop_working_after_a_short_overlap(&PostV1AccountAPIKeysRotateRequest {
        id: "id".to_string(),
        overlap_hours: None,
        expires_in_days: None
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**overlap_hours:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**expires_in_days:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_api_keys_revoke</a>(request: PostV1AccountApiKeysRevokeRequest) -> Result&lt;PostV1AccountApiKeysRevokeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_api_keys_revoke(
            &PostV1AccountAPIKeysRevokeRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_consent_accept</a>(request: PostV1AccountConsentAcceptRequest) -> Result&lt;PostV1AccountConsentAcceptResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_consent_accept(
            &PostV1AccountConsentAcceptRequest {
                accept_terms: true,
                accept_dpa: true,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**accept_terms:** `bool` 
    
</dd>
</dl>

<dl>
<dd>

**accept_dpa:** `bool` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_profile_update</a>(request: PostV1AccountProfileUpdateRequest) -> Result&lt;PostV1AccountProfileUpdateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_profile_update(
            &PostV1AccountProfileUpdateRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_email_change_request</a>(request: PostV1AccountEmailChangeRequestRequest) -> Result&lt;PostV1AccountEmailChangeRequestResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_email_change_request(
            &PostV1AccountEmailChangeRequestRequest {
                new_email: "newEmail".to_string(),
                locale: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**new_email:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**locale:** `Option<PostV1AccountEmailChangeRequestRequestLocale>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_sessions_list</a>(request: PostV1AccountSessionsListRequest) -> Result&lt;PostV1AccountSessionsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_sessions_list(
            &PostV1AccountSessionsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_sessions_revoke</a>(request: PostV1AccountSessionsRevokeRequest) -> Result&lt;PostV1AccountSessionsRevokeResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_sessions_revoke(
            &PostV1AccountSessionsRevokeRequest {
                id: "id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_sessions_revoke_others</a>(request: PostV1AccountSessionsRevokeOthersRequest) -> Result&lt;PostV1AccountSessionsRevokeOthersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_sessions_revoke_others(
            &PostV1AccountSessionsRevokeOthersRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">download_everything_nordlet_stores_about_the_signed_in_user</a>(request: PostV1AccountExportRequest) -> Result&lt;PostV1AccountExportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .download_everything_nordlet_stores_about_the_signed_in_user(
            &PostV1AccountExportRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">delete_the_signed_in_user_account</a>(request: PostV1AccountDeleteRequest) -> Result&lt;PostV1AccountDeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Removes the user: sessions, sign-in links, memberships and pending invitations are deleted at once; the email and name are replaced by an anonymous placeholder immediately and the remaining row is removed after 30 days. Refused while the user still owns or pays for a company that is not deleted.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .delete_the_signed_in_user_account(
            &PostV1AccountDeleteRequest {
                confirm_email: "confirmEmail".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**confirm_email:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_referral_get</a>(request: PostV1AccountReferralGetRequest) -> Result&lt;PostV1AccountReferralGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_referral_get(
            &PostV1AccountReferralGetRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_referral_convert</a>(request: PostV1AccountReferralConvertRequest) -> Result&lt;PostV1AccountReferralConvertResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_referral_convert(
            &PostV1AccountReferralConvertRequest { points: 1000000 },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**points:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_table_settings_get</a>(request: PostV1AccountTableSettingsGetRequest) -> Result&lt;PostV1AccountTableSettingsGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_table_settings_get(
            &PostV1AccountTableSettingsGetRequest {
                table_key: "tableKey".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**table_key:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_table_settings_set</a>(request: PostV1AccountTableSettingsSetRequest) -> Result&lt;PostV1AccountTableSettingsSetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_table_settings_set(
            &PostV1AccountTableSettingsSetRequest {
                table_key: "tableKey".to_string(),
                columns: None,
                page_size: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**table_key:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**columns:** `Option<Option<Vec<String>>>` 
    
</dd>
</dl>

<dl>
<dd>

**page_size:** `Option<Option<f64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">post_v1_account_table_settings_list</a>(request: PostV1AccountTableSettingsListRequest) -> Result&lt;PostV1AccountTableSettingsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .account
        .post_v1account_table_settings_list(
            &PostV1AccountTableSettingsListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

