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

**notes:** `Option<String>` 
    
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

**notes:** `Option<String>` 
    
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

**address:** `Option<PostV1PartnersUpdateRequestAddress>` 
    
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
                translations: None,
                components: None,
            },
            None,
        )
        .await;
}
```
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

**translations:** `Option<std::collections::HashMap<String, PostV1CatalogItemsCreateRequestTranslationsValue>>` 
    
</dd>
</dl>

<dl>
<dd>

**components:** `Option<Vec<PostV1CatalogItemsCreateRequestComponentsItem>>` 
    
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
                translations: None,
                components: None,
            },
            None,
        )
        .await;
}
```
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

**name:** `Option<String>` 
    
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

**translations:** `Option<std::collections::HashMap<String, PostV1CatalogItemsUpdateRequestTranslationsValue>>` 
    
</dd>
</dl>

<dl>
<dd>

**components:** `Option<Vec<PostV1CatalogItemsUpdateRequestComponentsItem>>` 
    
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
                vat_scheme: None,
                vat_country_code: None,
                deemed_supplier: None,
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

**vat_scheme:** `Option<PostV1SalesInvoicesCreateRequestVatScheme>` 
    
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
                currency: None,
                issue_date: None,
                due_date: None,
                vat_scheme: None,
                vat_country_code: None,
                deemed_supplier: None,
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

**vat_scheme:** `Option<Option<PostV1SalesInvoicesUpdateRequestVatScheme>>` 
    
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

**notes:** `Option<String>` 
    
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
            },
            None,
        )
        .await;
}
```
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
                apply_npd: None,
                npd_override: None,
                pension_accumulation: None,
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

**apply_npd:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**npd_override:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**pension_accumulation:** `Option<bool>` 
    
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
                apply_npd: None,
                npd_override: None,
                pension_accumulation: None,
                notes: None,
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

**code:** `Option<String>` 
    
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

**address:** `Option<PostV1HrEmployeesUpdateRequestAddress>` 
    
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

**apply_npd:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**npd_override:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**pension_accumulation:** `Option<bool>` 
    
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

<dl>
<dd>

**termination_date:** `Option<String>` 
    
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
                contract_no: "contractNo".to_string(),
                start_date: "startDate".to_string(),
                base_salary: "baseSalary".to_string(),
                position_id: None,
                department_id: None,
                schedule_id: None,
                r#type: None,
                end_date: None,
                salary_type: None,
                work_hours_per_week: None,
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

**contract_no:** `String` 
    
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

**work_hours_per_week:** `Option<String>` 
    
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

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">post_v1_payroll_calc</a>(request: PostV1PayrollCalcRequest) -> Result&lt;PostV1PayrollCalcResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .post_v1payroll_calc(
            &PostV1PayrollCalcRequest {
                taxable_base: "taxableBase".to_string(),
                date: "date".to_string(),
                apply_npd: None,
                npd_override: None,
                pension_accumulation: None,
                fixed_term: None,
            },
            None,
        )
        .await;
}
```
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

**apply_npd:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**npd_override:** `Option<String>` 
    
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
                partner_id: "partnerId".to_string(),
                number: "number".to_string(),
                start_date: "startDate".to_string(),
                type_id: None,
                name: None,
                end_date: None,
                auto_renew: None,
                value: None,
                billing_period: None,
                currency: None,
                status: None,
                notes: None,
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

**partner_id:** `String` 
    
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
                name: None,
                end_date: None,
                auto_renew: None,
                value: None,
                billing_period: None,
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

**type_id:** `Option<Option<String>>` 
    
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
</dd>
</dl>


</dd>
</dl>
</details>

## Production
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
            },
            None,
        )
        .await;
}
```
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

**dispatch_at:** `Option<String>` 
    
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
            },
            None,
        )
        .await;
}
```
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
                format: None,
            },
            None,
        )
        .await;
}
```
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

**format:** `Option<PostV1BankStatementsImportRequestFormat>` 
    
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
                entity_id: "entityId".to_string(),
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

**entity:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**entity_id:** `String` 
    
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
            },
            None,
        )
        .await;
}
```
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
            },
            None,
        )
        .await;
}
```
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
                locale: PostV1AccountLocaleSetRequestLocale::Lt,
            },
            None,
        )
        .await;
}
```
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
                address: None,
                email: None,
                phone: None,
                iban: None,
                bank_name: None,
                peppol_id: None,
                default_invoice_currency: None,
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

**default_invoice_currency:** `Option<String>` 
    
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

**address:** `Option<PostV1AccountCompaniesUpdateRequestAddress>` 
    
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

**default_invoice_currency:** `Option<String>` 
    
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
            },
            None,
        )
        .await;
}
```
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

