# Reference
## reference
<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">exchange_rates_sync</a>(request: ExchangeRatesSyncReferenceRequest) -> Result&lt;ExchangeRatesSyncReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .exchange_rates_sync(
            &ExchangeRatesSyncReferenceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">exchange_rates_list</a>(request: ExchangeRatesListReferenceRequest) -> Result&lt;ExchangeRatesListReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .exchange_rates_list(
            &ExchangeRatesListReferenceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<ExchangeRatesListReferenceRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<ExchangeRatesListReferenceRequestFilterItem>>` 
    
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

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">exchange_rates_set</a>(request: ExchangeRatesSetReferenceRequest) -> Result&lt;ExchangeRatesSetReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .exchange_rates_set(
            &ExchangeRatesSetReferenceRequest {
                currency: "currency".to_string(),
                date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                rate: "121.00000000".to_string(),
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">exchange_rates_overrides_list</a>(request: ExchangeRatesOverridesListReferenceRequest) -> Result&lt;ExchangeRatesOverridesListReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .exchange_rates_overrides_list(
            &ExchangeRatesOverridesListReferenceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<ExchangeRatesOverridesListReferenceRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<ExchangeRatesOverridesListReferenceRequestFilterItem>>` 
    
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

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">exchange_rates_overrides_delete</a>(request: ExchangeRatesOverridesDeleteReferenceRequest) -> Result&lt;ExchangeRatesOverridesDeleteReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .exchange_rates_overrides_delete(
            &ExchangeRatesOverridesDeleteReferenceRequest {
                currency: "currency".to_string(),
                date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">countries_list</a>(request: CountriesListReferenceRequest) -> Result&lt;CountriesListReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .countries_list(
            &CountriesListReferenceRequest {
                ..Default::default()
            },
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

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">lt_counties_list</a>(request: LtCountiesListReferenceRequest) -> Result&lt;LtCountiesListReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .lt_counties_list(
            &LtCountiesListReferenceRequest {
                ..Default::default()
            },
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

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">lt_municipalities_list</a>(request: LtMunicipalitiesListReferenceRequest) -> Result&lt;LtMunicipalitiesListReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .lt_municipalities_list(
            &LtMunicipalitiesListReferenceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">lt_cities_list</a>(request: LtCitiesListReferenceRequest) -> Result&lt;LtCitiesListReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .lt_cities_list(
            &LtCitiesListReferenceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">banks_list</a>(request: BanksListReferenceRequest) -> Result&lt;BanksListReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .banks_list(
            &BanksListReferenceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<BanksListReferenceRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<BanksListReferenceRequestFilterItem>>` 
    
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

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">banks_upsert</a>(request: BanksUpsertReferenceRequest) -> Result&lt;BanksUpsertReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .banks_upsert(
            &BanksUpsertReferenceRequest {
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

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">lt_regions_list</a>(request: LtRegionsListReferenceRequest) -> Result&lt;LtRegionsListReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .lt_regions_list(
            &LtRegionsListReferenceRequest {
                ..Default::default()
            },
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

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">currencies_list</a>(request: CurrenciesListReferenceRequest) -> Result&lt;CurrenciesListReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .currencies_list(
            &CurrenciesListReferenceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<CurrenciesListReferenceRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<CurrenciesListReferenceRequestFilterItem>>` 
    
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

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">vat_classifiers_list</a>(request: VatClassifiersListReferenceRequest) -> Result&lt;VatClassifiersListReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .vat_classifiers_list(
            &VatClassifiersListReferenceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<VatClassifiersListReferenceRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<VatClassifiersListReferenceRequestFilterItem>>` 
    
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

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">vat_classifiers_upsert</a>(request: VatClassifiersUpsertReferenceRequest) -> Result&lt;VatClassifiersUpsertReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .vat_classifiers_upsert(
            &VatClassifiersUpsertReferenceRequest {
                rows: vec![VatClassifiersUpsertReferenceRequestRowsItem {
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

**rows:** `Vec<VatClassifiersUpsertReferenceRequestRowsItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">eu_vat_rates_list</a>(request: EuVatRatesListReferenceRequest) -> Result&lt;EuVatRatesListReferenceResponse, ApiError&gt;</code></summary>
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
        .eu_vat_rates_list(
            &EuVatRatesListReferenceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">eu_vat_rates_set_overrides</a>(request: EuVatRatesSetOverridesReferenceRequest) -> Result&lt;EuVatRatesSetOverridesReferenceResponse, ApiError&gt;</code></summary>
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
        .eu_vat_rates_set_overrides(
            &EuVatRatesSetOverridesReferenceRequest {
                country_code: "countryCode".to_string(),
                rates: vec![EuVatRatesSetOverridesReferenceRequestRatesItem {
                    category: EuVatRatesSetOverridesReferenceRequestRatesItemCategory::Standard,
                    rate_percent: "121.00".to_string(),
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

**rates:** `Vec<EuVatRatesSetOverridesReferenceRequestRatesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">vat_resolve</a>(request: VatResolveReferenceRequest) -> Result&lt;VatResolveReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .vat_resolve(
            &VatResolveReferenceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**supply_type:** `Option<VatResolveReferenceRequestSupplyType>` 
    
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

<dl>
<dd>

**service_kind:** `Option<VatResolveReferenceRequestServiceKind>` 
    
</dd>
</dl>

<dl>
<dd>

**service_country_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**underlying_supplier_gave_vat_number:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**underlying_supplier_charges_vat:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**goods_kind:** `Option<VatResolveReferenceRequestGoodsKind>` 
    
</dd>
</dl>

<dl>
<dd>

**goods_location_country_code:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">cn_codes_list</a>(request: CnCodesListReferenceRequest) -> Result&lt;CnCodesListReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .cn_codes_list(
            &CnCodesListReferenceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<CnCodesListReferenceRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<CnCodesListReferenceRequestFilterItem>>` 
    
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

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">cn_codes_upsert</a>(request: CnCodesUpsertReferenceRequest) -> Result&lt;CnCodesUpsertReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .cn_codes_upsert(
            &CnCodesUpsertReferenceRequest {
                rows: vec![CnCodesUpsertReferenceRequestRowsItem {
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

**rows:** `Vec<CnCodesUpsertReferenceRequestRowsItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">compliance_versions_list</a>(request: ComplianceVersionsListReferenceRequest) -> Result&lt;ComplianceVersionsListReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .compliance_versions_list(
            &ComplianceVersionsListReferenceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">intrastat_thresholds_list</a>(request: IntrastatThresholdsListReferenceRequest) -> Result&lt;IntrastatThresholdsListReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .intrastat_thresholds_list(
            &IntrastatThresholdsListReferenceRequest {
                ..Default::default()
            },
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

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">units_list</a>(request: UnitsListReferenceRequest) -> Result&lt;UnitsListReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .units_list(
            &UnitsListReferenceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<UnitsListReferenceRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<UnitsListReferenceRequestFilterItem>>` 
    
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

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">series_create</a>(request: SeriesCreateReferenceRequest) -> Result&lt;SeriesCreateReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .series_create(
            &SeriesCreateReferenceRequest {
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

<details><summary><code>client.reference.<a href="/src/api/resources/reference/client.rs">series_list</a>(request: SeriesListReferenceRequest) -> Result&lt;SeriesListReferenceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .series_list(
            &SeriesListReferenceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<SeriesListReferenceRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<SeriesListReferenceRequestFilterItem>>` 
    
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

## partners
<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">addresses_create</a>(request: AddressesCreatePartnersRequest) -> Result&lt;AddressesCreatePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .addresses_create(
            &AddressesCreatePartnersRequest {
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

**type_:** `Option<AddressesCreatePartnersRequestType>` 
    
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">addresses_update</a>(request: AddressesUpdatePartnersRequest) -> Result&lt;AddressesUpdatePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .addresses_update(
            &AddressesUpdatePartnersRequest {
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

**type_:** `Option<AddressesUpdatePartnersRequestType>` 
    
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">addresses_delete</a>(request: AddressesDeletePartnersRequest) -> Result&lt;AddressesDeletePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .addresses_delete(
            &AddressesDeletePartnersRequest {
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">addresses_list</a>(request: AddressesListPartnersRequest) -> Result&lt;AddressesListPartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .addresses_list(
            &AddressesListPartnersRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<AddressesListPartnersRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<AddressesListPartnersRequestFilterItem>>` 
    
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">contacts_create</a>(request: ContactsCreatePartnersRequest) -> Result&lt;ContactsCreatePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .contacts_create(
            &ContactsCreatePartnersRequest {
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">contacts_update</a>(request: ContactsUpdatePartnersRequest) -> Result&lt;ContactsUpdatePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .contacts_update(
            &ContactsUpdatePartnersRequest {
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">contacts_delete</a>(request: ContactsDeletePartnersRequest) -> Result&lt;ContactsDeletePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .contacts_delete(
            &ContactsDeletePartnersRequest {
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">contacts_list</a>(request: ContactsListPartnersRequest) -> Result&lt;ContactsListPartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .contacts_list(
            &ContactsListPartnersRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<ContactsListPartnersRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<ContactsListPartnersRequestFilterItem>>` 
    
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">bank_accounts_create</a>(request: BankAccountsCreatePartnersRequest) -> Result&lt;BankAccountsCreatePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .bank_accounts_create(
            &BankAccountsCreatePartnersRequest {
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">bank_accounts_update</a>(request: BankAccountsUpdatePartnersRequest) -> Result&lt;BankAccountsUpdatePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .bank_accounts_update(
            &BankAccountsUpdatePartnersRequest {
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">bank_accounts_delete</a>(request: BankAccountsDeletePartnersRequest) -> Result&lt;BankAccountsDeletePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .bank_accounts_delete(
            &BankAccountsDeletePartnersRequest {
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">bank_accounts_list</a>(request: BankAccountsListPartnersRequest) -> Result&lt;BankAccountsListPartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .bank_accounts_list(
            &BankAccountsListPartnersRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<BankAccountsListPartnersRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<BankAccountsListPartnersRequestFilterItem>>` 
    
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">files_list</a>(request: FilesListPartnersRequest) -> Result&lt;FilesListPartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .files_list(
            &FilesListPartnersRequest {
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">debt_reminders_preview</a>(request: DebtRemindersPreviewPartnersRequest) -> Result&lt;DebtRemindersPreviewPartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .debt_reminders_preview(
            &DebtRemindersPreviewPartnersRequest {
                ..Default::default()
            },
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">debt_reminders_list</a>(request: DebtRemindersListPartnersRequest) -> Result&lt;DebtRemindersListPartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .debt_reminders_list(
            &DebtRemindersListPartnersRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<DebtRemindersListPartnersRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<DebtRemindersListPartnersRequestFilterItem>>` 
    
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">validate_vat</a>(request: ValidateVatPartnersRequest) -> Result&lt;ValidateVatPartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .validate_vat(
            &ValidateVatPartnersRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">vat_reviews_list</a>(request: VatReviewsListPartnersRequest) -> Result&lt;VatReviewsListPartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .vat_reviews_list(
            &VatReviewsListPartnersRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<VatReviewsListPartnersRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<VatReviewsListPartnersRequestFilterItem>>` 
    
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">vat_reviews_resolve</a>(request: VatReviewsResolvePartnersRequest) -> Result&lt;VatReviewsResolvePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .vat_reviews_resolve(
            &VatReviewsResolvePartnersRequest {
                id: "id".to_string(),
                resolution: VatReviewsResolvePartnersRequestResolution::ConfirmedValid,
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

**resolution:** `VatReviewsResolvePartnersRequestResolution` 
    
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">create</a>(request: CreatePartnersRequest) -> Result&lt;CreatePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .create(
            &CreatePartnersRequest {
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

**type_:** `Option<CreatePartnersRequestType>` 
    
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

**address:** `Option<CreatePartnersRequestAddress>` 
    
</dd>
</dl>

<dl>
<dd>

**correspondence_address:** `Option<CreatePartnersRequestCorrespondenceAddress>` 
    
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

**legal_country_class:** `Option<CreatePartnersRequestLegalCountryClass>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">find_or_create</a>(request: FindOrCreatePartnersRequest) -> Result&lt;FindOrCreatePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .find_or_create(
            &FindOrCreatePartnersRequest {
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

**type_:** `Option<FindOrCreatePartnersRequestType>` 
    
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

**address:** `Option<FindOrCreatePartnersRequestAddress>` 
    
</dd>
</dl>

<dl>
<dd>

**correspondence_address:** `Option<FindOrCreatePartnersRequestCorrespondenceAddress>` 
    
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

**legal_country_class:** `Option<FindOrCreatePartnersRequestLegalCountryClass>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">get</a>(request: GetPartnersRequest) -> Result&lt;GetPartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .get(
            &GetPartnersRequest {
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">update</a>(request: UpdatePartnersRequest) -> Result&lt;UpdatePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .update(
            &UpdatePartnersRequest {
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

**type_:** `Option<UpdatePartnersRequestType>` 
    
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

**address:** `Option<Option<UpdatePartnersRequestAddress>>` 
    
</dd>
</dl>

<dl>
<dd>

**correspondence_address:** `Option<Option<UpdatePartnersRequestCorrespondenceAddress>>` 
    
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

**legal_country_class:** `Option<Option<UpdatePartnersRequestLegalCountryClass>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">delete</a>(request: DeletePartnersRequest) -> Result&lt;DeletePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .delete(
            &DeletePartnersRequest {
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">merge</a>(request: MergePartnersRequest) -> Result&lt;MergePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .merge(
            &MergePartnersRequest {
                source_id: "sourceId".to_string(),
                target_id: "targetId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**source_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**target_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">anonymize</a>(request: AnonymizePartnersRequest) -> Result&lt;AnonymizePartnersResponse, ApiError&gt;</code></summary>
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
        .anonymize(
            &AnonymizePartnersRequest {
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">list</a>(request: ListPartnersRequest) -> Result&lt;ListPartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .list(
            &ListPartnersRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<ListPartnersRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<ListPartnersRequestFilterItem>>` 
    
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">groups_create</a>(request: GroupsCreatePartnersRequest) -> Result&lt;GroupsCreatePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .groups_create(
            &GroupsCreatePartnersRequest {
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">groups_update</a>(request: GroupsUpdatePartnersRequest) -> Result&lt;GroupsUpdatePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .groups_update(
            &GroupsUpdatePartnersRequest {
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">groups_delete</a>(request: GroupsDeletePartnersRequest) -> Result&lt;GroupsDeletePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .groups_delete(
            &GroupsDeletePartnersRequest {
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">groups_list</a>(request: GroupsListPartnersRequest) -> Result&lt;GroupsListPartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .groups_list(
            &GroupsListPartnersRequest {
                ..Default::default()
            },
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">statuses_create</a>(request: StatusesCreatePartnersRequest) -> Result&lt;StatusesCreatePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .statuses_create(
            &StatusesCreatePartnersRequest {
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">statuses_update</a>(request: StatusesUpdatePartnersRequest) -> Result&lt;StatusesUpdatePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .statuses_update(
            &StatusesUpdatePartnersRequest {
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">statuses_delete</a>(request: StatusesDeletePartnersRequest) -> Result&lt;StatusesDeletePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .statuses_delete(
            &StatusesDeletePartnersRequest {
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">statuses_list</a>(request: StatusesListPartnersRequest) -> Result&lt;StatusesListPartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .statuses_list(
            &StatusesListPartnersRequest {
                ..Default::default()
            },
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">inquiries_create</a>(request: InquiriesCreatePartnersRequest) -> Result&lt;InquiriesCreatePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .inquiries_create(
            &InquiriesCreatePartnersRequest {
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">inquiries_update</a>(request: InquiriesUpdatePartnersRequest) -> Result&lt;InquiriesUpdatePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .inquiries_update(
            &InquiriesUpdatePartnersRequest {
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

**status:** `Option<InquiriesUpdatePartnersRequestStatus>` 
    
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">inquiries_get</a>(request: InquiriesGetPartnersRequest) -> Result&lt;InquiriesGetPartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .inquiries_get(
            &InquiriesGetPartnersRequest {
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">inquiries_list</a>(request: InquiriesListPartnersRequest) -> Result&lt;InquiriesListPartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .inquiries_list(
            &InquiriesListPartnersRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<InquiriesListPartnersRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<InquiriesListPartnersRequestFilterItem>>` 
    
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

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">credit_check</a>(request: CreditCheckPartnersRequest) -> Result&lt;CreditCheckPartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .credit_check(
            &CreditCheckPartnersRequest {
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

## Leads
<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">create</a>(request: CreateLeadsRequest) -> Result&lt;CreateLeadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .leads
        .create(
            &CreateLeadsRequest {
                name: "name".to_string(),
                contact_name: None,
                email: None,
                phone: None,
                website: None,
                country_code: None,
                source_id: None,
                type_id: None,
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

**type_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<CreateLeadsRequestStatus>` 
    
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

**documents:** `Option<Vec<CreateLeadsRequestDocumentsItem>>` 
    
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

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">get</a>(request: GetLeadsRequest) -> Result&lt;GetLeadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .leads
        .get(
            &GetLeadsRequest {
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

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">update</a>(request: UpdateLeadsRequest) -> Result&lt;UpdateLeadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .leads
        .update(
            &UpdateLeadsRequest {
                id: "id".to_string(),
                name: None,
                contact_name: None,
                email: None,
                phone: None,
                website: None,
                country_code: None,
                source_id: None,
                type_id: None,
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

**type_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<UpdateLeadsRequestStatus>` 
    
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

**documents:** `Option<Vec<UpdateLeadsRequestDocumentsItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">delete</a>(request: DeleteLeadsRequest) -> Result&lt;DeleteLeadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .leads
        .delete(
            &DeleteLeadsRequest {
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

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">list</a>(request: ListLeadsRequest) -> Result&lt;ListLeadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .leads
        .list(
            &ListLeadsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<ListLeadsRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<ListLeadsRequestFilterItem>>` 
    
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

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">notes_create</a>(request: NotesCreateLeadsRequest) -> Result&lt;NotesCreateLeadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .leads
        .notes_create(
            &NotesCreateLeadsRequest {
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

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">notes_delete</a>(request: NotesDeleteLeadsRequest) -> Result&lt;NotesDeleteLeadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .leads
        .notes_delete(
            &NotesDeleteLeadsRequest {
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

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">notes_list</a>(request: NotesListLeadsRequest) -> Result&lt;NotesListLeadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .leads
        .notes_list(
            &NotesListLeadsRequest {
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

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">files_list</a>(request: FilesListLeadsRequest) -> Result&lt;FilesListLeadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .leads
        .files_list(
            &FilesListLeadsRequest {
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

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">sources_create</a>(request: SourcesCreateLeadsRequest) -> Result&lt;SourcesCreateLeadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .leads
        .sources_create(
            &SourcesCreateLeadsRequest {
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

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">sources_update</a>(request: SourcesUpdateLeadsRequest) -> Result&lt;SourcesUpdateLeadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .leads
        .sources_update(
            &SourcesUpdateLeadsRequest {
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

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">sources_delete</a>(request: SourcesDeleteLeadsRequest) -> Result&lt;SourcesDeleteLeadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .leads
        .sources_delete(
            &SourcesDeleteLeadsRequest {
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

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">sources_list</a>(request: SourcesListLeadsRequest) -> Result&lt;SourcesListLeadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .leads
        .sources_list(
            &SourcesListLeadsRequest {
                ..Default::default()
            },
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

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">sources_options</a>(request: SourcesOptionsLeadsRequest) -> Result&lt;SourcesOptionsLeadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .leads
        .sources_options(
            &SourcesOptionsLeadsRequest {
                ..Default::default()
            },
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

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">types_create</a>(request: TypesCreateLeadsRequest) -> Result&lt;TypesCreateLeadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .leads
        .types_create(
            &TypesCreateLeadsRequest {
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

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">types_update</a>(request: TypesUpdateLeadsRequest) -> Result&lt;TypesUpdateLeadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .leads
        .types_update(
            &TypesUpdateLeadsRequest {
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

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">types_delete</a>(request: TypesDeleteLeadsRequest) -> Result&lt;TypesDeleteLeadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .leads
        .types_delete(
            &TypesDeleteLeadsRequest {
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

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">types_list</a>(request: TypesListLeadsRequest) -> Result&lt;TypesListLeadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .leads
        .types_list(
            &TypesListLeadsRequest {
                ..Default::default()
            },
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

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">types_options</a>(request: TypesOptionsLeadsRequest) -> Result&lt;TypesOptionsLeadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .leads
        .types_options(
            &TypesOptionsLeadsRequest {
                ..Default::default()
            },
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

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">convert</a>(request: ConvertLeadsRequest) -> Result&lt;ConvertLeadsResponse, ApiError&gt;</code></summary>
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
        .leads
        .convert(
            &ConvertLeadsRequest {
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

**partner_type:** `Option<ConvertLeadsRequestPartnerType>` 
    
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

## catalog
<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">items_create</a>(request: ItemsCreateCatalogRequest) -> Result&lt;ItemsCreateCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .items_create(
            &ItemsCreateCatalogRequest {
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

**type_:** `Option<ItemsCreateCatalogRequestType>` 
    
</dd>
</dl>

<dl>
<dd>

**tracking:** `Option<ItemsCreateCatalogRequestTracking>` 
    
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

**translations:** `Option<std::collections::HashMap<String, ItemsCreateCatalogRequestTranslationsValue>>` 
    
</dd>
</dl>

<dl>
<dd>

**components:** `Option<Vec<ItemsCreateCatalogRequestComponentsItem>>` 
    
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">items_get</a>(request: ItemsGetCatalogRequest) -> Result&lt;ItemsGetCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .items_get(
            &ItemsGetCatalogRequest {
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">items_update</a>(request: ItemsUpdateCatalogRequest) -> Result&lt;ItemsUpdateCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .items_update(
            &ItemsUpdateCatalogRequest {
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

**type_:** `Option<ItemsUpdateCatalogRequestType>` 
    
</dd>
</dl>

<dl>
<dd>

**tracking:** `Option<ItemsUpdateCatalogRequestTracking>` 
    
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

**translations:** `Option<Option<std::collections::HashMap<String, Option<ItemsUpdateCatalogRequestTranslationsValue>>>>` 
    
</dd>
</dl>

<dl>
<dd>

**components:** `Option<Vec<ItemsUpdateCatalogRequestComponentsItem>>` 
    
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">items_delete</a>(request: ItemsDeleteCatalogRequest) -> Result&lt;ItemsDeleteCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .items_delete(
            &ItemsDeleteCatalogRequest {
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">items_list</a>(request: ItemsListCatalogRequest) -> Result&lt;ItemsListCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .items_list(
            &ItemsListCatalogRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<ItemsListCatalogRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<ItemsListCatalogRequestFilterItem>>` 
    
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">items_files_list</a>(request: ItemsFilesListCatalogRequest) -> Result&lt;ItemsFilesListCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .items_files_list(
            &ItemsFilesListCatalogRequest {
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">items_kinds_create</a>(request: ItemsKindsCreateCatalogRequest) -> Result&lt;ItemsKindsCreateCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .items_kinds_create(
            &ItemsKindsCreateCatalogRequest {
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

**saft_type:** `Option<ItemsKindsCreateCatalogRequestSaftType>` 
    
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">items_kinds_update</a>(request: ItemsKindsUpdateCatalogRequest) -> Result&lt;ItemsKindsUpdateCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .items_kinds_update(
            &ItemsKindsUpdateCatalogRequest {
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

**saft_type:** `Option<ItemsKindsUpdateCatalogRequestSaftType>` 
    
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">items_kinds_delete</a>(request: ItemsKindsDeleteCatalogRequest) -> Result&lt;ItemsKindsDeleteCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .items_kinds_delete(
            &ItemsKindsDeleteCatalogRequest {
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">items_kinds_list</a>(request: ItemsKindsListCatalogRequest) -> Result&lt;ItemsKindsListCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .items_kinds_list(
            &ItemsKindsListCatalogRequest {
                ..Default::default()
            },
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">units_create</a>(request: UnitsCreateCatalogRequest) -> Result&lt;UnitsCreateCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .units_create(
            &UnitsCreateCatalogRequest {
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">units_update</a>(request: UnitsUpdateCatalogRequest) -> Result&lt;UnitsUpdateCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .units_update(
            &UnitsUpdateCatalogRequest {
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">units_delete</a>(request: UnitsDeleteCatalogRequest) -> Result&lt;UnitsDeleteCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .units_delete(
            &UnitsDeleteCatalogRequest {
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">units_list</a>(request: UnitsListCatalogRequest) -> Result&lt;UnitsListCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .units_list(
            &UnitsListCatalogRequest {
                ..Default::default()
            },
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">units_options</a>(request: UnitsOptionsCatalogRequest) -> Result&lt;UnitsOptionsCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .units_options(
            &UnitsOptionsCatalogRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**locale:** `Option<UnitsOptionsCatalogRequestLocale>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">item_groups_create</a>(request: ItemGroupsCreateCatalogRequest) -> Result&lt;ItemGroupsCreateCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .item_groups_create(
            &ItemGroupsCreateCatalogRequest {
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">item_groups_update</a>(request: ItemGroupsUpdateCatalogRequest) -> Result&lt;ItemGroupsUpdateCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .item_groups_update(
            &ItemGroupsUpdateCatalogRequest {
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">item_groups_delete</a>(request: ItemGroupsDeleteCatalogRequest) -> Result&lt;ItemGroupsDeleteCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .item_groups_delete(
            &ItemGroupsDeleteCatalogRequest {
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">item_groups_list</a>(request: ItemGroupsListCatalogRequest) -> Result&lt;ItemGroupsListCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .item_groups_list(
            &ItemGroupsListCatalogRequest {
                ..Default::default()
            },
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">items_suppliers_upsert</a>(request: ItemsSuppliersUpsertCatalogRequest) -> Result&lt;ItemsSuppliersUpsertCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .items_suppliers_upsert(
            &ItemsSuppliersUpsertCatalogRequest {
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">items_suppliers_list</a>(request: ItemsSuppliersListCatalogRequest) -> Result&lt;ItemsSuppliersListCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .items_suppliers_list(
            &ItemsSuppliersListCatalogRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">items_suppliers_delete</a>(request: ItemsSuppliersDeleteCatalogRequest) -> Result&lt;ItemsSuppliersDeleteCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .items_suppliers_delete(
            &ItemsSuppliersDeleteCatalogRequest {
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">price_lists_create</a>(request: PriceListsCreateCatalogRequest) -> Result&lt;PriceListsCreateCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .price_lists_create(
            &PriceListsCreateCatalogRequest {
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">price_lists_update</a>(request: PriceListsUpdateCatalogRequest) -> Result&lt;PriceListsUpdateCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .price_lists_update(
            &PriceListsUpdateCatalogRequest {
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">price_lists_list</a>(request: PriceListsListCatalogRequest) -> Result&lt;PriceListsListCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .price_lists_list(
            &PriceListsListCatalogRequest {
                ..Default::default()
            },
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">price_lists_items_set</a>(request: PriceListsItemsSetCatalogRequest) -> Result&lt;PriceListsItemsSetCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .price_lists_items_set(
            &PriceListsItemsSetCatalogRequest {
                price_list_id: "priceListId".to_string(),
                items: vec![PriceListsItemsSetCatalogRequestItemsItem {
                    item_id: "itemId".to_string(),
                    unit_price_excl_vat: "121.0000".to_string(),
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

**items:** `Vec<PriceListsItemsSetCatalogRequestItemsItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">price_lists_items_list</a>(request: PriceListsItemsListCatalogRequest) -> Result&lt;PriceListsItemsListCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .price_lists_items_list(
            &PriceListsItemsListCatalogRequest {
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

<details><summary><code>client.catalog.<a href="/src/api/resources/catalog/client.rs">price_lists_items_delete</a>(request: PriceListsItemsDeleteCatalogRequest) -> Result&lt;PriceListsItemsDeleteCatalogResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .price_lists_items_delete(
            &PriceListsItemsDeleteCatalogRequest {
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

## sales
<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">invoices_create</a>(request: InvoicesCreateSalesRequest) -> Result&lt;InvoicesCreateSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_create(
            &InvoicesCreateSalesRequest {
                partner_id: "partnerId".to_string(),
                lines: vec![InvoicesCreateSalesRequestLinesItem {
                    ..Default::default()
                }],
                r#type: None,
                currency: None,
                issue_date: None,
                due_date: None,
                credited_invoice_id: None,
                credited_invoice_reference: None,
                credited_invoice_date: None,
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

**type_:** `Option<InvoicesCreateSalesRequestType>` 
    
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

**credited_invoice_reference:** `Option<String>` — Number of an original invoice issued outside Nordlet; give it with creditedInvoiceDate
    
</dd>
</dl>

<dl>
<dd>

**credited_invoice_date:** `Option<String>` — Issue date of the original invoice issued outside Nordlet
    
</dd>
</dl>

<dl>
<dd>

**agreement_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_scheme:** `Option<InvoicesCreateSalesRequestVatScheme>` 
    
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

**lines:** `Vec<InvoicesCreateSalesRequestLinesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">invoices_get</a>(request: InvoicesGetSalesRequest) -> Result&lt;InvoicesGetSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_get(
            &InvoicesGetSalesRequest {
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">invoices_pdf</a>(request: InvoicesPdfSalesRequest) -> Result&lt;InvoicesPdfSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_pdf(
            &InvoicesPdfSalesRequest {
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

**locale:** `Option<InvoicesPdfSalesRequestLocale>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">invoices_send</a>(request: InvoicesSendSalesRequest) -> Result&lt;InvoicesSendSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_send(
            &InvoicesSendSalesRequest {
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

**locale:** `Option<InvoicesSendSalesRequestLocale>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">invoices_peppol_xml</a>(request: InvoicesPeppolXmlSalesRequest) -> Result&lt;InvoicesPeppolXmlSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_peppol_xml(
            &InvoicesPeppolXMLSalesRequest {
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">invoices_peppol_send</a>(request: InvoicesPeppolSendSalesRequest) -> Result&lt;InvoicesPeppolSendSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Send an issued invoice or credit note to the customer over Peppol through the company's own access point (Settings → Compliance → EU; Nordlet supports Recommand, Storecove and e-invoice.be). Without one the call is refused with 422 and the document can only be downloaded with `sales/invoices/peppol-xml`. `status` is `pending` until the receiving access point confirms, then `delivered`; `failed` and `rejected` come with `detail`, and the invoice can then be sent again. Later changes arrive through the access point's webhook and are announced as `sale_invoice.peppol_delivered`, `sale_invoice.peppol_rejected` and `sale_invoice.peppol_failed`.
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
        .invoices_peppol_send(
            &InvoicesPeppolSendSalesRequest {
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">invoices_peppol_status</a>(request: InvoicesPeppolStatusSalesRequest) -> Result&lt;InvoicesPeppolStatusSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Ask the company's Peppol access point what happened to an invoice sent with `sales/invoices/peppol-send`, and store the answer: `pending`, `delivered` (the receiving access point confirmed it), `rejected` (the receiver refused it, see `detail`) or `failed` (it could not be delivered, see `detail`). The access point's webhook updates the same fields without this call. Storecove has no call for the status of a sent document, so for a Storecove access point this answers 422 and the status comes only from its webhook.
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
        .invoices_peppol_status(
            &InvoicesPeppolStatusSalesRequest {
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">invoices_einvoice_xml</a>(request: InvoicesEinvoiceXmlSalesRequest) -> Result&lt;InvoicesEinvoiceXmlSalesResponse, ApiError&gt;</code></summary>
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
        .invoices_einvoice_xml(
            &InvoicesEinvoiceXMLSalesRequest {
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">invoices_einvoice_send</a>(request: InvoicesEinvoiceSendSalesRequest) -> Result&lt;InvoicesEinvoiceSendSalesResponse, ApiError&gt;</code></summary>
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
        .invoices_einvoice_send(
            &InvoicesEinvoiceSendSalesRequest {
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">invoices_einvoice_status</a>(request: InvoicesEinvoiceStatusSalesRequest) -> Result&lt;InvoicesEinvoiceStatusSalesResponse, ApiError&gt;</code></summary>
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
        .invoices_einvoice_status(
            &InvoicesEinvoiceStatusSalesRequest {
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">invoices_update</a>(request: InvoicesUpdateSalesRequest) -> Result&lt;InvoicesUpdateSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_update(
            &InvoicesUpdateSalesRequest {
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

**vat_scheme:** `Option<Option<InvoicesUpdateSalesRequestVatScheme>>` 
    
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

**lines:** `Option<Vec<InvoicesUpdateSalesRequestLinesItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">invoices_delete</a>(request: InvoicesDeleteSalesRequest) -> Result&lt;InvoicesDeleteSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_delete(
            &InvoicesDeleteSalesRequest {
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">invoices_issue</a>(request: InvoicesIssueSalesRequest) -> Result&lt;InvoicesIssueSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_issue(
            &InvoicesIssueSalesRequest {
                id: "id".to_string(),
                series: None,
                issue_date: None,
                warehouse_id: None,
                return_to_stock: None,
            },
            None,
        )
        .await;
}
```
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

<dl>
<dd>

**return_to_stock:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">invoices_lock</a>(request: InvoicesLockSalesRequest) -> Result&lt;InvoicesLockSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_lock(
            &InvoicesLockSalesRequest {
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">invoices_unlock</a>(request: InvoicesUnlockSalesRequest) -> Result&lt;InvoicesUnlockSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_unlock(
            &InvoicesUnlockSalesRequest {
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">invoices_payment_link</a>(request: InvoicesPaymentLinkSalesRequest) -> Result&lt;InvoicesPaymentLinkSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_payment_link(
            &InvoicesPaymentLinkSalesRequest {
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">invoices_payment_settings_get</a>(request: InvoicesPaymentSettingsGetSalesRequest) -> Result&lt;InvoicesPaymentSettingsGetSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_payment_settings_get(
            &InvoicesPaymentSettingsGetSalesRequest {
                ..Default::default()
            },
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">invoices_payment_settings_update</a>(request: InvoicesPaymentSettingsUpdateSalesRequest) -> Result&lt;InvoicesPaymentSettingsUpdateSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_payment_settings_update(
            &InvoicesPaymentSettingsUpdateSalesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">recognition_schedules_list</a>(request: RecognitionSchedulesListSalesRequest) -> Result&lt;RecognitionSchedulesListSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .recognition_schedules_list(
            &RecognitionSchedulesListSalesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<RecognitionSchedulesListSalesRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<RecognitionSchedulesListSalesRequestFilterItem>>` 
    
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">invoices_apply_advance</a>(request: InvoicesApplyAdvanceSalesRequest) -> Result&lt;InvoicesApplyAdvanceSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_apply_advance(
            &InvoicesApplyAdvanceSalesRequest {
                advance_id: "advanceId".to_string(),
                invoice_id: "invoiceId".to_string(),
                date: None,
                amount: None,
            },
            None,
        )
        .await;
}
```
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

<dl>
<dd>

**amount:** `Option<String>` — Gross amount of the advance to apply; defaults to the unapplied advance or the unpaid balance of the invoice, whichever is smaller
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">invoices_list</a>(request: InvoicesListSalesRequest) -> Result&lt;InvoicesListSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_list(
            &InvoicesListSalesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<InvoicesListSalesRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<InvoicesListSalesRequestFilterItem>>` 
    
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">acts_create</a>(request: ActsCreateSalesRequest) -> Result&lt;ActsCreateSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .acts_create(
            &ActsCreateSalesRequest {
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

**type_:** `Option<ActsCreateSalesRequestType>` 
    
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

**lines:** `Option<Vec<ActsCreateSalesRequestLinesItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">acts_update</a>(request: ActsUpdateSalesRequest) -> Result&lt;ActsUpdateSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .acts_update(
            &ActsUpdateSalesRequest {
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

**type_:** `Option<ActsUpdateSalesRequestType>` 
    
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

**lines:** `Option<Vec<ActsUpdateSalesRequestLinesItem>>` 
    
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">acts_issue</a>(request: ActsIssueSalesRequest) -> Result&lt;ActsIssueSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .acts_issue(
            &ActsIssueSalesRequest {
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">acts_cancel</a>(request: ActsCancelSalesRequest) -> Result&lt;ActsCancelSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .acts_cancel(
            &ActsCancelSalesRequest {
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">acts_get</a>(request: ActsGetSalesRequest) -> Result&lt;ActsGetSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .acts_get(
            &ActsGetSalesRequest {
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">acts_list</a>(request: ActsListSalesRequest) -> Result&lt;ActsListSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .acts_list(
            &ActsListSalesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<ActsListSalesRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<ActsListSalesRequestFilterItem>>` 
    
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">acts_pdf</a>(request: ActsPdfSalesRequest) -> Result&lt;ActsPdfSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .acts_pdf(
            &ActsPdfSalesRequest {
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

**locale:** `Option<ActsPdfSalesRequestLocale>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">recognition_compute</a>(request: RecognitionComputeSalesRequest) -> Result&lt;RecognitionComputeSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .recognition_compute(
            &RecognitionComputeSalesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">recognition_run</a>(request: RecognitionRunSalesRequest) -> Result&lt;RecognitionRunSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .recognition_run(
            &RecognitionRunSalesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">recognition_progress</a>(request: RecognitionProgressSalesRequest) -> Result&lt;RecognitionProgressSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .recognition_progress(
            &RecognitionProgressSalesRequest {
                invoice_line_id: "invoiceLineId".to_string(),
                percent_complete: "121.00".to_string(),
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">recognition_modify</a>(request: RecognitionModifySalesRequest) -> Result&lt;RecognitionModifySalesResponse, ApiError&gt;</code></summary>
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
        .recognition_modify(
            &RecognitionModifySalesRequest {
                invoice_line_id: "invoiceLineId".to_string(),
                approach: RecognitionModifySalesRequestApproach::Prospective,
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

**approach:** `RecognitionModifySalesRequestApproach` 
    
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

**new_milestones:** `Option<Vec<RecognitionModifySalesRequestNewMilestonesItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">recognition_runs_list</a>(request: RecognitionRunsListSalesRequest) -> Result&lt;RecognitionRunsListSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .recognition_runs_list(
            &RecognitionRunsListSalesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<RecognitionRunsListSalesRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<RecognitionRunsListSalesRequestFilterItem>>` 
    
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">recognition_summary</a>(request: RecognitionSummarySalesRequest) -> Result&lt;RecognitionSummarySalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .recognition_summary(
            &RecognitionSummarySalesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">refund_liability_list</a>(request: RefundLiabilityListSalesRequest) -> Result&lt;RefundLiabilityListSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .refund_liability_list(
            &RefundLiabilityListSalesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<RefundLiabilityListSalesRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<RefundLiabilityListSalesRequestFilterItem>>` 
    
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

<details><summary><code>client.sales.<a href="/src/api/resources/sales/client.rs">refund_liability_true_up</a>(request: RefundLiabilityTrueUpSalesRequest) -> Result&lt;RefundLiabilityTrueUpSalesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .refund_liability_true_up(
            &RefundLiabilityTrueUpSalesRequest {
                invoice_id: "invoiceId".to_string(),
                estimated_total: "121.0000".to_string(),
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

## OperationTypes
<details><summary><code>client.operation_types.<a href="/src/api/resources/operation_types/client.rs">create</a>(request: CreateOperationTypesRequest) -> Result&lt;CreateOperationTypesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .operation_types
        .create(
            &CreateOperationTypesRequest {
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

**invoice_type:** `Option<Option<CreateOperationTypesRequestInvoiceType>>` 
    
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

<details><summary><code>client.operation_types.<a href="/src/api/resources/operation_types/client.rs">update</a>(request: UpdateOperationTypesRequest) -> Result&lt;UpdateOperationTypesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .operation_types
        .update(
            &UpdateOperationTypesRequest {
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

**invoice_type:** `Option<Option<UpdateOperationTypesRequestInvoiceType>>` 
    
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

<details><summary><code>client.operation_types.<a href="/src/api/resources/operation_types/client.rs">get</a>(request: GetOperationTypesRequest) -> Result&lt;GetOperationTypesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .operation_types
        .get(
            &GetOperationTypesRequest {
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

<details><summary><code>client.operation_types.<a href="/src/api/resources/operation_types/client.rs">delete</a>(request: DeleteOperationTypesRequest) -> Result&lt;DeleteOperationTypesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .operation_types
        .delete(
            &DeleteOperationTypesRequest {
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

<details><summary><code>client.operation_types.<a href="/src/api/resources/operation_types/client.rs">list</a>(request: ListOperationTypesRequest) -> Result&lt;ListOperationTypesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .operation_types
        .list(
            &ListOperationTypesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<ListOperationTypesRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<ListOperationTypesRequestFilterItem>>` 
    
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

## DocumentSeries
<details><summary><code>client.document_series.<a href="/src/api/resources/document_series/client.rs">create</a>(request: CreateDocumentSeriesRequest) -> Result&lt;CreateDocumentSeriesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .document_series
        .create(
            &CreateDocumentSeriesRequest {
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

**document_type:** `Option<CreateDocumentSeriesRequestDocumentType>` 
    
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

<details><summary><code>client.document_series.<a href="/src/api/resources/document_series/client.rs">update</a>(request: UpdateDocumentSeriesRequest) -> Result&lt;UpdateDocumentSeriesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .document_series
        .update(
            &UpdateDocumentSeriesRequest {
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

**document_type:** `Option<UpdateDocumentSeriesRequestDocumentType>` 
    
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

<details><summary><code>client.document_series.<a href="/src/api/resources/document_series/client.rs">get</a>(request: GetDocumentSeriesRequest) -> Result&lt;GetDocumentSeriesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .document_series
        .get(
            &GetDocumentSeriesRequest {
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

<details><summary><code>client.document_series.<a href="/src/api/resources/document_series/client.rs">delete</a>(request: DeleteDocumentSeriesRequest) -> Result&lt;DeleteDocumentSeriesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .document_series
        .delete(
            &DeleteDocumentSeriesRequest {
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

<details><summary><code>client.document_series.<a href="/src/api/resources/document_series/client.rs">list</a>(request: ListDocumentSeriesRequest) -> Result&lt;ListDocumentSeriesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .document_series
        .list(
            &ListDocumentSeriesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<ListDocumentSeriesRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<ListDocumentSeriesRequestFilterItem>>` 
    
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

## purchases
<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">invoices_create</a>(request: InvoicesCreatePurchasesRequest) -> Result&lt;InvoicesCreatePurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_create(
            &InvoicesCreatePurchasesRequest {
                partner_id: "partnerId".to_string(),
                document_number: "documentNumber".to_string(),
                document_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                lines: vec![InvoicesCreatePurchasesRequestLinesItem {
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

**type_:** `Option<InvoicesCreatePurchasesRequestType>` 
    
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

**lines:** `Vec<InvoicesCreatePurchasesRequestLinesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">invoices_get</a>(request: InvoicesGetPurchasesRequest) -> Result&lt;InvoicesGetPurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_get(
            &InvoicesGetPurchasesRequest {
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

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">invoices_update</a>(request: InvoicesUpdatePurchasesRequest) -> Result&lt;InvoicesUpdatePurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_update(
            &InvoicesUpdatePurchasesRequest {
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

**lines:** `Option<Vec<InvoicesUpdatePurchasesRequestLinesItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">invoices_delete</a>(request: InvoicesDeletePurchasesRequest) -> Result&lt;InvoicesDeletePurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_delete(
            &InvoicesDeletePurchasesRequest {
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

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">invoices_register</a>(request: InvoicesRegisterPurchasesRequest) -> Result&lt;InvoicesRegisterPurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_register(
            &InvoicesRegisterPurchasesRequest {
                id: "id".to_string(),
                registration_date: None,
                warehouse_id: None,
                return_from_stock: None,
            },
            None,
        )
        .await;
}
```
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

<dl>
<dd>

**return_from_stock:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">deferrals_list</a>(request: DeferralsListPurchasesRequest) -> Result&lt;DeferralsListPurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .deferrals_list(
            &DeferralsListPurchasesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<DeferralsListPurchasesRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<DeferralsListPurchasesRequestFilterItem>>` 
    
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

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">deferrals_post</a>(request: DeferralsPostPurchasesRequest) -> Result&lt;DeferralsPostPurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .deferrals_post(
            &DeferralsPostPurchasesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">invoices_list</a>(request: InvoicesListPurchasesRequest) -> Result&lt;InvoicesListPurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_list(
            &InvoicesListPurchasesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<InvoicesListPurchasesRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<InvoicesListPurchasesRequestFilterItem>>` 
    
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

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">orders_create</a>(request: OrdersCreatePurchasesRequest) -> Result&lt;OrdersCreatePurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_create(
            &OrdersCreatePurchasesRequest {
                partner_id: "partnerId".to_string(),
                order_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                lines: vec![OrdersCreatePurchasesRequestLinesItem {
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

**lines:** `Vec<OrdersCreatePurchasesRequestLinesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">orders_update</a>(request: OrdersUpdatePurchasesRequest) -> Result&lt;OrdersUpdatePurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_update(
            &OrdersUpdatePurchasesRequest {
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

**lines:** `Option<Vec<OrdersUpdatePurchasesRequestLinesItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">orders_get</a>(request: OrdersGetPurchasesRequest) -> Result&lt;OrdersGetPurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_get(
            &OrdersGetPurchasesRequest {
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

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">orders_list</a>(request: OrdersListPurchasesRequest) -> Result&lt;OrdersListPurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_list(
            &OrdersListPurchasesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<OrdersListPurchasesRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<OrdersListPurchasesRequestFilterItem>>` 
    
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

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">orders_submit</a>(request: OrdersSubmitPurchasesRequest) -> Result&lt;OrdersSubmitPurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_submit(
            &OrdersSubmitPurchasesRequest {
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

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">orders_approve</a>(request: OrdersApprovePurchasesRequest) -> Result&lt;OrdersApprovePurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_approve(
            &OrdersApprovePurchasesRequest {
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

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">orders_reject</a>(request: OrdersRejectPurchasesRequest) -> Result&lt;OrdersRejectPurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_reject(
            &OrdersRejectPurchasesRequest {
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

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">orders_cancel</a>(request: OrdersCancelPurchasesRequest) -> Result&lt;OrdersCancelPurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_cancel(
            &OrdersCancelPurchasesRequest {
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

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">orders_close</a>(request: OrdersClosePurchasesRequest) -> Result&lt;OrdersClosePurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_close(
            &OrdersClosePurchasesRequest {
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

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">orders_delete</a>(request: OrdersDeletePurchasesRequest) -> Result&lt;OrdersDeletePurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_delete(
            &OrdersDeletePurchasesRequest {
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

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">receipts_create</a>(request: ReceiptsCreatePurchasesRequest) -> Result&lt;ReceiptsCreatePurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .receipts_create(
            &ReceiptsCreatePurchasesRequest {
                order_id: "orderId".to_string(),
                receipt_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                lines: vec![ReceiptsCreatePurchasesRequestLinesItem {
                    order_line_id: "orderLineId".to_string(),
                    quantity: "121.0000".to_string(),
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

**lines:** `Vec<ReceiptsCreatePurchasesRequestLinesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">receipts_get</a>(request: ReceiptsGetPurchasesRequest) -> Result&lt;ReceiptsGetPurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .receipts_get(
            &ReceiptsGetPurchasesRequest {
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

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">receipts_list</a>(request: ReceiptsListPurchasesRequest) -> Result&lt;ReceiptsListPurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .receipts_list(
            &ReceiptsListPurchasesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<ReceiptsListPurchasesRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<ReceiptsListPurchasesRequestFilterItem>>` 
    
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

<details><summary><code>client.purchases.<a href="/src/api/resources/purchases/client.rs">invoices_match</a>(request: InvoicesMatchPurchasesRequest) -> Result&lt;InvoicesMatchPurchasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invoices_match(
            &InvoicesMatchPurchasesRequest {
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

## capture
<details><summary><code>client.capture.<a href="/src/api/resources/capture/client.rs">settings_get</a>(request: SettingsGetCaptureRequest) -> Result&lt;SettingsGetCaptureResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .settings_get(
            &SettingsGetCaptureRequest {
                ..Default::default()
            },
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

<details><summary><code>client.capture.<a href="/src/api/resources/capture/client.rs">settings_update</a>(request: SettingsUpdateCaptureRequest) -> Result&lt;SettingsUpdateCaptureResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .settings_update(
            &SettingsUpdateCaptureRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.capture.<a href="/src/api/resources/capture/client.rs">settings_regenerate_intake</a>(request: SettingsRegenerateIntakeCaptureRequest) -> Result&lt;SettingsRegenerateIntakeCaptureResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .settings_regenerate_intake(
            &SettingsRegenerateIntakeCaptureRequest {
                ..Default::default()
            },
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

<details><summary><code>client.capture.<a href="/src/api/resources/capture/client.rs">inbound_email</a>(request: InboundEmailCaptureRequest) -> Result&lt;InboundEmailCaptureResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .inbound_email(
            &InboundEmailCaptureRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**to_full:** `Option<Vec<InboundEmailCaptureRequestToFullItem>>` 
    
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

**postmark_attachments:** `Option<Vec<InboundEmailCaptureRequestAttachmentsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**to:** `Option<InboundEmailCaptureRequestTo>` 
    
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

**attachments:** `Option<Vec<InboundEmailCaptureRequestAttachmentsItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.capture.<a href="/src/api/resources/capture/client.rs">documents_upload</a>(request: DocumentsUploadCaptureRequest) -> Result&lt;DocumentsUploadCaptureResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .documents_upload(
            &DocumentsUploadCaptureRequest {
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

<details><summary><code>client.capture.<a href="/src/api/resources/capture/client.rs">documents_extract</a>(request: DocumentsExtractCaptureRequest) -> Result&lt;DocumentsExtractCaptureResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .documents_extract(
            &DocumentsExtractCaptureRequest {
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

<details><summary><code>client.capture.<a href="/src/api/resources/capture/client.rs">documents_get</a>(request: DocumentsGetCaptureRequest) -> Result&lt;DocumentsGetCaptureResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .documents_get(
            &DocumentsGetCaptureRequest {
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

<details><summary><code>client.capture.<a href="/src/api/resources/capture/client.rs">documents_list</a>(request: DocumentsListCaptureRequest) -> Result&lt;DocumentsListCaptureResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .documents_list(
            &DocumentsListCaptureRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<DocumentsListCaptureRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<DocumentsListCaptureRequestFilterItem>>` 
    
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

<details><summary><code>client.capture.<a href="/src/api/resources/capture/client.rs">documents_delete</a>(request: DocumentsDeleteCaptureRequest) -> Result&lt;DocumentsDeleteCaptureResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .documents_delete(
            &DocumentsDeleteCaptureRequest {
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

<details><summary><code>client.capture.<a href="/src/api/resources/capture/client.rs">documents_confirm</a>(request: DocumentsConfirmCaptureRequest) -> Result&lt;DocumentsConfirmCaptureResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates the purchase invoice (or credit note, see `type`) from `lines`. Lines with the opposite sign go in `oppositeLines` and are saved as a second document of the opposite type for the same supplier: a purchase credit note against the new invoice, or a purchase invoice next to the new credit note. It is numbered `oppositeDocumentNumber`, by default the document number followed by "-CR" (credit note) or "-INV" (invoice).
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
        .capture
        .documents_confirm(
            &DocumentsConfirmCaptureRequest {
                id: "id".to_string(),
                document_number: "documentNumber".to_string(),
                document_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                lines: vec![DocumentsConfirmCaptureRequestLinesItem {
                    ..Default::default()
                }],
                partner_id: None,
                new_supplier: None,
                r#type: None,
                due_date: None,
                currency: None,
                notes: None,
                opposite_lines: None,
                opposite_document_number: None,
            },
            None,
        )
        .await;
}
```
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

**new_supplier:** `Option<DocumentsConfirmCaptureRequestNewSupplier>` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<DocumentsConfirmCaptureRequestType>` 
    
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

**lines:** `Vec<DocumentsConfirmCaptureRequestLinesItem>` 
    
</dd>
</dl>

<dl>
<dd>

**opposite_lines:** `Option<Vec<DocumentsConfirmCaptureRequestOppositeLinesItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**opposite_document_number:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## peppol
<details><summary><code>client.peppol.<a href="/src/api/resources/peppol/client.rs">participants_lookup</a>(request: ParticipantsLookupPeppolRequest) -> Result&lt;ParticipantsLookupPeppolResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Look a receiver up on the Peppol network (SML and SMP) and say which Peppol BIS Billing 3.0 documents it accepts. Give `partnerId` to look up a partner by its Peppol ID, VAT code or registration code, or `participantId` as "<scheme>:<identifier>". Works without an access point.
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
        .peppol
        .participants_lookup(
            &ParticipantsLookupPeppolRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**participant_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.peppol.<a href="/src/api/resources/peppol/client.rs">webhooks</a>(provider: WebhooksPeppolRequestProvider, company_id: String) -> Result&lt;WebhooksPeppolResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .peppol
        .webhooks(
            &WebhooksPeppolRequestProvider::Recommand,
            &"companyId".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**provider:** `WebhooksPeppolRequestProvider` 
    
</dd>
</dl>

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

## declarations
<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">lt_intrastat_compute</a>(request: LtIntrastatComputeDeclarationsRequest) -> Result&lt;LtIntrastatComputeDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .lt_intrastat_compute(
            &LtIntrastatComputeDeclarationsRequest {
                year: 1000000,
                month: 1000000,
                flow: LtIntrastatComputeDeclarationsRequestFlow::Arrivals,
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

**flow:** `LtIntrastatComputeDeclarationsRequestFlow` 
    
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

**transport_mode:** `Option<LtIntrastatComputeDeclarationsRequestTransportMode>` 
    
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">lt_ivaz_generate</a>(request: LtIvazGenerateDeclarationsRequest) -> Result&lt;LtIvazGenerateDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .lt_ivaz_generate(
            &LtIvazGenerateDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">lt_intrastat_obligation</a>(request: LtIntrastatObligationDeclarationsRequest) -> Result&lt;LtIntrastatObligationDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .lt_intrastat_obligation(
            &LtIntrastatObligationDeclarationsRequest { year: 1000000 },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">lt_isaf_generate</a>(request: LtIsafGenerateDeclarationsRequest) -> Result&lt;LtIsafGenerateDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .lt_isaf_generate(
            &LtIsafGenerateDeclarationsRequest {
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

**data_type:** `Option<LtIsafGenerateDeclarationsRequestDataType>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">lt_fr0600_compute</a>(request: LtFr0600ComputeDeclarationsRequest) -> Result&lt;LtFr0600ComputeDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .lt_fr0600compute(
            &LtFr0600ComputeDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">lt_gpm313_compute</a>(request: LtGpm313ComputeDeclarationsRequest) -> Result&lt;LtGpm313ComputeDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .lt_gpm313compute(
            &LtGpm313ComputeDeclarationsRequest {
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

**payout_timing:** `Option<LtGpm313ComputeDeclarationsRequestPayoutTiming>` 
    
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">lt_sam_compute</a>(request: LtSamComputeDeclarationsRequest) -> Result&lt;LtSamComputeDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .lt_sam_compute(
            &LtSamComputeDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">lt_sd_generate</a>(request: LtSdGenerateDeclarationsRequest) -> Result&lt;LtSdGenerateDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .lt_sd_generate(
            &LtSdGenerateDeclarationsRequest {
                r#type: LtSdGenerateDeclarationsRequestType::OneSd,
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**type_:** `LtSdGenerateDeclarationsRequestType` 
    
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">lt_saft_generate</a>(request: LtSaftGenerateDeclarationsRequest) -> Result&lt;LtSaftGenerateDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .lt_saft_generate(
            &LtSaftGenerateDeclarationsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

**data_type:** `Option<LtSaftGenerateDeclarationsRequestDataType>` 
    
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">lt_ivaz_amend</a>(request: LtIvazAmendDeclarationsRequest) -> Result&lt;LtIvazAmendDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .lt_ivaz_amend(
            &LtIvazAmendDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">lt_ivaz_cancel</a>(request: LtIvazCancelDeclarationsRequest) -> Result&lt;LtIvazCancelDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .lt_ivaz_cancel(
            &LtIvazCancelDeclarationsRequest {
                entries: vec![LtIvazCancelDeclarationsRequestEntriesItem {
                    waybill_id: "waybillId".to_string(),
                    reason: LtIvazCancelDeclarationsRequestEntriesItemReason::One,
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

**entries:** `Vec<LtIvazCancelDeclarationsRequestEntriesItem>` 
    
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">lt_fr0564_compute</a>(request: LtFr0564ComputeDeclarationsRequest) -> Result&lt;LtFr0564ComputeDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .lt_fr0564compute(
            &LtFr0564ComputeDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">lt_gpm312_compute</a>(request: LtGpm312ComputeDeclarationsRequest) -> Result&lt;LtGpm312ComputeDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .lt_gpm312compute(
            &LtGpm312ComputeDeclarationsRequest {
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

**payout_timing:** `Option<LtGpm312ComputeDeclarationsRequestPayoutTiming>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">lt_pln204_compute</a>(request: LtPln204ComputeDeclarationsRequest) -> Result&lt;LtPln204ComputeDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .lt_pln204compute(&LtPln204ComputeDeclarationsRequest { year: 1000000 }, None)
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">eu_oss_compute</a>(request: EuOssComputeDeclarationsRequest) -> Result&lt;EuOssComputeDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .eu_oss_compute(
            &EuOssComputeDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">eu_ioss_compute</a>(request: EuIossComputeDeclarationsRequest) -> Result&lt;EuIossComputeDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .eu_ioss_compute(
            &EuIossComputeDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">eu_own_goods_transfers_compute</a>(request: EuOwnGoodsTransfersComputeDeclarationsRequest) -> Result&lt;EuOwnGoodsTransfersComputeDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .eu_own_goods_transfers_compute(
            &EuOwnGoodsTransfersComputeDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">eu_digital_reporting_list</a>(request: EuDigitalReportingListDeclarationsRequest) -> Result&lt;EuDigitalReportingListDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .eu_digital_reporting_list(
            &EuDigitalReportingListDeclarationsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">eu_dac7_preview</a>(request: EuDac7PreviewDeclarationsRequest) -> Result&lt;EuDac7PreviewDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Which platform sellers are reportable for the year (Council Directive (EU) 2021/514, Annex V) and why the others are excluded, the data still missing, and how the company files the report in its Member State.
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
        .eu_dac7preview(&EuDac7PreviewDeclarationsRequest { year: 1000000 }, None)
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">eu_dac7_xml</a>(request: EuDac7XmlDeclarationsRequest) -> Result&lt;EuDac7XmlDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .eu_dac7xml(&EuDac7XMLDeclarationsRequest { year: 1000000 }, None)
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">eu_distance_sales_threshold_get</a>(request: EuDistanceSalesThresholdGetDeclarationsRequest) -> Result&lt;EuDistanceSalesThresholdGetDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .eu_distance_sales_threshold_get(
            &EuDistanceSalesThresholdGetDeclarationsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">eu_union_turnover_get</a>(request: EuUnionTurnoverGetDeclarationsRequest) -> Result&lt;EuUnionTurnoverGetDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .eu_union_turnover_get(
            &EuUnionTurnoverGetDeclarationsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">eu_sme_cross_border_report_compute</a>(request: EuSmeCrossBorderReportComputeDeclarationsRequest) -> Result&lt;EuSmeCrossBorderReportComputeDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .eu_sme_cross_border_report_compute(
            &EuSmeCrossBorderReportComputeDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">eu_sme_thresholds_list</a>(request: EuSmeThresholdsListDeclarationsRequest) -> Result&lt;EuSmeThresholdsListDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .eu_sme_thresholds_list(
            &EuSmeThresholdsListDeclarationsRequest {
                ..Default::default()
            },
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">eu_sme_threshold_get</a>(request: EuSmeThresholdGetDeclarationsRequest) -> Result&lt;EuSmeThresholdGetDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .eu_sme_threshold_get(
            &EuSmeThresholdGetDeclarationsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">eu_vat_return_packs_list</a>(request: EuVatReturnPacksListDeclarationsRequest) -> Result&lt;EuVatReturnPacksListDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .eu_vat_return_packs_list(
            &EuVatReturnPacksListDeclarationsRequest {
                ..Default::default()
            },
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">eu_vat_return_compute</a>(request: EuVatReturnComputeDeclarationsRequest) -> Result&lt;EuVatReturnComputeDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .eu_vat_return_compute(
            &EuVatReturnComputeDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">pl_jpk_v7m_generate</a>(request: PlJpkV7MGenerateDeclarationsRequest) -> Result&lt;PlJpkV7MGenerateDeclarationsResponse, ApiError&gt;</code></summary>
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
        .pl_jpk_v7m_generate(
            &PlJpkV7MGenerateDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">pl_vat_ue_generate</a>(request: PlVatUeGenerateDeclarationsRequest) -> Result&lt;PlVatUeGenerateDeclarationsResponse, ApiError&gt;</code></summary>
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
        .pl_vat_ue_generate(
            &PlVatUeGenerateDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">pl_intrastat_generate</a>(request: PlIntrastatGenerateDeclarationsRequest) -> Result&lt;PlIntrastatGenerateDeclarationsResponse, ApiError&gt;</code></summary>
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
        .pl_intrastat_generate(
            &PlIntrastatGenerateDeclarationsRequest {
                year: 1000000,
                month: 1000000,
                flow: PlIntrastatGenerateDeclarationsRequestFlow::Arrivals,
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

**flow:** `PlIntrastatGenerateDeclarationsRequestFlow` 
    
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">pl_ksef_received_list</a>(request: PlKsefReceivedListDeclarationsRequest) -> Result&lt;PlKsefReceivedListDeclarationsResponse, ApiError&gt;</code></summary>
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
        .pl_ksef_received_list(
            &PlKsefReceivedListDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">pl_ksef_received_fetch</a>(request: PlKsefReceivedFetchDeclarationsRequest) -> Result&lt;PlKsefReceivedFetchDeclarationsResponse, ApiError&gt;</code></summary>
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
        .pl_ksef_received_fetch(
            &PlKsefReceivedFetchDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">pl_ksef_receipt</a>(request: PlKsefReceiptDeclarationsRequest) -> Result&lt;PlKsefReceiptDeclarationsResponse, ApiError&gt;</code></summary>
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
        .pl_ksef_receipt(
            &PlKsefReceiptDeclarationsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">tax_adjustments_list</a>(request: TaxAdjustmentsListDeclarationsRequest) -> Result&lt;TaxAdjustmentsListDeclarationsResponse, ApiError&gt;</code></summary>
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
        .tax_adjustments_list(
            &TaxAdjustmentsListDeclarationsRequest { year: 1000000 },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">tax_adjustments_create</a>(request: TaxAdjustmentsCreateDeclarationsRequest) -> Result&lt;TaxAdjustmentsCreateDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .tax_adjustments_create(
            &TaxAdjustmentsCreateDeclarationsRequest {
                year: 1000000,
                kind: TaxAdjustmentsCreateDeclarationsRequestKind::NonDeductible,
                amount: "121.00".to_string(),
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

**kind:** `TaxAdjustmentsCreateDeclarationsRequestKind` 
    
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">tax_adjustments_update</a>(request: TaxAdjustmentsUpdateDeclarationsRequest) -> Result&lt;TaxAdjustmentsUpdateDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .tax_adjustments_update(
            &TaxAdjustmentsUpdateDeclarationsRequest {
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

**kind:** `Option<TaxAdjustmentsUpdateDeclarationsRequestKind>` 
    
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">tax_adjustments_delete</a>(request: TaxAdjustmentsDeleteDeclarationsRequest) -> Result&lt;TaxAdjustmentsDeleteDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .tax_adjustments_delete(
            &TaxAdjustmentsDeleteDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">tax_payments_list</a>(request: TaxPaymentsListDeclarationsRequest) -> Result&lt;TaxPaymentsListDeclarationsResponse, ApiError&gt;</code></summary>
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
        .tax_payments_list(
            &TaxPaymentsListDeclarationsRequest {
                tax: TaxPaymentsListDeclarationsRequestTax::CorporateIncomeTax,
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

**tax:** `TaxPaymentsListDeclarationsRequestTax` 
    
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">tax_payments_create</a>(request: TaxPaymentsCreateDeclarationsRequest) -> Result&lt;TaxPaymentsCreateDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .tax_payments_create(
            &TaxPaymentsCreateDeclarationsRequest {
                tax: TaxPaymentsCreateDeclarationsRequestTax::CorporateIncomeTax,
                year: 1000000,
                kind: TaxPaymentsCreateDeclarationsRequestKind::Advance,
                amount: "121.00".to_string(),
                paid_on: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

**tax:** `TaxPaymentsCreateDeclarationsRequestTax` 
    
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

**kind:** `TaxPaymentsCreateDeclarationsRequestKind` 
    
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">tax_payments_update</a>(request: TaxPaymentsUpdateDeclarationsRequest) -> Result&lt;TaxPaymentsUpdateDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .tax_payments_update(
            &TaxPaymentsUpdateDeclarationsRequest {
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

**kind:** `Option<TaxPaymentsUpdateDeclarationsRequestKind>` 
    
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">tax_payments_delete</a>(request: TaxPaymentsDeleteDeclarationsRequest) -> Result&lt;TaxPaymentsDeleteDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .tax_payments_delete(
            &TaxPaymentsDeleteDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">annual_accounts_get</a>(request: AnnualAccountsGetDeclarationsRequest) -> Result&lt;AnnualAccountsGetDeclarationsResponse, ApiError&gt;</code></summary>
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
        .annual_accounts_get(
            &AnnualAccountsGetDeclarationsRequest { year: 1000000 },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">annual_accounts_set</a>(request: AnnualAccountsSetDeclarationsRequest) -> Result&lt;AnnualAccountsSetDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .annual_accounts_set(
            &AnnualAccountsSetDeclarationsRequest {
                year: 1000000,
                adopted: true,
                date_of_preparation: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">annual_accounts_signatures_create</a>(request: AnnualAccountsSignaturesCreateDeclarationsRequest) -> Result&lt;AnnualAccountsSignaturesCreateDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .annual_accounts_signatures_create(
            &AnnualAccountsSignaturesCreateDeclarationsRequest {
                year: 1000000,
                director_name: "directorName".to_string(),
                director_type:
                    AnnualAccountsSignaturesCreateDeclarationsRequestDirectorType::ManagingCurrent,
                signed: true,
                signed_on: None,
                signed_at: None,
                reason_not_signed: None,
            },
            None,
        )
        .await;
}
```
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

**director_type:** `AnnualAccountsSignaturesCreateDeclarationsRequestDirectorType` 
    
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">annual_accounts_signatures_update</a>(request: AnnualAccountsSignaturesUpdateDeclarationsRequest) -> Result&lt;AnnualAccountsSignaturesUpdateDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .annual_accounts_signatures_update(
            &AnnualAccountsSignaturesUpdateDeclarationsRequest {
                id: "id".to_string(),
                director_name: "directorName".to_string(),
                director_type:
                    AnnualAccountsSignaturesUpdateDeclarationsRequestDirectorType::ManagingCurrent,
                signed: true,
                signed_on: None,
                signed_at: None,
                reason_not_signed: None,
            },
            None,
        )
        .await;
}
```
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

**director_type:** `AnnualAccountsSignaturesUpdateDeclarationsRequestDirectorType` 
    
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">annual_accounts_signatures_delete</a>(request: AnnualAccountsSignaturesDeleteDeclarationsRequest) -> Result&lt;AnnualAccountsSignaturesDeleteDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .annual_accounts_signatures_delete(
            &AnnualAccountsSignaturesDeleteDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">annual_accounts_distributions_create</a>(request: AnnualAccountsDistributionsCreateDeclarationsRequest) -> Result&lt;AnnualAccountsDistributionsCreateDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .annual_accounts_distributions_create(
            &AnnualAccountsDistributionsCreateDeclarationsRequest {
                year: 1000000,
                decided_on: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                kind: AnnualAccountsDistributionsCreateDeclarationsRequestKind::Dividend,
                amount: "121.00".to_string(),
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

**kind:** `AnnualAccountsDistributionsCreateDeclarationsRequestKind` 
    
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">annual_accounts_distributions_update</a>(request: AnnualAccountsDistributionsUpdateDeclarationsRequest) -> Result&lt;AnnualAccountsDistributionsUpdateDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .annual_accounts_distributions_update(
            &AnnualAccountsDistributionsUpdateDeclarationsRequest {
                id: "id".to_string(),
                decided_on: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                kind: AnnualAccountsDistributionsUpdateDeclarationsRequestKind::Dividend,
                amount: "121.00".to_string(),
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

**kind:** `AnnualAccountsDistributionsUpdateDeclarationsRequestKind` 
    
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">annual_accounts_distributions_delete</a>(request: AnnualAccountsDistributionsDeleteDeclarationsRequest) -> Result&lt;AnnualAccountsDistributionsDeleteDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .annual_accounts_distributions_delete(
            &AnnualAccountsDistributionsDeleteDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">annual_accounts_attachments_add</a>(request: AnnualAccountsAttachmentsAddDeclarationsRequest) -> Result&lt;AnnualAccountsAttachmentsAddDeclarationsResponse, ApiError&gt;</code></summary>
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
        .annual_accounts_attachments_add(
            &AnnualAccountsAttachmentsAddDeclarationsRequest {
                year: 1000000,
                kind: AnnualAccountsAttachmentsAddDeclarationsRequestKind::FullReport,
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

**kind:** `AnnualAccountsAttachmentsAddDeclarationsRequestKind` 
    
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">annual_accounts_attachments_delete</a>(request: AnnualAccountsAttachmentsDeleteDeclarationsRequest) -> Result&lt;AnnualAccountsAttachmentsDeleteDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .annual_accounts_attachments_delete(
            &AnnualAccountsAttachmentsDeleteDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">cy_td4_generate</a>(request: CyTd4GenerateDeclarationsRequest) -> Result&lt;CyTd4GenerateDeclarationsResponse, ApiError&gt;</code></summary>
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
        .cy_td4generate(&CyTd4GenerateDeclarationsRequest { year: 1000000 }, None)
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">cy_he32_generate</a>(request: CyHe32GenerateDeclarationsRequest) -> Result&lt;CyHe32GenerateDeclarationsResponse, ApiError&gt;</code></summary>
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
        .cy_he32generate(&CyHe32GenerateDeclarationsRequest { year: 1000000 }, None)
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">de_returns_generate</a>(request: DeReturnsGenerateDeclarationsRequest) -> Result&lt;DeReturnsGenerateDeclarationsResponse, ApiError&gt;</code></summary>
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
        .de_returns_generate(
            &DeReturnsGenerateDeclarationsRequest {
                rule_key: DeReturnsGenerateDeclarationsRequestRuleKey::DeEBilanz,
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

**rule_key:** `DeReturnsGenerateDeclarationsRequestRuleKey` 
    
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">de_return_facts_get</a>(request: DeReturnFactsGetDeclarationsRequest) -> Result&lt;DeReturnFactsGetDeclarationsResponse, ApiError&gt;</code></summary>
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
        .de_return_facts_get(&DeReturnFactsGetDeclarationsRequest { year: 1000000 }, None)
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">de_return_facts_set</a>(request: DeReturnFactsSetDeclarationsRequest) -> Result&lt;DeReturnFactsSetDeclarationsResponse, ApiError&gt;</code></summary>
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
        .de_return_facts_set(
            &DeReturnFactsSetDeclarationsRequest {
                year: 1000000,
                facts: DeReturnFactsSetDeclarationsRequestFacts {
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

**facts:** `DeReturnFactsSetDeclarationsRequestFacts` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">de_deuev_generate</a>(request: DeDeuevGenerateDeclarationsRequest) -> Result&lt;DeDeuevGenerateDeclarationsResponse, ApiError&gt;</code></summary>
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
        .de_deuev_generate(
            &DeDeuevGenerateDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">de_beitragsnachweis_generate</a>(request: DeBeitragsnachweisGenerateDeclarationsRequest) -> Result&lt;DeBeitragsnachweisGenerateDeclarationsResponse, ApiError&gt;</code></summary>
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
        .de_beitragsnachweis_generate(
            &DeBeitragsnachweisGenerateDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">dk_selskabsskat_generate</a>(request: DkSelskabsskatGenerateDeclarationsRequest) -> Result&lt;DkSelskabsskatGenerateDeclarationsResponse, ApiError&gt;</code></summary>
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
        .dk_selskabsskat_generate(
            &DkSelskabsskatGenerateDeclarationsRequest { year: 1000000 },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">ee_employment_register_send</a>(request: EeEmploymentRegisterSendDeclarationsRequest) -> Result&lt;EeEmploymentRegisterSendDeclarationsResponse, ApiError&gt;</code></summary>
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
        .ee_employment_register_send(
            &EeEmploymentRegisterSendDeclarationsRequest {
                contract_id: "contractId".to_string(),
                event: EeEmploymentRegisterSendDeclarationsRequestEvent::Start,
            },
            None,
        )
        .await;
}
```
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

**event:** `EeEmploymentRegisterSendDeclarationsRequestEvent` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">es_verifactu_declaracion_responsable</a>(request: EsVerifactuDeclaracionResponsableDeclarationsRequest) -> Result&lt;EsVerifactuDeclaracionResponsableDeclarationsResponse, ApiError&gt;</code></summary>
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
        .es_verifactu_declaracion_responsable(
            &EsVerifactuDeclaracionResponsableDeclarationsRequest {
                ..Default::default()
            },
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">ie_ct1_generate</a>(request: IeCt1GenerateDeclarationsRequest) -> Result&lt;IeCt1GenerateDeclarationsResponse, ApiError&gt;</code></summary>
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
        .ie_ct1generate(&IeCt1GenerateDeclarationsRequest { year: 1000000 }, None)
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">ie_b1_generate</a>(request: IeB1GenerateDeclarationsRequest) -> Result&lt;IeB1GenerateDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Build the working paper for the Form B1 annual return of a financial year - company details, registered office, directors and secretary from Settings → Officers, the members from Settings → Shareholders, the issued share capital and the figures of the financial statements - in the order the CORE screens ask for them. The CRO publishes no file format for the B1, so it is keyed into CORE.
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
        .ie_b1generate(&IeB1GenerateDeclarationsRequest { year: 1000000 }, None)
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">it_sdi_purchase_send</a>(request: ItSdiPurchaseSendDeclarationsRequest) -> Result&lt;ItSdiPurchaseSendDeclarationsResponse, ApiError&gt;</code></summary>
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
        .it_sdi_purchase_send(
            &ItSdiPurchaseSendDeclarationsRequest {
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

**tipo_documento:** `Option<ItSdiPurchaseSendDeclarationsRequestTipoDocumento>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">it_sdi_purchase_preview</a>(request: ItSdiPurchasePreviewDeclarationsRequest) -> Result&lt;ItSdiPurchasePreviewDeclarationsResponse, ApiError&gt;</code></summary>
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
        .it_sdi_purchase_preview(
            &ItSdiPurchasePreviewDeclarationsRequest {
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

**tipo_documento:** `Option<ItSdiPurchasePreviewDeclarationsRequestTipoDocumento>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">lt_saft_send</a>(request: LtSaftSendDeclarationsRequest) -> Result&lt;LtSaftSendDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Upload the SAF-T file to i.SAF-T over the iSAFTUploaderService web service and start its processing. The file, the case reference and the status are kept as a declaration submission (submissionId), whose outcome Nordlet then checks with i.SAF-T. The submission itself is confirmed separately, because after confirmation the file can no longer be corrected. A range and data type already sent is sent again only with amend: true.
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
        .lt_saft_send(
            &LtSaftSendDeclarationsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                data_type: None,
                confirm: None,
                amend: None,
            },
            None,
        )
        .await;
}
```
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

**data_type:** `Option<LtSaftSendDeclarationsRequestDataType>` 
    
</dd>
</dl>

<dl>
<dd>

**confirm:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**amend:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">lt_sd_ffdata</a>(request: LtSdFfdataDeclarationsRequest) -> Result&lt;LtSdFfdataDeclarationsResponse, ApiError&gt;</code></summary>
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
        .lt_sd_ffdata(
            &LtSdFfdataDeclarationsRequest {
                r#type: LtSdFfdataDeclarationsRequestType::OneSd,
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

**type_:** `LtSdFfdataDeclarationsRequestType` 
    
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">lt_pln204_ffdata</a>(request: LtPln204FfdataDeclarationsRequest) -> Result&lt;LtPln204FfdataDeclarationsResponse, ApiError&gt;</code></summary>
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
        .lt_pln204ffdata(&LtPln204FfdataDeclarationsRequest { year: 1000000 }, None)
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">mt_company_tax_generate</a>(request: MtCompanyTaxGenerateDeclarationsRequest) -> Result&lt;MtCompanyTaxGenerateDeclarationsResponse, ApiError&gt;</code></summary>
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
        .mt_company_tax_generate(
            &MtCompanyTaxGenerateDeclarationsRequest { year: 1000000 },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">mt_annual_return_generate</a>(request: MtAnnualReturnGenerateDeclarationsRequest) -> Result&lt;MtAnnualReturnGenerateDeclarationsResponse, ApiError&gt;</code></summary>
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
        .mt_annual_return_generate(
            &MtAnnualReturnGenerateDeclarationsRequest { year: 1000000 },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">pl_jpk_fa_generate</a>(request: PlJpkFaGenerateDeclarationsRequest) -> Result&lt;PlJpkFaGenerateDeclarationsResponse, ApiError&gt;</code></summary>
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
        .pl_jpk_fa_generate(
            &PlJpkFaGenerateDeclarationsRequest {
                date_from: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                date_to: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">pl_jpk_kr_generate</a>(request: PlJpkKrGenerateDeclarationsRequest) -> Result&lt;PlJpkKrGenerateDeclarationsResponse, ApiError&gt;</code></summary>
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
        .pl_jpk_kr_generate(
            &PlJpkKrGenerateDeclarationsRequest {
                date_from: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                date_to: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">pl_jpk_mag_generate</a>(request: PlJpkMagGenerateDeclarationsRequest) -> Result&lt;PlJpkMagGenerateDeclarationsResponse, ApiError&gt;</code></summary>
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
        .pl_jpk_mag_generate(
            &PlJpkMagGenerateDeclarationsRequest {
                date_from: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                date_to: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">pl_pit11_generate</a>(request: PlPit11GenerateDeclarationsRequest) -> Result&lt;PlPit11GenerateDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Generate PIT-11(29) for every person on the payroll of one year: the pay, the deductible costs, the advance withheld and the social and health contributions taken off it. One document per person, because that is how the form is filed, addressed to the tax office of the place of residence of that person (employee field plKodUrzedu); a person without that code is refused with 422.
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
        .pl_pit11generate(&PlPit11GenerateDeclarationsRequest { year: 1000000 }, None)
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">pl_cit8_generate</a>(request: PlCit8GenerateDeclarationsRequest) -> Result&lt;PlCit8GenerateDeclarationsResponse, ApiError&gt;</code></summary>
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
        .pl_cit8generate(&PlCit8GenerateDeclarationsRequest { year: 1000000 }, None)
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">pl_zus_dra_compute</a>(request: PlZusDraComputeDeclarationsRequest) -> Result&lt;PlZusDraComputeDeclarationsResponse, ApiError&gt;</code></summary>
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
        .pl_zus_dra_compute(
            &PlZusDraComputeDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">pl_zus_dra_kedu</a>(request: PlZusDraKeduDeclarationsRequest) -> Result&lt;PlZusDraKeduDeclarationsResponse, ApiError&gt;</code></summary>
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
        .pl_zus_dra_kedu(
            &PlZusDraKeduDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">pl_zus_dra_pdf</a>(request: PlZusDraPdfDeclarationsRequest) -> Result&lt;PlZusDraPdfDeclarationsResponse, ApiError&gt;</code></summary>
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
        .pl_zus_dra_pdf(
            &PlZusDraPdfDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">ro_etransport_build</a>(request: RoEtransportBuildDeclarationsRequest) -> Result&lt;RoEtransportBuildDeclarationsResponse, ApiError&gt;</code></summary>
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
        .ro_etransport_build(
            &RoEtransportBuildDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">ro_etransport_submit</a>(request: RoEtransportSubmitDeclarationsRequest) -> Result&lt;RoEtransportSubmitDeclarationsResponse, ApiError&gt;</code></summary>
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
        .ro_etransport_submit(
            &RoEtransportSubmitDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">ro_etransport_status</a>(request: RoEtransportStatusDeclarationsRequest) -> Result&lt;RoEtransportStatusDeclarationsResponse, ApiError&gt;</code></summary>
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
        .ro_etransport_status(
            &RoEtransportStatusDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">li_lohndeklaration_generate</a>(request: LiLohndeklarationGenerateDeclarationsRequest) -> Result&lt;LiLohndeklarationGenerateDeclarationsResponse, ApiError&gt;</code></summary>
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
        .li_lohndeklaration_generate(
            &LiLohndeklarationGenerateDeclarationsRequest { year: 1000000 },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">li_lohnlisten_generate</a>(request: LiLohnlistenGenerateDeclarationsRequest) -> Result&lt;LiLohnlistenGenerateDeclarationsResponse, ApiError&gt;</code></summary>
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
        .li_lohnlisten_generate(
            &LiLohnlistenGenerateDeclarationsRequest { year: 1000000 },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">configs_list</a>(request: ConfigsListDeclarationsRequest) -> Result&lt;ConfigsListDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .configs_list(
            &ConfigsListDeclarationsRequest {
                ..Default::default()
            },
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">configs_update</a>(request: ConfigsUpdateDeclarationsRequest) -> Result&lt;ConfigsUpdateDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .configs_update(
            &ConfigsUpdateDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">certificates_upload</a>(request: CertificatesUploadDeclarationsRequest) -> Result&lt;CertificatesUploadDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .certificates_upload(
            &CertificatesUploadDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">certificates_list</a>(request: CertificatesListDeclarationsRequest) -> Result&lt;CertificatesListDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .certificates_list(
            &CertificatesListDeclarationsRequest {
                ..Default::default()
            },
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">certificates_delete</a>(request: CertificatesDeleteDeclarationsRequest) -> Result&lt;CertificatesDeleteDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .certificates_delete(
            &CertificatesDeleteDeclarationsRequest {
                system: "system".to_string(),
                field_key: CertificatesDeleteDeclarationsRequestFieldKey::Certificate,
            },
            None,
        )
        .await;
}
```
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

**field_key:** `CertificatesDeleteDeclarationsRequestFieldKey` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">automation_list</a>(request: AutomationListDeclarationsRequest) -> Result&lt;AutomationListDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .automation_list(
            &AutomationListDeclarationsRequest {
                ..Default::default()
            },
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">automation_update</a>(request: AutomationUpdateDeclarationsRequest) -> Result&lt;AutomationUpdateDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .automation_update(
            &AutomationUpdateDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">submissions_retry</a>(request: SubmissionsRetryDeclarationsRequest) -> Result&lt;SubmissionsRetryDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .submissions_retry(
            &SubmissionsRetryDeclarationsRequest {
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">submissions_create</a>(request: SubmissionsCreateDeclarationsRequest) -> Result&lt;SubmissionsCreateDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .submissions_create(
            &SubmissionsCreateDeclarationsRequest {
                obligation: SubmissionsCreateDeclarationsRequestObligation::LtIsaf,
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

**obligation:** `SubmissionsCreateDeclarationsRequestObligation` 
    
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

**data_type:** `Option<SubmissionsCreateDeclarationsRequestDataType>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">submissions_mark</a>(request: SubmissionsMarkDeclarationsRequest) -> Result&lt;SubmissionsMarkDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .submissions_mark(
            &SubmissionsMarkDeclarationsRequest {
                id: "id".to_string(),
                status: SubmissionsMarkDeclarationsRequestStatus::Submitted,
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

**status:** `SubmissionsMarkDeclarationsRequestStatus` 
    
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

<details><summary><code>client.declarations.<a href="/src/api/resources/declarations/client.rs">submissions_list</a>(request: SubmissionsListDeclarationsRequest) -> Result&lt;SubmissionsListDeclarationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .submissions_list(
            &SubmissionsListDeclarationsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<SubmissionsListDeclarationsRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<SubmissionsListDeclarationsRequestFilterItem>>` 
    
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

## ledger
<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">accounts_list</a>(request: AccountsListLedgerRequest) -> Result&lt;AccountsListLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .accounts_list(
            &AccountsListLedgerRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<AccountsListLedgerRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<AccountsListLedgerRequestFilterItem>>` 
    
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">accounts_create</a>(request: AccountsCreateLedgerRequest) -> Result&lt;AccountsCreateLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .accounts_create(
            &AccountsCreateLedgerRequest {
                code: "code".to_string(),
                name: "name".to_string(),
                r#type: AccountsCreateLedgerRequestType::Asset,
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

**translations:** `Option<std::collections::HashMap<String, AccountsCreateLedgerRequestTranslationsValue>>` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `AccountsCreateLedgerRequestType` 
    
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">accounts_update</a>(request: AccountsUpdateLedgerRequest) -> Result&lt;AccountsUpdateLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .accounts_update(
            &AccountsUpdateLedgerRequest {
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

**translations:** `Option<Option<std::collections::HashMap<String, Option<AccountsUpdateLedgerRequestTranslationsValue>>>>` 
    
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">accounts_apply_template</a>(request: AccountsApplyTemplateLedgerRequest) -> Result&lt;AccountsApplyTemplateLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .accounts_apply_template(
            &AccountsApplyTemplateLedgerRequest {
                ..Default::default()
            },
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">accounts_switch_chart</a>(request: AccountsSwitchChartLedgerRequest) -> Result&lt;AccountsSwitchChartLedgerResponse, ApiError&gt;</code></summary>
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
        .accounts_switch_chart(
            &AccountsSwitchChartLedgerRequest {
                ..Default::default()
            },
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">periods_list</a>(request: PeriodsListLedgerRequest) -> Result&lt;PeriodsListLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .periods_list(
            &PeriodsListLedgerRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<PeriodsListLedgerRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PeriodsListLedgerRequestFilterItem>>` 
    
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">periods_lock</a>(request: PeriodsLockLedgerRequest) -> Result&lt;PeriodsLockLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .periods_lock(
            &PeriodsLockLedgerRequest {
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">periods_unlock</a>(request: PeriodsUnlockLedgerRequest) -> Result&lt;PeriodsUnlockLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .periods_unlock(
            &PeriodsUnlockLedgerRequest {
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">journal_transactions_list</a>(request: JournalTransactionsListLedgerRequest) -> Result&lt;JournalTransactionsListLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .journal_transactions_list(
            &JournalTransactionsListLedgerRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<JournalTransactionsListLedgerRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<JournalTransactionsListLedgerRequestFilterItem>>` 
    
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">cost_centers_create</a>(request: CostCentersCreateLedgerRequest) -> Result&lt;CostCentersCreateLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .cost_centers_create(
            &CostCentersCreateLedgerRequest {
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">cost_centers_update</a>(request: CostCentersUpdateLedgerRequest) -> Result&lt;CostCentersUpdateLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .cost_centers_update(
            &CostCentersUpdateLedgerRequest {
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">cost_centers_list</a>(request: CostCentersListLedgerRequest) -> Result&lt;CostCentersListLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .cost_centers_list(
            &CostCentersListLedgerRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<CostCentersListLedgerRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<CostCentersListLedgerRequestFilterItem>>` 
    
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">cost_center_groups_create</a>(request: CostCenterGroupsCreateLedgerRequest) -> Result&lt;CostCenterGroupsCreateLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .cost_center_groups_create(
            &CostCenterGroupsCreateLedgerRequest {
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">cost_center_groups_update</a>(request: CostCenterGroupsUpdateLedgerRequest) -> Result&lt;CostCenterGroupsUpdateLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .cost_center_groups_update(
            &CostCenterGroupsUpdateLedgerRequest {
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">cost_center_groups_delete</a>(request: CostCenterGroupsDeleteLedgerRequest) -> Result&lt;CostCenterGroupsDeleteLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .cost_center_groups_delete(
            &CostCenterGroupsDeleteLedgerRequest {
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">cost_center_groups_list</a>(request: CostCenterGroupsListLedgerRequest) -> Result&lt;CostCenterGroupsListLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .cost_center_groups_list(
            &CostCenterGroupsListLedgerRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<CostCenterGroupsListLedgerRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<CostCenterGroupsListLedgerRequestFilterItem>>` 
    
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">posting_rules_list</a>(request: PostingRulesListLedgerRequest) -> Result&lt;PostingRulesListLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .posting_rules_list(
            &PostingRulesListLedgerRequest {
                ..Default::default()
            },
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">posting_rules_update</a>(request: PostingRulesUpdateLedgerRequest) -> Result&lt;PostingRulesUpdateLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .posting_rules_update(
            &PostingRulesUpdateLedgerRequest {
                rules: vec![PostingRulesUpdateLedgerRequestRulesItem {
                    key: PostingRulesUpdateLedgerRequestRulesItemKey::SalesReceivable,
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

**rules:** `Vec<PostingRulesUpdateLedgerRequestRulesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">owners_create</a>(request: OwnersCreateLedgerRequest) -> Result&lt;OwnersCreateLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .owners_create(
            &OwnersCreateLedgerRequest {
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

**shares_type:** `Option<OwnersCreateLedgerRequestSharesType>` 
    
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

**partner_liability:** `Option<Option<OwnersCreateLedgerRequestPartnerLiability>>` 
    
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

**address:** `Option<OwnersCreateLedgerRequestAddress>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">owners_update</a>(request: OwnersUpdateLedgerRequest) -> Result&lt;OwnersUpdateLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .owners_update(
            &OwnersUpdateLedgerRequest {
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

**shares_type:** `Option<Option<OwnersUpdateLedgerRequestSharesType>>` 
    
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

**partner_liability:** `Option<Option<OwnersUpdateLedgerRequestPartnerLiability>>` 
    
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

**address:** `Option<Option<OwnersUpdateLedgerRequestAddress>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">owners_delete</a>(request: OwnersDeleteLedgerRequest) -> Result&lt;OwnersDeleteLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .owners_delete(
            &OwnersDeleteLedgerRequest {
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">owners_list</a>(request: OwnersListLedgerRequest) -> Result&lt;OwnersListLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .owners_list(
            &OwnersListLedgerRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<OwnersListLedgerRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<OwnersListLedgerRequestFilterItem>>` 
    
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">journal_transactions_get</a>(request: JournalTransactionsGetLedgerRequest) -> Result&lt;JournalTransactionsGetLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .journal_transactions_get(
            &JournalTransactionsGetLedgerRequest {
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">journal_transactions_create</a>(request: JournalTransactionsCreateLedgerRequest) -> Result&lt;JournalTransactionsCreateLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .journal_transactions_create(
            &JournalTransactionsCreateLedgerRequest {
                date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                entries: vec![JournalTransactionsCreateLedgerRequestEntriesItem {
                    account_code: "accountCode".to_string(),
                    ..Default::default()
                }],
                description: None,
                currency: None,
            },
            None,
        )
        .await;
}
```
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

**currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**entries:** `Vec<JournalTransactionsCreateLedgerRequestEntriesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">statement_rows_schemes</a>(request: StatementRowsSchemesLedgerRequest) -> Result&lt;StatementRowsSchemesLedgerResponse, ApiError&gt;</code></summary>
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
        .statement_rows_schemes(
            &StatementRowsSchemesLedgerRequest {
                ..Default::default()
            },
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">statement_rows_list</a>(request: StatementRowsListLedgerRequest) -> Result&lt;StatementRowsListLedgerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .statement_rows_list(
            &StatementRowsListLedgerRequest {
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

<details><summary><code>client.ledger.<a href="/src/api/resources/ledger/client.rs">statement_rows_set</a>(request: StatementRowsSetLedgerRequest) -> Result&lt;StatementRowsSetLedgerResponse, ApiError&gt;</code></summary>
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
        .statement_rows_set(
            &StatementRowsSetLedgerRequest {
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

## Officers
<details><summary><code>client.officers.<a href="/src/api/resources/officers/client.rs">list</a>(request: ListOfficersRequest) -> Result&lt;ListOfficersResponse, ApiError&gt;</code></summary>
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
        .officers
        .list(
            &ListOfficersRequest {
                ..Default::default()
            },
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

<details><summary><code>client.officers.<a href="/src/api/resources/officers/client.rs">create</a>(request: CreateOfficersRequest) -> Result&lt;CreateOfficersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .officers
        .create(
            &CreateOfficersRequest {
                name: "name".to_string(),
                role: CreateOfficersRequestRole::Director,
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

**role:** `CreateOfficersRequestRole` 
    
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

<details><summary><code>client.officers.<a href="/src/api/resources/officers/client.rs">update</a>(request: UpdateOfficersRequest) -> Result&lt;UpdateOfficersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .officers
        .update(
            &UpdateOfficersRequest {
                id: "id".to_string(),
                name: "name".to_string(),
                role: UpdateOfficersRequestRole::Director,
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

**role:** `UpdateOfficersRequestRole` 
    
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

<details><summary><code>client.officers.<a href="/src/api/resources/officers/client.rs">delete</a>(request: DeleteOfficersRequest) -> Result&lt;DeleteOfficersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .officers
        .delete(
            &DeleteOfficersRequest {
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

## PlatformSellers
<details><summary><code>client.platform_sellers.<a href="/src/api/resources/platform_sellers/client.rs">list</a>(request: ListPlatformSellersRequest) -> Result&lt;ListPlatformSellersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Individuals and entities that sell goods, rent out property or transport, or perform personal services through the platform the company operates. The yearly DAC7 report is built from them.
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
        .platform_sellers
        .list(
            &ListPlatformSellersRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<ListPlatformSellersRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<ListPlatformSellersRequestFilterItem>>` 
    
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

<details><summary><code>client.platform_sellers.<a href="/src/api/resources/platform_sellers/client.rs">get</a>(request: GetPlatformSellersRequest) -> Result&lt;GetPlatformSellersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .platform_sellers
        .get(
            &GetPlatformSellersRequest {
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

<details><summary><code>client.platform_sellers.<a href="/src/api/resources/platform_sellers/client.rs">create</a>(request: CreatePlatformSellersRequest) -> Result&lt;CreatePlatformSellersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .platform_sellers
        .create(
            &CreatePlatformSellersRequest {
                kind: CreatePlatformSellersRequestKind::Individual,
                address: CreatePlatformSellersRequestAddress {
                    country_code: "countryCode".to_string(),
                    ..Default::default()
                },
                partner_id: None,
                first_name: None,
                middle_name: None,
                last_name: None,
                entity_name: None,
                tax_residences: None,
                vat_code: None,
                business_registration_number: None,
                birth_date: None,
                birth_city: None,
                birth_country_code: None,
                iban: None,
                account_holder_name: None,
                government_entity: None,
                listed_entity: None,
                permanent_establishments: None,
                activities: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**kind:** `CreatePlatformSellersRequestKind` 
    
</dd>
</dl>

<dl>
<dd>

**partner_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**first_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**middle_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**last_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**entity_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**tax_residences:** `Option<Vec<CreatePlatformSellersRequestTaxResidencesItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**business_registration_number:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**address:** `CreatePlatformSellersRequestAddress` 
    
</dd>
</dl>

<dl>
<dd>

**birth_date:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**birth_city:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**birth_country_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**iban:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**account_holder_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**government_entity:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**listed_entity:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**permanent_establishments:** `Option<Vec<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**activities:** `Option<Vec<CreatePlatformSellersRequestActivitiesItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.platform_sellers.<a href="/src/api/resources/platform_sellers/client.rs">update</a>(request: UpdatePlatformSellersRequest) -> Result&lt;UpdatePlatformSellersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .platform_sellers
        .update(
            &UpdatePlatformSellersRequest {
                id: "id".to_string(),
                kind: UpdatePlatformSellersRequestKind::Individual,
                address: UpdatePlatformSellersRequestAddress {
                    country_code: "countryCode".to_string(),
                    ..Default::default()
                },
                partner_id: None,
                first_name: None,
                middle_name: None,
                last_name: None,
                entity_name: None,
                tax_residences: None,
                vat_code: None,
                business_registration_number: None,
                birth_date: None,
                birth_city: None,
                birth_country_code: None,
                iban: None,
                account_holder_name: None,
                government_entity: None,
                listed_entity: None,
                permanent_establishments: None,
                activities: None,
            },
            None,
        )
        .await;
}
```
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

**kind:** `UpdatePlatformSellersRequestKind` 
    
</dd>
</dl>

<dl>
<dd>

**partner_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**first_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**middle_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**last_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**entity_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**tax_residences:** `Option<Vec<UpdatePlatformSellersRequestTaxResidencesItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**vat_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**business_registration_number:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**address:** `UpdatePlatformSellersRequestAddress` 
    
</dd>
</dl>

<dl>
<dd>

**birth_date:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**birth_city:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**birth_country_code:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**iban:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**account_holder_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**government_entity:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**listed_entity:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**permanent_establishments:** `Option<Vec<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**activities:** `Option<Vec<UpdatePlatformSellersRequestActivitiesItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.platform_sellers.<a href="/src/api/resources/platform_sellers/client.rs">delete</a>(request: DeletePlatformSellersRequest) -> Result&lt;DeletePlatformSellersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .platform_sellers
        .delete(
            &DeletePlatformSellersRequest {
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

## migration
<details><summary><code>client.migration.<a href="/src/api/resources/migration/client.rs">books_validate</a>(request: BooksValidateMigrationRequest) -> Result&lt;BooksValidateMigrationResponse, ApiError&gt;</code></summary>
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
        .books_validate(
            &BooksValidateMigrationRequest {
                cutover_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

**accounts:** `Option<Vec<BooksValidateMigrationRequestAccountsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**partners:** `Option<Vec<BooksValidateMigrationRequestPartnersItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**items:** `Option<Vec<BooksValidateMigrationRequestItemsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**opening_balances:** `Option<BooksValidateMigrationRequestOpeningBalances>` 
    
</dd>
</dl>

<dl>
<dd>

**journal:** `Option<Vec<BooksValidateMigrationRequestJournalItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**open_receivables:** `Option<Vec<BooksValidateMigrationRequestOpenReceivablesItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**open_payables:** `Option<Vec<BooksValidateMigrationRequestOpenPayablesItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**asset_groups:** `Option<Vec<BooksValidateMigrationRequestAssetGroupsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**fixed_assets:** `Option<Vec<BooksValidateMigrationRequestFixedAssetsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**stock:** `Option<Vec<BooksValidateMigrationRequestStockItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.migration.<a href="/src/api/resources/migration/client.rs">books_import</a>(request: BooksImportMigrationRequest) -> Result&lt;BooksImportMigrationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Brings a company over from another system in one call: chart of accounts, partners, items, opening balances (or the full journal history), open customer and supplier invoices, fixed assets with their accumulated depreciation, and stock on hand. The whole package is written in one database transaction - if any row fails, nothing is stored.
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
        .books_import(
            &BooksImportMigrationRequest {
                cutover_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

**accounts:** `Option<Vec<BooksImportMigrationRequestAccountsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**partners:** `Option<Vec<BooksImportMigrationRequestPartnersItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**items:** `Option<Vec<BooksImportMigrationRequestItemsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**opening_balances:** `Option<BooksImportMigrationRequestOpeningBalances>` 
    
</dd>
</dl>

<dl>
<dd>

**journal:** `Option<Vec<BooksImportMigrationRequestJournalItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**open_receivables:** `Option<Vec<BooksImportMigrationRequestOpenReceivablesItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**open_payables:** `Option<Vec<BooksImportMigrationRequestOpenPayablesItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**asset_groups:** `Option<Vec<BooksImportMigrationRequestAssetGroupsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**fixed_assets:** `Option<Vec<BooksImportMigrationRequestFixedAssetsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**stock:** `Option<Vec<BooksImportMigrationRequestStockItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## assets
<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">settings_get</a>(request: SettingsGetAssetsRequest) -> Result&lt;SettingsGetAssetsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .settings_get(
            &SettingsGetAssetsRequest {
                ..Default::default()
            },
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

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">settings_update</a>(request: SettingsUpdateAssetsRequest) -> Result&lt;SettingsUpdateAssetsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .settings_update(
            &SettingsUpdateAssetsRequest {
                auto_depreciation: true,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**auto_depreciation:** `bool` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">groups_create</a>(request: GroupsCreateAssetsRequest) -> Result&lt;GroupsCreateAssetsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .groups_create(
            &GroupsCreateAssetsRequest {
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

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">groups_list</a>(request: GroupsListAssetsRequest) -> Result&lt;GroupsListAssetsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .groups_list(
            &GroupsListAssetsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<GroupsListAssetsRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<GroupsListAssetsRequestFilterItem>>` 
    
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

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">assets_create</a>(request: AssetsCreateAssetsRequest) -> Result&lt;AssetsCreateAssetsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .assets_create(
            &AssetsCreateAssetsRequest {
                group_id: "groupId".to_string(),
                code: "code".to_string(),
                name: "name".to_string(),
                acquisition_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                acquisition_cost: "121.0000".to_string(),
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

**documents:** `Option<Vec<AssetsCreateAssetsRequestDocumentsItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">assets_update</a>(request: AssetsUpdateAssetsRequest) -> Result&lt;AssetsUpdateAssetsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .assets_update(
            &AssetsUpdateAssetsRequest {
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

**documents:** `Option<Vec<AssetsUpdateAssetsRequestDocumentsItem>>` 
    
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

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">assets_input_vat</a>(request: AssetsInputVatAssetsRequest) -> Result&lt;AssetsInputVatAssetsResponse, ApiError&gt;</code></summary>
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
    client
        .assets
        .assets_input_vat(
            &AssetsInputVatAssetsRequest {
                id: "id".to_string(),
                input_vat_real_estate: true,
                input_vat_use_changes: vec![AssetsInputVatAssetsRequestInputVatUseChangesItem {
                    year: 1000000,
                    percent: "121.00".to_string(),
                    reason: AssetsInputVatAssetsRequestInputVatUseChangesItemReason::UseChange,
                }],
                input_vat_amount: None,
                input_vat_first_use_date: None,
                input_vat_deductible_percent: None,
            },
            None,
        )
        .await;
}
```
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

**input_vat_use_changes:** `Vec<AssetsInputVatAssetsRequestInputVatUseChangesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">assets_get</a>(request: AssetsGetAssetsRequest) -> Result&lt;AssetsGetAssetsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .assets_get(
            &AssetsGetAssetsRequest {
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

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">assets_list</a>(request: AssetsListAssetsRequest) -> Result&lt;AssetsListAssetsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .assets_list(
            &AssetsListAssetsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<AssetsListAssetsRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<AssetsListAssetsRequestFilterItem>>` 
    
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

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">assets_modernize</a>(request: AssetsModernizeAssetsRequest) -> Result&lt;AssetsModernizeAssetsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .assets_modernize(
            &AssetsModernizeAssetsRequest {
                id: "id".to_string(),
                date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                amount: "121.0000".to_string(),
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

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">assets_dispose</a>(request: AssetsDisposeAssetsRequest) -> Result&lt;AssetsDisposeAssetsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Dispose of a fixed asset (sold, scrapped or written off). Removes its cost and accumulated depreciation, books the net book value as a disposal loss and the proceeds as a disposal gain (posting rules assets.disposalLoss, assets.disposalGain, assets.disposalProceeds), and stops its depreciation. Depreciation must be posted for every month before the disposal month.
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
        .assets
        .assets_dispose(
            &AssetsDisposeAssetsRequest {
                id: "id".to_string(),
                date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                reason: AssetsDisposeAssetsRequestReason::Sold,
                proceeds: None,
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

**reason:** `AssetsDisposeAssetsRequestReason` 
    
</dd>
</dl>

<dl>
<dd>

**proceeds:** `Option<String>` — Sale price excluding VAT; 0 when scrapped or written off
    
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

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">depreciation_preview</a>(request: DepreciationPreviewAssetsRequest) -> Result&lt;DepreciationPreviewAssetsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .depreciation_preview(
            &DepreciationPreviewAssetsRequest {
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

<details><summary><code>client.assets.<a href="/src/api/resources/assets/client.rs">depreciation_post</a>(request: DepreciationPostAssetsRequest) -> Result&lt;DepreciationPostAssetsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .depreciation_post(
            &DepreciationPostAssetsRequest {
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

## hr
<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">positions_create</a>(request: PositionsCreateHrRequest) -> Result&lt;PositionsCreateHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .positions_create(
            &PositionsCreateHrRequest {
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

**translations:** `Option<std::collections::HashMap<String, PositionsCreateHrRequestTranslationsValue>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">positions_update</a>(request: PositionsUpdateHrRequest) -> Result&lt;PositionsUpdateHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .positions_update(
            &PositionsUpdateHrRequest {
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

**translations:** `Option<Option<std::collections::HashMap<String, Option<PositionsUpdateHrRequestTranslationsValue>>>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">positions_list</a>(request: PositionsListHrRequest) -> Result&lt;PositionsListHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .positions_list(
            &PositionsListHrRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<PositionsListHrRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PositionsListHrRequestFilterItem>>` 
    
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">employees_create</a>(request: EmployeesCreateHrRequest) -> Result&lt;EmployeesCreateHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .employees_create(
            &EmployeesCreateHrRequest {
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

**address:** `Option<EmployeesCreateHrRequestAddress>` 
    
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

**attributes:** `Option<Vec<EmployeesCreateHrRequestAttributesItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">employees_update</a>(request: EmployeesUpdateHrRequest) -> Result&lt;EmployeesUpdateHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .employees_update(
            &EmployeesUpdateHrRequest {
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

**address:** `Option<Option<EmployeesUpdateHrRequestAddress>>` 
    
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

**attributes:** `Option<Vec<EmployeesUpdateHrRequestAttributesItem>>` 
    
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

**status:** `Option<EmployeesUpdateHrRequestStatus>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">employees_get</a>(request: EmployeesGetHrRequest) -> Result&lt;EmployeesGetHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .employees_get(
            &EmployeesGetHrRequest {
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">employees_fields</a>(request: EmployeesFieldsHrRequest) -> Result&lt;EmployeesFieldsHrResponse, ApiError&gt;</code></summary>
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
        .employees_fields(
            &EmployeesFieldsHrRequest {
                ..Default::default()
            },
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">employees_list</a>(request: EmployeesListHrRequest) -> Result&lt;EmployeesListHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .employees_list(
            &EmployeesListHrRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<EmployeesListHrRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<EmployeesListHrRequestFilterItem>>` 
    
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">employees_delete</a>(request: EmployeesDeleteHrRequest) -> Result&lt;EmployeesDeleteHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .employees_delete(
            &EmployeesDeleteHrRequest {
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">employees_anonymize</a>(request: EmployeesAnonymizeHrRequest) -> Result&lt;EmployeesAnonymizeHrResponse, ApiError&gt;</code></summary>
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
        .employees_anonymize(
            &EmployeesAnonymizeHrRequest {
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">contracts_create</a>(request: ContractsCreateHrRequest) -> Result&lt;ContractsCreateHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .contracts_create(
            &ContractsCreateHrRequest {
                employee_id: "employeeId".to_string(),
                start_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                base_salary: "121.0000".to_string(),
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

**type_:** `Option<ContractsCreateHrRequestType>` 
    
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

**salary_type:** `Option<ContractsCreateHrRequestSalaryType>` 
    
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">contracts_end</a>(request: ContractsEndHrRequest) -> Result&lt;ContractsEndHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .contracts_end(
            &ContractsEndHrRequest {
                id: "id".to_string(),
                end_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">contracts_list</a>(request: ContractsListHrRequest) -> Result&lt;ContractsListHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .contracts_list(
            &ContractsListHrRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<ContractsListHrRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<ContractsListHrRequestFilterItem>>` 
    
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">leave_balances_set</a>(request: LeaveBalancesSetHrRequest) -> Result&lt;LeaveBalancesSetHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .leave_balances_set(
            &LeaveBalancesSetHrRequest {
                employee_id: "employeeId".to_string(),
                year: 1000000,
                entitled_days: "121.00".to_string(),
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">leave_balances_list</a>(request: LeaveBalancesListHrRequest) -> Result&lt;LeaveBalancesListHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .leave_balances_list(
            &LeaveBalancesListHrRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">incapacity_certificates_create</a>(request: IncapacityCertificatesCreateHrRequest) -> Result&lt;IncapacityCertificatesCreateHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .incapacity_certificates_create(
            &IncapacityCertificatesCreateHrRequest {
                employee_id: "employeeId".to_string(),
                number: "number".to_string(),
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">incapacity_certificates_list</a>(request: IncapacityCertificatesListHrRequest) -> Result&lt;IncapacityCertificatesListHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .incapacity_certificates_list(
            &IncapacityCertificatesListHrRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<IncapacityCertificatesListHrRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<IncapacityCertificatesListHrRequestFilterItem>>` 
    
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">per_diem_rates_create</a>(request: PerDiemRatesCreateHrRequest) -> Result&lt;PerDiemRatesCreateHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .per_diem_rates_create(
            &PerDiemRatesCreateHrRequest {
                country_code: "countryCode".to_string(),
                daily_amount: "121.00".to_string(),
                valid_from: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
            },
            None,
        )
        .await;
}
```
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

**daily_amount:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**valid_from:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">per_diem_rates_list</a>(request: PerDiemRatesListHrRequest) -> Result&lt;PerDiemRatesListHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .per_diem_rates_list(
            &PerDiemRatesListHrRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<PerDiemRatesListHrRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<PerDiemRatesListHrRequestFilterItem>>` 
    
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">per_diem_rates_delete</a>(request: PerDiemRatesDeleteHrRequest) -> Result&lt;PerDiemRatesDeleteHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .per_diem_rates_delete(
            &PerDiemRatesDeleteHrRequest {
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">business_trips_create</a>(request: BusinessTripsCreateHrRequest) -> Result&lt;BusinessTripsCreateHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .business_trips_create(
            &BusinessTripsCreateHrRequest {
                employee_id: "employeeId".to_string(),
                destination_country_code: "destinationCountryCode".to_string(),
                purpose: "purpose".to_string(),
                start_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                end_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
            },
            None,
        )
        .await;
}
```
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

**destination_country_code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**purpose:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">business_trips_get</a>(request: BusinessTripsGetHrRequest) -> Result&lt;BusinessTripsGetHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .business_trips_get(
            &BusinessTripsGetHrRequest {
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">business_trips_list</a>(request: BusinessTripsListHrRequest) -> Result&lt;BusinessTripsListHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .business_trips_list(
            &BusinessTripsListHrRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<BusinessTripsListHrRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<BusinessTripsListHrRequestFilterItem>>` 
    
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">business_trips_approve</a>(request: BusinessTripsApproveHrRequest) -> Result&lt;BusinessTripsApproveHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .business_trips_approve(
            &BusinessTripsApproveHrRequest {
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">business_trips_delete</a>(request: BusinessTripsDeleteHrRequest) -> Result&lt;BusinessTripsDeleteHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .business_trips_delete(
            &BusinessTripsDeleteHrRequest {
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">employees_records_create</a>(request: EmployeesRecordsCreateHrRequest) -> Result&lt;EmployeesRecordsCreateHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .employees_records_create(
            &EmployeesRecordsCreateHrRequest {
                employee_id: "employeeId".to_string(),
                r#type: EmployeesRecordsCreateHrRequestType::Education,
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

**type_:** `EmployeesRecordsCreateHrRequestType` 
    
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">employees_records_update</a>(request: EmployeesRecordsUpdateHrRequest) -> Result&lt;EmployeesRecordsUpdateHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .employees_records_update(
            &EmployeesRecordsUpdateHrRequest {
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

**type_:** `Option<EmployeesRecordsUpdateHrRequestType>` 
    
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">employees_records_delete</a>(request: EmployeesRecordsDeleteHrRequest) -> Result&lt;EmployeesRecordsDeleteHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .employees_records_delete(
            &EmployeesRecordsDeleteHrRequest {
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">employees_records_list</a>(request: EmployeesRecordsListHrRequest) -> Result&lt;EmployeesRecordsListHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .employees_records_list(
            &EmployeesRecordsListHrRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<EmployeesRecordsListHrRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<EmployeesRecordsListHrRequestFilterItem>>` 
    
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">employees_attachments_list</a>(request: EmployeesAttachmentsListHrRequest) -> Result&lt;EmployeesAttachmentsListHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .employees_attachments_list(
            &EmployeesAttachmentsListHrRequest {
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">timesheets_generate</a>(request: TimesheetsGenerateHrRequest) -> Result&lt;TimesheetsGenerateHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .timesheets_generate(
            &TimesheetsGenerateHrRequest {
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">timesheets_upsert</a>(request: TimesheetsUpsertHrRequest) -> Result&lt;TimesheetsUpsertHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .timesheets_upsert(
            &TimesheetsUpsertHrRequest {
                employee_id: "employeeId".to_string(),
                year: 1000000,
                month: 1000000,
                days: vec![TimesheetsUpsertHrRequestDaysItem {
                    day: 1000000,
                    hours: "121.00".to_string(),
                    r#type: TimesheetsUpsertHrRequestDaysItemType::Work,
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

**days:** `Vec<TimesheetsUpsertHrRequestDaysItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">timesheets_get</a>(request: TimesheetsGetHrRequest) -> Result&lt;TimesheetsGetHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .timesheets_get(
            &TimesheetsGetHrRequest {
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">timesheets_list</a>(request: TimesheetsListHrRequest) -> Result&lt;TimesheetsListHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .timesheets_list(
            &TimesheetsListHrRequest {
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

<details><summary><code>client.hr.<a href="/src/api/resources/hr/client.rs">timesheets_delete</a>(request: TimesheetsDeleteHrRequest) -> Result&lt;TimesheetsDeleteHrResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .timesheets_delete(
            &TimesheetsDeleteHrRequest {
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

## fleet
<details><summary><code>client.fleet.<a href="/src/api/resources/fleet/client.rs">vehicles_create</a>(request: VehiclesCreateFleetRequest) -> Result&lt;VehiclesCreateFleetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .vehicles_create(
            &VehiclesCreateFleetRequest {
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

**fuel_type:** `Option<VehiclesCreateFleetRequestFuelType>` 
    
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

**documents:** `Option<Vec<VehiclesCreateFleetRequestDocumentsItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.fleet.<a href="/src/api/resources/fleet/client.rs">vehicles_update</a>(request: VehiclesUpdateFleetRequest) -> Result&lt;VehiclesUpdateFleetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .vehicles_update(
            &VehiclesUpdateFleetRequest {
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

**fuel_type:** `Option<Option<VehiclesUpdateFleetRequestFuelType>>` 
    
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

**status:** `Option<VehiclesUpdateFleetRequestStatus>` 
    
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

<details><summary><code>client.fleet.<a href="/src/api/resources/fleet/client.rs">vehicles_get</a>(request: VehiclesGetFleetRequest) -> Result&lt;VehiclesGetFleetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .vehicles_get(
            &VehiclesGetFleetRequest {
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

<details><summary><code>client.fleet.<a href="/src/api/resources/fleet/client.rs">vehicles_list</a>(request: VehiclesListFleetRequest) -> Result&lt;VehiclesListFleetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .vehicles_list(
            &VehiclesListFleetRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<VehiclesListFleetRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<VehiclesListFleetRequestFilterItem>>` 
    
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

<details><summary><code>client.fleet.<a href="/src/api/resources/fleet/client.rs">assignments_create</a>(request: AssignmentsCreateFleetRequest) -> Result&lt;AssignmentsCreateFleetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .assignments_create(
            &AssignmentsCreateFleetRequest {
                vehicle_id: "vehicleId".to_string(),
                employee_id: "employeeId".to_string(),
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

<details><summary><code>client.fleet.<a href="/src/api/resources/fleet/client.rs">assignments_end</a>(request: AssignmentsEndFleetRequest) -> Result&lt;AssignmentsEndFleetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .assignments_end(
            &AssignmentsEndFleetRequest {
                id: "id".to_string(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.fleet.<a href="/src/api/resources/fleet/client.rs">assignments_list</a>(request: AssignmentsListFleetRequest) -> Result&lt;AssignmentsListFleetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .assignments_list(
            &AssignmentsListFleetRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<AssignmentsListFleetRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<AssignmentsListFleetRequestFilterItem>>` 
    
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

<details><summary><code>client.fleet.<a href="/src/api/resources/fleet/client.rs">natura_preview</a>(request: NaturaPreviewFleetRequest) -> Result&lt;NaturaPreviewFleetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .natura_preview(
            &NaturaPreviewFleetRequest {
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

## payroll
<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">departments_create</a>(request: DepartmentsCreatePayrollRequest) -> Result&lt;DepartmentsCreatePayrollResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .departments_create(
            &DepartmentsCreatePayrollRequest {
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

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">departments_list</a>(request: DepartmentsListPayrollRequest) -> Result&lt;DepartmentsListPayrollResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .departments_list(
            &DepartmentsListPayrollRequest {
                ..Default::default()
            },
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

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">schedules_create</a>(request: SchedulesCreatePayrollRequest) -> Result&lt;SchedulesCreatePayrollResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .schedules_create(
            &SchedulesCreatePayrollRequest {
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

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">schedules_list</a>(request: SchedulesListPayrollRequest) -> Result&lt;SchedulesListPayrollResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .schedules_list(
            &SchedulesListPayrollRequest {
                ..Default::default()
            },
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

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">calc</a>(request: CalcPayrollRequest) -> Result&lt;CalcPayrollResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .calc(
            &CalcPayrollRequest {
                taxable_base: "121.00".to_string(),
                date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">runs_create</a>(request: RunsCreatePayrollRequest) -> Result&lt;RunsCreatePayrollResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .runs_create(
            &RunsCreatePayrollRequest {
                year: 1000000,
                month: 1000000,
                include_natura: None,
                gross_overrides: None,
                lines: None,
                notes: None,
                pay_date: None,
            },
            None,
        )
        .await;
}
```
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

**gross_overrides:** `Option<Vec<RunsCreatePayrollRequestGrossOverridesItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Option<Vec<RunsCreatePayrollRequestLinesItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**pay_date:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">runs_get</a>(request: RunsGetPayrollRequest) -> Result&lt;RunsGetPayrollResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .runs_get(
            &RunsGetPayrollRequest {
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

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">runs_list</a>(request: RunsListPayrollRequest) -> Result&lt;RunsListPayrollResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .runs_list(
            &RunsListPayrollRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<RunsListPayrollRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<RunsListPayrollRequestFilterItem>>` 
    
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

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">lines_attendance</a>(request: LinesAttendancePayrollRequest) -> Result&lt;LinesAttendancePayrollResponse, ApiError&gt;</code></summary>
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
        .lines_attendance(
            &LinesAttendancePayrollRequest {
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

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">runs_approve</a>(request: RunsApprovePayrollRequest) -> Result&lt;RunsApprovePayrollResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .runs_approve(
            &RunsApprovePayrollRequest {
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

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">runs_reverse</a>(request: RunsReversePayrollRequest) -> Result&lt;RunsReversePayrollResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .runs_reverse(
            &RunsReversePayrollRequest {
                id: "id".to_string(),
                reason: "reason".to_string(),
            },
            None,
        )
        .await;
}
```
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

**reason:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">runs_cancel</a>(request: RunsCancelPayrollRequest) -> Result&lt;RunsCancelPayrollResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .runs_cancel(
            &RunsCancelPayrollRequest {
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

<details><summary><code>client.payroll.<a href="/src/api/resources/payroll/client.rs">payments_export</a>(request: PaymentsExportPayrollRequest) -> Result&lt;PaymentsExportPayrollResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .payments_export(
            &PaymentsExportPayrollRequest {
                run_id: "runId".to_string(),
                bank_account_id: "bankAccountId".to_string(),
                execution_date: None,
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

<dl>
<dd>

**locale:** `Option<PaymentsExportPayrollRequestLocale>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## agreements
<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">settings_get</a>(request: SettingsGetAgreementsRequest) -> Result&lt;SettingsGetAgreementsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .settings_get(
            &SettingsGetAgreementsRequest {
                ..Default::default()
            },
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

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">settings_update</a>(request: SettingsUpdateAgreementsRequest) -> Result&lt;SettingsUpdateAgreementsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .settings_update(
            &SettingsUpdateAgreementsRequest { auto_billing: true },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**auto_billing:** `bool` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">types_create</a>(request: TypesCreateAgreementsRequest) -> Result&lt;TypesCreateAgreementsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .types_create(
            &TypesCreateAgreementsRequest {
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

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">types_list</a>(request: TypesListAgreementsRequest) -> Result&lt;TypesListAgreementsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .types_list(
            &TypesListAgreementsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<TypesListAgreementsRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<TypesListAgreementsRequestFilterItem>>` 
    
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

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">agreements_create</a>(request: AgreementsCreateAgreementsRequest) -> Result&lt;AgreementsCreateAgreementsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .agreements_create(
            &AgreementsCreateAgreementsRequest {
                number: "number".to_string(),
                start_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

**kind:** `Option<AgreementsCreateAgreementsRequestKind>` 
    
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

**billing_period:** `Option<AgreementsCreateAgreementsRequestBillingPeriod>` 
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<AgreementsCreateAgreementsRequestStatus>` 
    
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

**items:** `Option<Vec<AgreementsCreateAgreementsRequestItemsItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">agreements_get</a>(request: AgreementsGetAgreementsRequest) -> Result&lt;AgreementsGetAgreementsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .agreements_get(
            &AgreementsGetAgreementsRequest {
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

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">agreements_update</a>(request: AgreementsUpdateAgreementsRequest) -> Result&lt;AgreementsUpdateAgreementsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .agreements_update(
            &AgreementsUpdateAgreementsRequest {
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

**kind:** `Option<AgreementsUpdateAgreementsRequestKind>` 
    
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

**billing_period:** `Option<Option<AgreementsUpdateAgreementsRequestBillingPeriod>>` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<AgreementsUpdateAgreementsRequestStatus>` 
    
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

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">agreements_delete</a>(request: AgreementsDeleteAgreementsRequest) -> Result&lt;AgreementsDeleteAgreementsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .agreements_delete(
            &AgreementsDeleteAgreementsRequest {
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

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">agreements_list</a>(request: AgreementsListAgreementsRequest) -> Result&lt;AgreementsListAgreementsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .agreements_list(
            &AgreementsListAgreementsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<AgreementsListAgreementsRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<AgreementsListAgreementsRequestFilterItem>>` 
    
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

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">agreements_generate_invoice</a>(request: AgreementsGenerateInvoiceAgreementsRequest) -> Result&lt;AgreementsGenerateInvoiceAgreementsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .agreements_generate_invoice(
            &AgreementsGenerateInvoiceAgreementsRequest {
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

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">agreements_billing_run</a>(request: AgreementsBillingRunAgreementsRequest) -> Result&lt;AgreementsBillingRunAgreementsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .agreements_billing_run(
            &AgreementsBillingRunAgreementsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">insurance_policies_create</a>(request: InsurancePoliciesCreateAgreementsRequest) -> Result&lt;InsurancePoliciesCreateAgreementsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .insurance_policies_create(
            &InsurancePoliciesCreateAgreementsRequest {
                policy_number: "policyNumber".to_string(),
                insured_object: "insuredObject".to_string(),
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">insurance_policies_list</a>(request: InsurancePoliciesListAgreementsRequest) -> Result&lt;InsurancePoliciesListAgreementsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .insurance_policies_list(
            &InsurancePoliciesListAgreementsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<InsurancePoliciesListAgreementsRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<InsurancePoliciesListAgreementsRequestFilterItem>>` 
    
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

<details><summary><code>client.agreements.<a href="/src/api/resources/agreements/client.rs">insurance_policies_delete</a>(request: InsurancePoliciesDeleteAgreementsRequest) -> Result&lt;InsurancePoliciesDeleteAgreementsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .insurance_policies_delete(
            &InsurancePoliciesDeleteAgreementsRequest {
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

## inventory
<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">settings_get</a>(request: SettingsGetInventoryRequest) -> Result&lt;SettingsGetInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .settings_get(
            &SettingsGetInventoryRequest {
                ..Default::default()
            },
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

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">settings_update</a>(request: SettingsUpdateInventoryRequest) -> Result&lt;SettingsUpdateInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .settings_update(
            &SettingsUpdateInventoryRequest {
                negative_stock_policy: SettingsUpdateInventoryRequestNegativeStockPolicy::Reject,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**negative_stock_policy:** `SettingsUpdateInventoryRequestNegativeStockPolicy` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">warehouses_create</a>(request: WarehousesCreateInventoryRequest) -> Result&lt;WarehousesCreateInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .warehouses_create(
            &WarehousesCreateInventoryRequest {
                code: "code".to_string(),
                name: "name".to_string(),
                is_default: None,
                country_code: None,
            },
            None,
        )
        .await;
}
```
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

<dl>
<dd>

**country_code:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">warehouses_list</a>(request: WarehousesListInventoryRequest) -> Result&lt;WarehousesListInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .warehouses_list(
            &WarehousesListInventoryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<WarehousesListInventoryRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<WarehousesListInventoryRequestFilterItem>>` 
    
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

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">warehouses_update</a>(request: WarehousesUpdateInventoryRequest) -> Result&lt;WarehousesUpdateInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .warehouses_update(
            &WarehousesUpdateInventoryRequest {
                id: "id".to_string(),
                name: None,
                country_code: None,
            },
            None,
        )
        .await;
}
```
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

**country_code:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">stock_receive</a>(request: StockReceiveInventoryRequest) -> Result&lt;StockReceiveInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .stock_receive(
            &StockReceiveInventoryRequest {
                warehouse_id: "warehouseId".to_string(),
                item_id: "itemId".to_string(),
                date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                quantity: "121.0000".to_string(),
                unit_cost: "121.000000".to_string(),
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

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">stock_write_off</a>(request: StockWriteOffInventoryRequest) -> Result&lt;StockWriteOffInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .stock_write_off(
            &StockWriteOffInventoryRequest {
                warehouse_id: "warehouseId".to_string(),
                item_id: "itemId".to_string(),
                date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                quantity: "121.0000".to_string(),
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

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">stock_transfer</a>(request: StockTransferInventoryRequest) -> Result&lt;StockTransferInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .stock_transfer(
            &StockTransferInventoryRequest {
                from_warehouse_id: "fromWarehouseId".to_string(),
                to_warehouse_id: "toWarehouseId".to_string(),
                item_id: "itemId".to_string(),
                date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                quantity: "121.0000".to_string(),
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

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">stock_take</a>(request: StockTakeInventoryRequest) -> Result&lt;StockTakeInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .stock_take(
            &StockTakeInventoryRequest {
                warehouse_id: "warehouseId".to_string(),
                date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                lines: vec![StockTakeInventoryRequestLinesItem {
                    counted_qty: "121.0000".to_string(),
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

**lines:** `Vec<StockTakeInventoryRequestLinesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">stock_levels</a>(request: StockLevelsInventoryRequest) -> Result&lt;StockLevelsInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .stock_levels(
            &StockLevelsInventoryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">stock_movements_list</a>(request: StockMovementsListInventoryRequest) -> Result&lt;StockMovementsListInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .stock_movements_list(
            &StockMovementsListInventoryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<StockMovementsListInventoryRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<StockMovementsListInventoryRequestFilterItem>>` 
    
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

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">lots_list</a>(request: LotsListInventoryRequest) -> Result&lt;LotsListInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .lots_list(
            &LotsListInventoryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<LotsListInventoryRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<LotsListInventoryRequestFilterItem>>` 
    
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

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">lots_get</a>(request: LotsGetInventoryRequest) -> Result&lt;LotsGetInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .lots_get(
            &LotsGetInventoryRequest {
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

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">lots_update</a>(request: LotsUpdateInventoryRequest) -> Result&lt;LotsUpdateInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .lots_update(
            &LotsUpdateInventoryRequest {
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

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">landed_costs_create</a>(request: LandedCostsCreateInventoryRequest) -> Result&lt;LandedCostsCreateInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .landed_costs_create(
            &LandedCostsCreateInventoryRequest {
                date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                amount: "121.000000".to_string(),
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

**method:** `Option<LandedCostsCreateInventoryRequestMethod>` 
    
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

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">landed_costs_get</a>(request: LandedCostsGetInventoryRequest) -> Result&lt;LandedCostsGetInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .landed_costs_get(
            &LandedCostsGetInventoryRequest {
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

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">landed_costs_list</a>(request: LandedCostsListInventoryRequest) -> Result&lt;LandedCostsListInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .landed_costs_list(
            &LandedCostsListInventoryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<LandedCostsListInventoryRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<LandedCostsListInventoryRequestFilterItem>>` 
    
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

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">reorder_rules_create</a>(request: ReorderRulesCreateInventoryRequest) -> Result&lt;ReorderRulesCreateInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .reorder_rules_create(
            &ReorderRulesCreateInventoryRequest {
                item_id: "itemId".to_string(),
                min_qty: "121.0000".to_string(),
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

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">reorder_rules_update</a>(request: ReorderRulesUpdateInventoryRequest) -> Result&lt;ReorderRulesUpdateInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .reorder_rules_update(
            &ReorderRulesUpdateInventoryRequest {
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

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">reorder_rules_delete</a>(request: ReorderRulesDeleteInventoryRequest) -> Result&lt;ReorderRulesDeleteInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .reorder_rules_delete(
            &ReorderRulesDeleteInventoryRequest {
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

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">reorder_rules_list</a>(request: ReorderRulesListInventoryRequest) -> Result&lt;ReorderRulesListInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .reorder_rules_list(
            &ReorderRulesListInventoryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<ReorderRulesListInventoryRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<ReorderRulesListInventoryRequestFilterItem>>` 
    
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

<details><summary><code>client.inventory.<a href="/src/api/resources/inventory/client.rs">reorder_rules_check</a>(request: ReorderRulesCheckInventoryRequest) -> Result&lt;ReorderRulesCheckInventoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .reorder_rules_check(
            &ReorderRulesCheckInventoryRequest {
                ..Default::default()
            },
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

## production
<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">work_centers_create</a>(request: WorkCentersCreateProductionRequest) -> Result&lt;WorkCentersCreateProductionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .work_centers_create(
            &WorkCentersCreateProductionRequest {
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

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">work_centers_update</a>(request: WorkCentersUpdateProductionRequest) -> Result&lt;WorkCentersUpdateProductionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .work_centers_update(
            &WorkCentersUpdateProductionRequest {
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

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">work_centers_list</a>(request: WorkCentersListProductionRequest) -> Result&lt;WorkCentersListProductionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .work_centers_list(
            &WorkCentersListProductionRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<WorkCentersListProductionRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<WorkCentersListProductionRequestFilterItem>>` 
    
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

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">routings_create</a>(request: RoutingsCreateProductionRequest) -> Result&lt;RoutingsCreateProductionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .routings_create(
            &RoutingsCreateProductionRequest {
                code: "code".to_string(),
                name: "name".to_string(),
                operations: vec![RoutingsCreateProductionRequestOperationsItem {
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

**operations:** `Vec<RoutingsCreateProductionRequestOperationsItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">routings_get</a>(request: RoutingsGetProductionRequest) -> Result&lt;RoutingsGetProductionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .routings_get(
            &RoutingsGetProductionRequest {
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

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">routings_list</a>(request: RoutingsListProductionRequest) -> Result&lt;RoutingsListProductionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .routings_list(
            &RoutingsListProductionRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<RoutingsListProductionRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<RoutingsListProductionRequestFilterItem>>` 
    
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

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">maintenance_create</a>(request: MaintenanceCreateProductionRequest) -> Result&lt;MaintenanceCreateProductionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .maintenance_create(
            &MaintenanceCreateProductionRequest {
                work_center_id: "workCenterId".to_string(),
                r#type: MaintenanceCreateProductionRequestType::Preventive,
                planned_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

**type_:** `MaintenanceCreateProductionRequestType` 
    
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

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">maintenance_complete</a>(request: MaintenanceCompleteProductionRequest) -> Result&lt;MaintenanceCompleteProductionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .maintenance_complete(
            &MaintenanceCompleteProductionRequest {
                id: "id".to_string(),
                completed_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">maintenance_cancel</a>(request: MaintenanceCancelProductionRequest) -> Result&lt;MaintenanceCancelProductionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .maintenance_cancel(
            &MaintenanceCancelProductionRequest {
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

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">maintenance_list</a>(request: MaintenanceListProductionRequest) -> Result&lt;MaintenanceListProductionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .maintenance_list(
            &MaintenanceListProductionRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<MaintenanceListProductionRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<MaintenanceListProductionRequestFilterItem>>` 
    
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

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">boms_create</a>(request: BomsCreateProductionRequest) -> Result&lt;BomsCreateProductionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .boms_create(
            &BomsCreateProductionRequest {
                code: "code".to_string(),
                name: "name".to_string(),
                finished_item_id: "finishedItemId".to_string(),
                lines: vec![BomsCreateProductionRequestLinesItem {
                    component_item_id: "componentItemId".to_string(),
                    quantity: "121.0000".to_string(),
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

**lines:** `Vec<BomsCreateProductionRequestLinesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">boms_get</a>(request: BomsGetProductionRequest) -> Result&lt;BomsGetProductionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .boms_get(
            &BomsGetProductionRequest {
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

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">boms_list</a>(request: BomsListProductionRequest) -> Result&lt;BomsListProductionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .boms_list(
            &BomsListProductionRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<BomsListProductionRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<BomsListProductionRequestFilterItem>>` 
    
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

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">orders_create</a>(request: OrdersCreateProductionRequest) -> Result&lt;OrdersCreateProductionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_create(
            &OrdersCreateProductionRequest {
                bom_id: "bomId".to_string(),
                warehouse_id: "warehouseId".to_string(),
                quantity: "121.0000".to_string(),
                date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

**type_:** `Option<OrdersCreateProductionRequestType>` 
    
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

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">orders_record_operation</a>(request: OrdersRecordOperationProductionRequest) -> Result&lt;OrdersRecordOperationProductionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_record_operation(
            &OrdersRecordOperationProductionRequest {
                id: "id".to_string(),
                actual_minutes: "121.00".to_string(),
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">quality_checks_add</a>(request: QualityChecksAddProductionRequest) -> Result&lt;QualityChecksAddProductionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .quality_checks_add(
            &QualityChecksAddProductionRequest {
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

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">quality_checks_record</a>(request: QualityChecksRecordProductionRequest) -> Result&lt;QualityChecksRecordProductionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .quality_checks_record(
            &QualityChecksRecordProductionRequest {
                id: "id".to_string(),
                result: QualityChecksRecordProductionRequestResult::Passed,
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

**result:** `QualityChecksRecordProductionRequestResult` 
    
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

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">quality_checks_list</a>(request: QualityChecksListProductionRequest) -> Result&lt;QualityChecksListProductionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .quality_checks_list(
            &QualityChecksListProductionRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<QualityChecksListProductionRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<QualityChecksListProductionRequestFilterItem>>` 
    
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

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">orders_complete</a>(request: OrdersCompleteProductionRequest) -> Result&lt;OrdersCompleteProductionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_complete(
            &OrdersCompleteProductionRequest {
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

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">orders_get</a>(request: OrdersGetProductionRequest) -> Result&lt;OrdersGetProductionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_get(
            &OrdersGetProductionRequest {
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

<details><summary><code>client.production.<a href="/src/api/resources/production/client.rs">orders_list</a>(request: OrdersListProductionRequest) -> Result&lt;OrdersListProductionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_list(
            &OrdersListProductionRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<OrdersListProductionRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<OrdersListProductionRequestFilterItem>>` 
    
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

## ecommerce
<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">orders_create</a>(request: OrdersCreateEcommerceRequest) -> Result&lt;OrdersCreateEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_create(
            &OrdersCreateEcommerceRequest {
                lines: vec![OrdersCreateEcommerceRequestLinesItem {
                    description: "description".to_string(),
                    quantity: "121.0000".to_string(),
                    unit_price_excl_vat: "121.0000".to_string(),
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

**partner:** `Option<OrdersCreateEcommerceRequestPartner>` 
    
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

**lines:** `Vec<OrdersCreateEcommerceRequestLinesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">orders_get</a>(request: OrdersGetEcommerceRequest) -> Result&lt;OrdersGetEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_get(
            &OrdersGetEcommerceRequest {
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

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">orders_list</a>(request: OrdersListEcommerceRequest) -> Result&lt;OrdersListEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_list(
            &OrdersListEcommerceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<OrdersListEcommerceRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<OrdersListEcommerceRequestFilterItem>>` 
    
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

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">orders_reserve</a>(request: OrdersReserveEcommerceRequest) -> Result&lt;OrdersReserveEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_reserve(
            &OrdersReserveEcommerceRequest {
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

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">orders_fulfill</a>(request: OrdersFulfillEcommerceRequest) -> Result&lt;OrdersFulfillEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_fulfill(
            &OrdersFulfillEcommerceRequest {
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

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">orders_cancel</a>(request: OrdersCancelEcommerceRequest) -> Result&lt;OrdersCancelEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_cancel(
            &OrdersCancelEcommerceRequest {
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

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">products_list</a>(request: ProductsListEcommerceRequest) -> Result&lt;ProductsListEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .products_list(
            &ProductsListEcommerceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">stock_list</a>(request: StockListEcommerceRequest) -> Result&lt;StockListEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .stock_list(
            &StockListEcommerceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

## cash
<details><summary><code>client.cash.<a href="/src/api/resources/cash/client.rs">orders_create</a>(request: OrdersCreateCashRequest) -> Result&lt;OrdersCreateCashResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_create(
            &OrdersCreateCashRequest {
                r#type: OrdersCreateCashRequestType::Receipt,
                date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                amount: "121.0000".to_string(),
                purpose: "purpose".to_string(),
                counter_account_code: None,
                cash_account_code: None,
                sale_invoice_id: None,
                purchase_invoice_id: None,
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

**type_:** `OrdersCreateCashRequestType` 
    
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

**counter_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**cash_account_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**sale_invoice_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**purchase_invoice_id:** `Option<String>` 
    
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

<details><summary><code>client.cash.<a href="/src/api/resources/cash/client.rs">orders_get</a>(request: OrdersGetCashRequest) -> Result&lt;OrdersGetCashResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_get(
            &OrdersGetCashRequest {
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

<details><summary><code>client.cash.<a href="/src/api/resources/cash/client.rs">orders_list</a>(request: OrdersListCashRequest) -> Result&lt;OrdersListCashResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .orders_list(
            &OrdersListCashRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<OrdersListCashRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<OrdersListCashRequestFilterItem>>` 
    
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

<details><summary><code>client.cash.<a href="/src/api/resources/cash/client.rs">balance</a>(request: BalanceCashRequest) -> Result&lt;BalanceCashResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .balance(
            &BalanceCashRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.cash.<a href="/src/api/resources/cash/client.rs">expense_reports_create</a>(request: ExpenseReportsCreateCashRequest) -> Result&lt;ExpenseReportsCreateCashResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .expense_reports_create(
            &ExpenseReportsCreateCashRequest {
                employee_id: "employeeId".to_string(),
                date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                lines: vec![ExpenseReportsCreateCashRequestLinesItem {
                    description: "description".to_string(),
                    account_code: "accountCode".to_string(),
                    net_amount: "121.00".to_string(),
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

**employee_id:** `String` 
    
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

<dl>
<dd>

**lines:** `Vec<ExpenseReportsCreateCashRequestLinesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.cash.<a href="/src/api/resources/cash/client.rs">expense_reports_get</a>(request: ExpenseReportsGetCashRequest) -> Result&lt;ExpenseReportsGetCashResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .expense_reports_get(
            &ExpenseReportsGetCashRequest {
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

<details><summary><code>client.cash.<a href="/src/api/resources/cash/client.rs">expense_reports_list</a>(request: ExpenseReportsListCashRequest) -> Result&lt;ExpenseReportsListCashResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .expense_reports_list(
            &ExpenseReportsListCashRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<ExpenseReportsListCashRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<ExpenseReportsListCashRequestFilterItem>>` 
    
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

<details><summary><code>client.cash.<a href="/src/api/resources/cash/client.rs">advance_holders_balances</a>(request: AdvanceHoldersBalancesCashRequest) -> Result&lt;AdvanceHoldersBalancesCashResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .advance_holders_balances(
            &AdvanceHoldersBalancesCashRequest {
                ..Default::default()
            },
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

## projects
<details><summary><code>client.projects.<a href="/src/api/resources/projects/client.rs">create</a>(request: CreateProjectsRequest) -> Result&lt;CreateProjectsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .create(
            &CreateProjectsRequest {
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

<details><summary><code>client.projects.<a href="/src/api/resources/projects/client.rs">update</a>(request: UpdateProjectsRequest) -> Result&lt;UpdateProjectsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .update(
            &UpdateProjectsRequest {
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

**status:** `Option<UpdateProjectsRequestStatus>` 
    
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

<details><summary><code>client.projects.<a href="/src/api/resources/projects/client.rs">get</a>(request: GetProjectsRequest) -> Result&lt;GetProjectsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .get(
            &GetProjectsRequest {
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

<details><summary><code>client.projects.<a href="/src/api/resources/projects/client.rs">list</a>(request: ListProjectsRequest) -> Result&lt;ListProjectsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .list(
            &ListProjectsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<ListProjectsRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<ListProjectsRequestFilterItem>>` 
    
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

<details><summary><code>client.projects.<a href="/src/api/resources/projects/client.rs">time_entries_create</a>(request: TimeEntriesCreateProjectsRequest) -> Result&lt;TimeEntriesCreateProjectsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .time_entries_create(
            &TimeEntriesCreateProjectsRequest {
                project_id: "projectId".to_string(),
                date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                hours: "121.00".to_string(),
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

<details><summary><code>client.projects.<a href="/src/api/resources/projects/client.rs">time_entries_update</a>(request: TimeEntriesUpdateProjectsRequest) -> Result&lt;TimeEntriesUpdateProjectsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .time_entries_update(
            &TimeEntriesUpdateProjectsRequest {
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

<details><summary><code>client.projects.<a href="/src/api/resources/projects/client.rs">time_entries_delete</a>(request: TimeEntriesDeleteProjectsRequest) -> Result&lt;TimeEntriesDeleteProjectsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .time_entries_delete(
            &TimeEntriesDeleteProjectsRequest {
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

<details><summary><code>client.projects.<a href="/src/api/resources/projects/client.rs">time_entries_list</a>(request: TimeEntriesListProjectsRequest) -> Result&lt;TimeEntriesListProjectsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .time_entries_list(
            &TimeEntriesListProjectsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<TimeEntriesListProjectsRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<TimeEntriesListProjectsRequestFilterItem>>` 
    
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

<details><summary><code>client.projects.<a href="/src/api/resources/projects/client.rs">time_entries_bill</a>(request: TimeEntriesBillProjectsRequest) -> Result&lt;TimeEntriesBillProjectsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .time_entries_bill(
            &TimeEntriesBillProjectsRequest {
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

**group_by:** `Option<TimeEntriesBillProjectsRequestGroupBy>` 
    
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

<details><summary><code>client.projects.<a href="/src/api/resources/projects/client.rs">report</a>(request: ReportProjectsRequest) -> Result&lt;ReportProjectsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .report(
            &ReportProjectsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

## transport
<details><summary><code>client.transport.<a href="/src/api/resources/transport/client.rs">waybills_create</a>(request: WaybillsCreateTransportRequest) -> Result&lt;WaybillsCreateTransportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .waybills_create(
            &WaybillsCreateTransportRequest {
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

**lines:** `Option<Vec<WaybillsCreateTransportRequestLinesItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transport.<a href="/src/api/resources/transport/client.rs">waybills_update</a>(request: WaybillsUpdateTransportRequest) -> Result&lt;WaybillsUpdateTransportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .waybills_update(
            &WaybillsUpdateTransportRequest {
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

**lines:** `Option<Vec<WaybillsUpdateTransportRequestLinesItem>>` 
    
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

<details><summary><code>client.transport.<a href="/src/api/resources/transport/client.rs">waybills_issue</a>(request: WaybillsIssueTransportRequest) -> Result&lt;WaybillsIssueTransportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .waybills_issue(
            &WaybillsIssueTransportRequest {
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

<details><summary><code>client.transport.<a href="/src/api/resources/transport/client.rs">waybills_cancel</a>(request: WaybillsCancelTransportRequest) -> Result&lt;WaybillsCancelTransportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .waybills_cancel(
            &WaybillsCancelTransportRequest {
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

<details><summary><code>client.transport.<a href="/src/api/resources/transport/client.rs">waybills_get</a>(request: WaybillsGetTransportRequest) -> Result&lt;WaybillsGetTransportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .waybills_get(
            &WaybillsGetTransportRequest {
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

<details><summary><code>client.transport.<a href="/src/api/resources/transport/client.rs">waybills_list</a>(request: WaybillsListTransportRequest) -> Result&lt;WaybillsListTransportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .waybills_list(
            &WaybillsListTransportRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<WaybillsListTransportRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<WaybillsListTransportRequestFilterItem>>` 
    
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

## pos
<details><summary><code>client.pos.<a href="/src/api/resources/pos/client.rs">devices_create</a>(request: DevicesCreatePosRequest) -> Result&lt;DevicesCreatePosResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .devices_create(
            &DevicesCreatePosRequest {
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

<details><summary><code>client.pos.<a href="/src/api/resources/pos/client.rs">devices_update</a>(request: DevicesUpdatePosRequest) -> Result&lt;DevicesUpdatePosResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .devices_update(
            &DevicesUpdatePosRequest {
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

<details><summary><code>client.pos.<a href="/src/api/resources/pos/client.rs">devices_list</a>(request: DevicesListPosRequest) -> Result&lt;DevicesListPosResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .devices_list(
            &DevicesListPosRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<DevicesListPosRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<DevicesListPosRequestFilterItem>>` 
    
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

<details><summary><code>client.pos.<a href="/src/api/resources/pos/client.rs">reports_create</a>(request: ReportsCreatePosRequest) -> Result&lt;ReportsCreatePosResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .reports_create(
            &ReportsCreatePosRequest {
                report_number: "reportNumber".to_string(),
                date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                vat_lines: vec![ReportsCreatePosRequestVatLinesItem {
                    vat_rate_percent: "121.00".to_string(),
                    net_amount: "121.0000".to_string(),
                    vat_amount: "121.0000".to_string(),
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

**vat_lines:** `Vec<ReportsCreatePosRequestVatLinesItem>` 
    
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

**item_lines:** `Option<Vec<ReportsCreatePosRequestItemLinesItem>>` 
    
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

<details><summary><code>client.pos.<a href="/src/api/resources/pos/client.rs">reports_get</a>(request: ReportsGetPosRequest) -> Result&lt;ReportsGetPosResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .reports_get(
            &ReportsGetPosRequest {
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

<details><summary><code>client.pos.<a href="/src/api/resources/pos/client.rs">reports_list</a>(request: ReportsListPosRequest) -> Result&lt;ReportsListPosResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .reports_list(
            &ReportsListPosRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<ReportsListPosRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<ReportsListPosRequestFilterItem>>` 
    
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

<details><summary><code>client.pos.<a href="/src/api/resources/pos/client.rs">shifts_open</a>(request: ShiftsOpenPosRequest) -> Result&lt;ShiftsOpenPosResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .shifts_open(
            &ShiftsOpenPosRequest {
                device_id: "deviceId".to_string(),
                warehouse_id: None,
                opening_cash: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**device_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**warehouse_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**opening_cash:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pos.<a href="/src/api/resources/pos/client.rs">shifts_get</a>(request: ShiftsGetPosRequest) -> Result&lt;ShiftsGetPosResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .shifts_get(
            &ShiftsGetPosRequest {
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

<details><summary><code>client.pos.<a href="/src/api/resources/pos/client.rs">shifts_list</a>(request: ShiftsListPosRequest) -> Result&lt;ShiftsListPosResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .shifts_list(
            &ShiftsListPosRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<ShiftsListPosRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<ShiftsListPosRequestFilterItem>>` 
    
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

<details><summary><code>client.pos.<a href="/src/api/resources/pos/client.rs">receipts_create</a>(request: ReceiptsCreatePosRequest) -> Result&lt;ReceiptsCreatePosResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .receipts_create(
            &ReceiptsCreatePosRequest {
                shift_id: "shiftId".to_string(),
                lines: vec![ReceiptsCreatePosRequestLinesItem {
                    quantity: "121.0000".to_string(),
                    unit_price_incl_vat: "121.0000".to_string(),
                    vat_rate_percent: "121.00".to_string(),
                    ..Default::default()
                }],
                cash_amount: None,
                card_amount: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**shift_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Vec<ReceiptsCreatePosRequestLinesItem>` 
    
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
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pos.<a href="/src/api/resources/pos/client.rs">receipts_list</a>(request: ReceiptsListPosRequest) -> Result&lt;ReceiptsListPosResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .receipts_list(
            &ReceiptsListPosRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<ReceiptsListPosRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<ReceiptsListPosRequestFilterItem>>` 
    
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

<details><summary><code>client.pos.<a href="/src/api/resources/pos/client.rs">receipts_get</a>(request: ReceiptsGetPosRequest) -> Result&lt;ReceiptsGetPosResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .receipts_get(
            &ReceiptsGetPosRequest {
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

<details><summary><code>client.pos.<a href="/src/api/resources/pos/client.rs">shifts_close</a>(request: ShiftsClosePosRequest) -> Result&lt;ShiftsClosePosResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .shifts_close(
            &ShiftsClosePosRequest {
                id: "id".to_string(),
                counted_cash: "121.00".to_string(),
                date: None,
                report_number: None,
            },
            None,
        )
        .await;
}
```
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

**counted_cash:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**date:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**report_number:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## calendar
<details><summary><code>client.calendar.<a href="/src/api/resources/calendar/client.rs">list</a>(request: ListCalendarRequest) -> Result&lt;ListCalendarResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .list(
            &ListCalendarRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.calendar.<a href="/src/api/resources/calendar/client.rs">get</a>(request: GetCalendarRequest) -> Result&lt;GetCalendarResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .get(
            &GetCalendarRequest {
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

<details><summary><code>client.calendar.<a href="/src/api/resources/calendar/client.rs">submit</a>(request: SubmitCalendarRequest) -> Result&lt;SubmitCalendarResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

With amend: true the return is filed again as a correction of the one already submitted or accepted for the period; only returns whose format has a correction mark accept it.
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
        .submit(
            &SubmitCalendarRequest {
                key: "key".to_string(),
                amend: None,
            },
            None,
        )
        .await;
}
```
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

**amend:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.calendar.<a href="/src/api/resources/calendar/client.rs">download</a>(request: DownloadCalendarRequest) -> Result&lt;DownloadCalendarResponse, ApiError&gt;</code></summary>
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
        .download(
            &DownloadCalendarRequest {
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

<details><summary><code>client.calendar.<a href="/src/api/resources/calendar/client.rs">create</a>(request: CreateCalendarRequest) -> Result&lt;CreateCalendarResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .create(
            &CreateCalendarRequest {
                title: "title".to_string(),
                due_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

<details><summary><code>client.calendar.<a href="/src/api/resources/calendar/client.rs">update</a>(request: UpdateCalendarRequest) -> Result&lt;UpdateCalendarResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .update(
            &UpdateCalendarRequest {
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

<details><summary><code>client.calendar.<a href="/src/api/resources/calendar/client.rs">delete</a>(request: DeleteCalendarRequest) -> Result&lt;DeleteCalendarResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .delete(
            &DeleteCalendarRequest {
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

## audit
<details><summary><code>client.audit.<a href="/src/api/resources/audit/client.rs">list</a>(request: ListAuditRequest) -> Result&lt;ListAuditResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .list(
            &ListAuditRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<ListAuditRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<ListAuditRequestFilterItem>>` 
    
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

## webhooks
<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">subscriptions_create</a>(request: SubscriptionsCreateWebhooksRequest) -> Result&lt;SubscriptionsCreateWebhooksResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .subscriptions_create(
            &SubscriptionsCreateWebhooksRequest {
                url: "url".to_string(),
                events: vec![
                    SubscriptionsCreateWebhooksRequestEventsItem::AgreementInvoiceGenerated,
                ],
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

**events:** `Vec<SubscriptionsCreateWebhooksRequestEventsItem>` 
    
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

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">subscriptions_list</a>(request: SubscriptionsListWebhooksRequest) -> Result&lt;SubscriptionsListWebhooksResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .subscriptions_list(
            &SubscriptionsListWebhooksRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<SubscriptionsListWebhooksRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<SubscriptionsListWebhooksRequestFilterItem>>` 
    
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

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">subscriptions_update</a>(request: SubscriptionsUpdateWebhooksRequest) -> Result&lt;SubscriptionsUpdateWebhooksResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .subscriptions_update(
            &SubscriptionsUpdateWebhooksRequest {
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

**events:** `Option<Vec<SubscriptionsUpdateWebhooksRequestEventsItem>>` 
    
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

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">subscriptions_delete</a>(request: SubscriptionsDeleteWebhooksRequest) -> Result&lt;SubscriptionsDeleteWebhooksResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .subscriptions_delete(
            &SubscriptionsDeleteWebhooksRequest {
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

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">deliveries_list</a>(request: DeliveriesListWebhooksRequest) -> Result&lt;DeliveriesListWebhooksResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .deliveries_list(
            &DeliveriesListWebhooksRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<DeliveriesListWebhooksRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<DeliveriesListWebhooksRequestFilterItem>>` 
    
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

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">deliveries_redeliver</a>(request: DeliveriesRedeliverWebhooksRequest) -> Result&lt;DeliveriesRedeliverWebhooksResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .deliveries_redeliver(
            &DeliveriesRedeliverWebhooksRequest {
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

## bank
<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">accounts_create</a>(request: AccountsCreateBankRequest) -> Result&lt;AccountsCreateBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .accounts_create(
            &AccountsCreateBankRequest {
                name: "name".to_string(),
                r#type: None,
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

**type_:** `Option<AccountsCreateBankRequestType>` 
    
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">accounts_list</a>(request: AccountsListBankRequest) -> Result&lt;AccountsListBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .accounts_list(
            &AccountsListBankRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<AccountsListBankRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<AccountsListBankRequestFilterItem>>` 
    
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">accounts_update</a>(request: AccountsUpdateBankRequest) -> Result&lt;AccountsUpdateBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .accounts_update(
            &AccountsUpdateBankRequest {
                id: "id".to_string(),
                name: None,
                r#type: None,
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

**type_:** `Option<AccountsUpdateBankRequestType>` 
    
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">transactions_import</a>(request: TransactionsImportBankRequest) -> Result&lt;TransactionsImportBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .transactions_import(
            &TransactionsImportBankRequest {
                bank_account_id: "bankAccountId".to_string(),
                transactions: vec![TransactionsImportBankRequestTransactionsItem {
                    date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                    amount: "-121.0000".to_string(),
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

**transactions:** `Vec<TransactionsImportBankRequestTransactionsItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">statements_import</a>(request: StatementsImportBankRequest) -> Result&lt;StatementsImportBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .statements_import(
            &StatementsImportBankRequest {
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

**format:** `Option<StatementsImportBankRequestFormat>` 
    
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">transactions_list</a>(request: TransactionsListBankRequest) -> Result&lt;TransactionsListBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .transactions_list(
            &TransactionsListBankRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<TransactionsListBankRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<TransactionsListBankRequestFilterItem>>` 
    
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">transactions_match</a>(request: TransactionsMatchBankRequest) -> Result&lt;TransactionsMatchBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .transactions_match(
            &TransactionsMatchBankRequest {
                transaction_id: "transactionId".to_string(),
                document_type: TransactionsMatchBankRequestDocumentType::SaleInvoice,
                document_id: "documentId".to_string(),
                invoice_amount: None,
            },
            None,
        )
        .await;
}
```
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

**document_type:** `TransactionsMatchBankRequestDocumentType` 
    
</dd>
</dl>

<dl>
<dd>

**document_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**invoice_amount:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">transactions_match_many</a>(request: TransactionsMatchManyBankRequest) -> Result&lt;TransactionsMatchManyBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .transactions_match_many(
            &TransactionsMatchManyBankRequest {
                transaction_id: "transactionId".to_string(),
                allocations: vec![TransactionsMatchManyBankRequestAllocationsItem {
                    document_type:
                        TransactionsMatchManyBankRequestAllocationsItemDocumentType::SaleInvoice,
                    document_id: "documentId".to_string(),
                    amount: "121.0000".to_string(),
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

**transaction_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**allocations:** `Vec<TransactionsMatchManyBankRequestAllocationsItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">transactions_unmatch</a>(request: TransactionsUnmatchBankRequest) -> Result&lt;TransactionsUnmatchBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Undo a match. A payment matched to an invoice, or a line posted by an import template, gets a reversing journal transaction dated date (default: today) and the invoice paid amount and payment status are restored; a line linked to a payment-provider settlement is only unlinked. The line returns to status new.
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
        .transactions_unmatch(
            &TransactionsUnmatchBankRequest {
                transaction_id: "transactionId".to_string(),
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

**transaction_id:** `String` 
    
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">transactions_record</a>(request: TransactionsRecordBankRequest) -> Result&lt;TransactionsRecordBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .transactions_record(
            &TransactionsRecordBankRequest {
                bank_account_id: "bankAccountId".to_string(),
                date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                amount: "121.0000".to_string(),
                document_type: TransactionsRecordBankRequestDocumentType::SaleInvoice,
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

**document_type:** `TransactionsRecordBankRequestDocumentType` 
    
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">payments_export</a>(request: PaymentsExportBankRequest) -> Result&lt;PaymentsExportBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .payments_export(
            &PaymentsExportBankRequest {
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">import_templates_create</a>(request: ImportTemplatesCreateBankRequest) -> Result&lt;ImportTemplatesCreateBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .import_templates_create(
            &ImportTemplatesCreateBankRequest {
                name: "name".to_string(),
                r#type: ImportTemplatesCreateBankRequestType::Stripe,
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

**type_:** `ImportTemplatesCreateBankRequestType` 
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<Vec<ImportTemplatesCreateBankRequestFieldsItem>>` 
    
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">import_templates_update</a>(request: ImportTemplatesUpdateBankRequest) -> Result&lt;ImportTemplatesUpdateBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .import_templates_update(
            &ImportTemplatesUpdateBankRequest {
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

**type_:** `Option<ImportTemplatesUpdateBankRequestType>` 
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<Vec<ImportTemplatesUpdateBankRequestFieldsItem>>` 
    
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">import_templates_delete</a>(request: ImportTemplatesDeleteBankRequest) -> Result&lt;ImportTemplatesDeleteBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .import_templates_delete(
            &ImportTemplatesDeleteBankRequest {
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">import_templates_get</a>(request: ImportTemplatesGetBankRequest) -> Result&lt;ImportTemplatesGetBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .import_templates_get(
            &ImportTemplatesGetBankRequest {
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">import_templates_list</a>(request: ImportTemplatesListBankRequest) -> Result&lt;ImportTemplatesListBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .import_templates_list(
            &ImportTemplatesListBankRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<ImportTemplatesListBankRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<ImportTemplatesListBankRequestFilterItem>>` 
    
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">match_rules_create</a>(request: MatchRulesCreateBankRequest) -> Result&lt;MatchRulesCreateBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .match_rules_create(
            &MatchRulesCreateBankRequest {
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">match_rules_update</a>(request: MatchRulesUpdateBankRequest) -> Result&lt;MatchRulesUpdateBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .match_rules_update(
            &MatchRulesUpdateBankRequest {
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">match_rules_delete</a>(request: MatchRulesDeleteBankRequest) -> Result&lt;MatchRulesDeleteBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .match_rules_delete(
            &MatchRulesDeleteBankRequest {
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">match_rules_list</a>(request: MatchRulesListBankRequest) -> Result&lt;MatchRulesListBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .match_rules_list(
            &MatchRulesListBankRequest {
                ..Default::default()
            },
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">mandates_create</a>(request: MandatesCreateBankRequest) -> Result&lt;MandatesCreateBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .mandates_create(
            &MandatesCreateBankRequest {
                partner_id: "partnerId".to_string(),
                iban: "iban".to_string(),
                signature_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

**scheme:** `Option<MandatesCreateBankRequestScheme>` 
    
</dd>
</dl>

<dl>
<dd>

**sequence_type:** `Option<MandatesCreateBankRequestSequenceType>` 
    
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">mandates_update</a>(request: MandatesUpdateBankRequest) -> Result&lt;MandatesUpdateBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .mandates_update(
            &MandatesUpdateBankRequest {
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">mandates_cancel</a>(request: MandatesCancelBankRequest) -> Result&lt;MandatesCancelBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .mandates_cancel(
            &MandatesCancelBankRequest {
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">mandates_get</a>(request: MandatesGetBankRequest) -> Result&lt;MandatesGetBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .mandates_get(
            &MandatesGetBankRequest {
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">mandates_list</a>(request: MandatesListBankRequest) -> Result&lt;MandatesListBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .mandates_list(
            &MandatesListBankRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<MandatesListBankRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<MandatesListBankRequestFilterItem>>` 
    
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">direct_debits_candidates</a>(request: DirectDebitsCandidatesBankRequest) -> Result&lt;DirectDebitsCandidatesBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .direct_debits_candidates(
            &DirectDebitsCandidatesBankRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<DirectDebitsCandidatesBankRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<DirectDebitsCandidatesBankRequestFilterItem>>` 
    
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">direct_debits_export</a>(request: DirectDebitsExportBankRequest) -> Result&lt;DirectDebitsExportBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .direct_debits_export(
            &DirectDebitsExportBankRequest {
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">transactions_suggest_matches</a>(request: TransactionsSuggestMatchesBankRequest) -> Result&lt;TransactionsSuggestMatchesBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .transactions_suggest_matches(
            &TransactionsSuggestMatchesBankRequest {
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">settlements_import</a>(request: SettlementsImportBankRequest) -> Result&lt;SettlementsImportBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .settlements_import(
            &SettlementsImportBankRequest {
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

**provider:** `Option<SettlementsImportBankRequestProvider>` 
    
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">settlements_list</a>(request: SettlementsListBankRequest) -> Result&lt;SettlementsListBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .settlements_list(
            &SettlementsListBankRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<SettlementsListBankRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<SettlementsListBankRequestFilterItem>>` 
    
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">settlements_get</a>(request: SettlementsGetBankRequest) -> Result&lt;SettlementsGetBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .settlements_get(
            &SettlementsGetBankRequest {
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">settlements_match</a>(request: SettlementsMatchBankRequest) -> Result&lt;SettlementsMatchBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .settlements_match(
            &SettlementsMatchBankRequest {
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">settlements_commission</a>(request: SettlementsCommissionBankRequest) -> Result&lt;SettlementsCommissionBankResponse, ApiError&gt;</code></summary>
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
        .settlements_commission(
            &SettlementsCommissionBankRequest {
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">settlements_link</a>(request: SettlementsLinkBankRequest) -> Result&lt;SettlementsLinkBankResponse, ApiError&gt;</code></summary>
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
        .settlements_link(
            &SettlementsLinkBankRequest {
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">settlements_unlink</a>(request: SettlementsUnlinkBankRequest) -> Result&lt;SettlementsUnlinkBankResponse, ApiError&gt;</code></summary>
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
        .settlements_unlink(
            &SettlementsUnlinkBankRequest {
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">settlements_post</a>(request: SettlementsPostBankRequest) -> Result&lt;SettlementsPostBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .settlements_post(
            &SettlementsPostBankRequest {
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">feeds_banks_list</a>(request: FeedsBanksListBankRequest) -> Result&lt;FeedsBanksListBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .feeds_banks_list(
            &FeedsBanksListBankRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">feeds_connections_start</a>(request: FeedsConnectionsStartBankRequest) -> Result&lt;FeedsConnectionsStartBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .feeds_connections_start(
            &FeedsConnectionsStartBankRequest {
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

**psu_type:** `Option<FeedsConnectionsStartBankRequestPsuType>` 
    
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">feeds_connections_complete</a>(request: FeedsConnectionsCompleteBankRequest) -> Result&lt;FeedsConnectionsCompleteBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .feeds_connections_complete(
            &FeedsConnectionsCompleteBankRequest {
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">feeds_connections_get</a>(request: FeedsConnectionsGetBankRequest) -> Result&lt;FeedsConnectionsGetBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .feeds_connections_get(
            &FeedsConnectionsGetBankRequest {
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">feeds_connections_list</a>(request: FeedsConnectionsListBankRequest) -> Result&lt;FeedsConnectionsListBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .feeds_connections_list(
            &FeedsConnectionsListBankRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<FeedsConnectionsListBankRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<FeedsConnectionsListBankRequestFilterItem>>` 
    
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">feeds_connections_delete</a>(request: FeedsConnectionsDeleteBankRequest) -> Result&lt;FeedsConnectionsDeleteBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .feeds_connections_delete(
            &FeedsConnectionsDeleteBankRequest {
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">feeds_accounts_link</a>(request: FeedsAccountsLinkBankRequest) -> Result&lt;FeedsAccountsLinkBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .feeds_accounts_link(
            &FeedsAccountsLinkBankRequest {
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

**create_bank_account:** `Option<FeedsAccountsLinkBankRequestCreateBankAccount>` 
    
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

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">feeds_accounts_configure</a>(request: FeedsAccountsConfigureBankRequest) -> Result&lt;FeedsAccountsConfigureBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .feeds_accounts_configure(
            &FeedsAccountsConfigureBankRequest {
                id: "id".to_string(),
                import_template_id: None,
                sync_schedule: None,
            },
            None,
        )
        .await;
}
```
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

**sync_schedule:** `Option<FeedsAccountsConfigureBankRequestSyncSchedule>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bank.<a href="/src/api/resources/bank/client.rs">feeds_sync</a>(request: FeedsSyncBankRequest) -> Result&lt;FeedsSyncBankResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .feeds_sync(
            &FeedsSyncBankRequest {
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

## files
<details><summary><code>client.files.<a href="/src/api/resources/files/client.rs">upload</a>(request: UploadFilesRequest) -> Result&lt;UploadFilesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .upload(
            &UploadFilesRequest {
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

**mime_type:** `String` — Stored as the bare media type; only PNG, JPEG, GIF, WebP and PDF files are shown in the browser, every other type is downloaded
    
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

<details><summary><code>client.files.<a href="/src/api/resources/files/client.rs">get</a>(request: GetFilesRequest) -> Result&lt;GetFilesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .get(
            &GetFilesRequest {
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

<details><summary><code>client.files.<a href="/src/api/resources/files/client.rs">list</a>(request: ListFilesRequest) -> Result&lt;ListFilesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .list(
            &ListFilesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<ListFilesRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<ListFilesRequestFilterItem>>` 
    
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

<details><summary><code>client.files.<a href="/src/api/resources/files/client.rs">delete</a>(request: DeleteFilesRequest) -> Result&lt;DeleteFilesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .delete(
            &DeleteFilesRequest {
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

## reports
<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">trial_balance</a>(request: TrialBalanceReportsRequest) -> Result&lt;TrialBalanceReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .trial_balance(
            &TrialBalanceReportsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">size_category</a>(request: SizeCategoryReportsRequest) -> Result&lt;SizeCategoryReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .size_category(&SizeCategoryReportsRequest { year: 1000000 }, None)
        .await;
}
```
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">financial_statements</a>(request: FinancialStatementsReportsRequest) -> Result&lt;FinancialStatementsReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .financial_statements(
            &FinancialStatementsReportsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

**category:** `Option<FinancialStatementsReportsRequestCategory>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">general_journal</a>(request: GeneralJournalReportsRequest) -> Result&lt;GeneralJournalReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .general_journal(
            &GeneralJournalReportsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">gl_detail</a>(request: GlDetailReportsRequest) -> Result&lt;GlDetailReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .gl_detail(
            &GlDetailReportsRequest {
                account_code: "accountCode".to_string(),
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">partner_balances</a>(request: PartnerBalancesReportsRequest) -> Result&lt;PartnerBalancesReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .partner_balances(
            &PartnerBalancesReportsRequest {
                ..Default::default()
            },
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">debt_aging</a>(request: DebtAgingReportsRequest) -> Result&lt;DebtAgingReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .debt_aging(
            &DebtAgingReportsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**side:** `Option<DebtAgingReportsRequestSide>` 
    
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">monthly_summary</a>(request: MonthlySummaryReportsRequest) -> Result&lt;MonthlySummaryReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .monthly_summary(
            &MonthlySummaryReportsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">stock_balance</a>(request: StockBalanceReportsRequest) -> Result&lt;StockBalanceReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .stock_balance(
            &StockBalanceReportsRequest {
                as_of: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">stock_movement</a>(request: StockMovementReportsRequest) -> Result&lt;StockMovementReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .stock_movement(
            &StockMovementReportsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">vat_summary</a>(request: VatSummaryReportsRequest) -> Result&lt;VatSummaryReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .vat_summary(
            &VatSummaryReportsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

**side:** `Option<VatSummaryReportsRequestSide>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">cash_flow</a>(request: CashFlowReportsRequest) -> Result&lt;CashFlowReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .cash_flow(
            &CashFlowReportsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">stock_aging</a>(request: StockAgingReportsRequest) -> Result&lt;StockAgingReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .stock_aging(
            &StockAgingReportsRequest {
                as_of: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">stock_shortage</a>(request: StockShortageReportsRequest) -> Result&lt;StockShortageReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .stock_shortage(
            &StockShortageReportsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">sie</a>(request: SieReportsRequest) -> Result&lt;SieReportsResponse, ApiError&gt;</code></summary>
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
        .sie(
            &SieReportsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">datev</a>(request: DatevReportsRequest) -> Result&lt;DatevReportsResponse, ApiError&gt;</code></summary>
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
        .datev(
            &DatevReportsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">fec</a>(request: FecReportsRequest) -> Result&lt;FecReportsResponse, ApiError&gt;</code></summary>
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
        .fec(
            &FecReportsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">eu_purchases</a>(request: EuPurchasesReportsRequest) -> Result&lt;EuPurchasesReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .eu_purchases(
            &EuPurchasesReportsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">vat_detail</a>(request: VatDetailReportsRequest) -> Result&lt;VatDetailReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .vat_detail(
            &VatDetailReportsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

**side:** `Option<VatDetailReportsRequestSide>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">pos_sales</a>(request: PosSalesReportsRequest) -> Result&lt;PosSalesReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .pos_sales(
            &PosSalesReportsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">online_sales</a>(request: OnlineSalesReportsRequest) -> Result&lt;OnlineSalesReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .online_sales(
            &OnlineSalesReportsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">oss</a>(request: OssReportsRequest) -> Result&lt;OssReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .oss(
            &OssReportsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">advance_reconciliation</a>(request: AdvanceReconciliationReportsRequest) -> Result&lt;AdvanceReconciliationReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .advance_reconciliation(
            &AdvanceReconciliationReportsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">write_off_acts</a>(request: WriteOffActsReportsRequest) -> Result&lt;WriteOffActsReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .write_off_acts(
            &WriteOffActsReportsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">cost_centers</a>(request: CostCentersReportsRequest) -> Result&lt;CostCentersReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .cost_centers(
            &CostCentersReportsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">cost_center_activity</a>(request: CostCenterActivityReportsRequest) -> Result&lt;CostCenterActivityReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .cost_center_activity(
            &CostCenterActivityReportsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">cost_center_items</a>(request: CostCenterItemsReportsRequest) -> Result&lt;CostCenterItemsReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .cost_center_items(
            &CostCenterItemsReportsRequest {
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">jobs_create</a>(request: JobsCreateReportsRequest) -> Result&lt;JobsCreateReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .jobs_create(
            &JobsCreateReportsRequest {
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

**formats:** `Option<Vec<JobsCreateReportsRequestFormatsItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">jobs_get</a>(request: JobsGetReportsRequest) -> Result&lt;JobsGetReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .jobs_get(
            &JobsGetReportsRequest {
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

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">jobs_list</a>(request: JobsListReportsRequest) -> Result&lt;JobsListReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .jobs_list(
            &JobsListReportsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**sort:** `Option<Vec<JobsListReportsRequestSortItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<Vec<JobsListReportsRequestFilterItem>>` 
    
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

## consolidation
<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">groups_create</a>(request: GroupsCreateConsolidationRequest) -> Result&lt;GroupsCreateConsolidationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .groups_create(
            &GroupsCreateConsolidationRequest {
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

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">groups_list</a>(request: GroupsListConsolidationRequest) -> Result&lt;GroupsListConsolidationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .groups_list(
            &GroupsListConsolidationRequest {
                ..Default::default()
            },
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

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">groups_get</a>(request: GroupsGetConsolidationRequest) -> Result&lt;GroupsGetConsolidationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .groups_get(
            &GroupsGetConsolidationRequest {
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

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">groups_update</a>(request: GroupsUpdateConsolidationRequest) -> Result&lt;GroupsUpdateConsolidationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .groups_update(
            &GroupsUpdateConsolidationRequest {
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

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">groups_delete</a>(request: GroupsDeleteConsolidationRequest) -> Result&lt;GroupsDeleteConsolidationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .groups_delete(
            &GroupsDeleteConsolidationRequest {
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

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">members_add</a>(request: MembersAddConsolidationRequest) -> Result&lt;MembersAddConsolidationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .members_add(
            &MembersAddConsolidationRequest {
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

**method:** `Option<MembersAddConsolidationRequestMethod>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">members_remove</a>(request: MembersRemoveConsolidationRequest) -> Result&lt;MembersRemoveConsolidationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .members_remove(
            &MembersRemoveConsolidationRequest {
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

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">intercompany_candidates</a>(request: IntercompanyCandidatesConsolidationRequest) -> Result&lt;IntercompanyCandidatesConsolidationResponse, ApiError&gt;</code></summary>
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
        .intercompany_candidates(
            &IntercompanyCandidatesConsolidationRequest {
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

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">intercompany_links_set</a>(request: IntercompanyLinksSetConsolidationRequest) -> Result&lt;IntercompanyLinksSetConsolidationResponse, ApiError&gt;</code></summary>
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
        .intercompany_links_set(
            &IntercompanyLinksSetConsolidationRequest {
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

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">intercompany_links_list</a>(request: IntercompanyLinksListConsolidationRequest) -> Result&lt;IntercompanyLinksListConsolidationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .intercompany_links_list(
            &IntercompanyLinksListConsolidationRequest {
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

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">intercompany_links_remove</a>(request: IntercompanyLinksRemoveConsolidationRequest) -> Result&lt;IntercompanyLinksRemoveConsolidationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .intercompany_links_remove(
            &IntercompanyLinksRemoveConsolidationRequest {
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

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">intercompany_report</a>(request: IntercompanyReportConsolidationRequest) -> Result&lt;IntercompanyReportConsolidationResponse, ApiError&gt;</code></summary>
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
        .intercompany_report(
            &IntercompanyReportConsolidationRequest {
                group_id: "groupId".to_string(),
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.consolidation.<a href="/src/api/resources/consolidation/client.rs">report</a>(request: ReportConsolidationRequest) -> Result&lt;ReportConsolidationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .report(
            &ReportConsolidationRequest {
                group_id: "groupId".to_string(),
                from_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to_date: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
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

**category:** `Option<ReportConsolidationRequestCategory>` 
    
</dd>
</dl>

<dl>
<dd>

**eliminations:** `Option<Vec<ReportConsolidationRequestEliminationsItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## public
<details><summary><code>client.public.<a href="/src/api/resources/public/client.rs">integration_requests</a>(request: IntegrationRequestsPublicRequest) -> Result&lt;IntegrationRequestsPublicResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .integration_requests(
            &IntegrationRequestsPublicRequest {
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

<details><summary><code>client.public.<a href="/src/api/resources/public/client.rs">pay</a>(token: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use nordlet::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.public.pay(&"token".to_string(), None).await;
}
```
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

## billing
<details><summary><code>client.billing.<a href="/src/api/resources/billing/client.rs">account_get</a>(request: AccountGetBillingRequest) -> Result&lt;AccountGetBillingResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .account_get(
            &AccountGetBillingRequest {
                ..Default::default()
            },
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

<details><summary><code>client.billing.<a href="/src/api/resources/billing/client.rs">account_set_plan</a>(request: AccountSetPlanBillingRequest) -> Result&lt;AccountSetPlanBillingResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .account_set_plan(
            &AccountSetPlanBillingRequest {
                plan: AccountSetPlanBillingRequestPlan::Starter,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**plan:** `AccountSetPlanBillingRequestPlan` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.billing.<a href="/src/api/resources/billing/client.rs">topup_create</a>(request: TopupCreateBillingRequest) -> Result&lt;TopupCreateBillingResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .topup_create(
            &TopupCreateBillingRequest {
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

**locale:** `Option<TopupCreateBillingRequestLocale>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.billing.<a href="/src/api/resources/billing/client.rs">portal_create</a>(request: PortalCreateBillingRequest) -> Result&lt;PortalCreateBillingResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .portal_create(
            &PortalCreateBillingRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**locale:** `Option<PortalCreateBillingRequestLocale>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.billing.<a href="/src/api/resources/billing/client.rs">transactions_list</a>(request: TransactionsListBillingRequest) -> Result&lt;TransactionsListBillingResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .transactions_list(
            &TransactionsListBillingRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.billing.<a href="/src/api/resources/billing/client.rs">usage_list</a>(request: UsageListBillingRequest) -> Result&lt;UsageListBillingResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .usage_list(
            &UsageListBillingRequest {
                from: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
                to: NaiveDate::parse_from_str("2026-07-01", "%Y-%m-%d").unwrap(),
            },
            None,
        )
        .await;
}
```
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

## account
<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">login_link_request</a>(request: LoginLinkRequestAccountRequest) -> Result&lt;LoginLinkRequestAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .login_link_request(
            &LoginLinkRequestAccountRequest {
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

**locale:** `Option<LoginLinkRequestAccountRequestLocale>` 
    
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">login_link_consume</a>(request: LoginLinkConsumeAccountRequest) -> Result&lt;LoginLinkConsumeAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .login_link_consume(
            &LoginLinkConsumeAccountRequest {
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">logout</a>(request: LogoutAccountRequest) -> Result&lt;LogoutAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .logout(
            &LogoutAccountRequest {
                ..Default::default()
            },
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">me</a>(request: MeAccountRequest) -> Result&lt;MeAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .me(
            &MeAccountRequest {
                ..Default::default()
            },
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">members_list</a>(request: MembersListAccountRequest) -> Result&lt;MembersListAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .members_list(
            &MembersListAccountRequest {
                ..Default::default()
            },
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">members_set_role</a>(request: MembersSetRoleAccountRequest) -> Result&lt;MembersSetRoleAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .members_set_role(
            &MembersSetRoleAccountRequest {
                user_id: "userId".to_string(),
                role: MembersSetRoleAccountRequestRole::Admin,
            },
            None,
        )
        .await;
}
```
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

**role:** `MembersSetRoleAccountRequestRole` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">members_transfer_ownership</a>(request: MembersTransferOwnershipAccountRequest) -> Result&lt;MembersTransferOwnershipAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .members_transfer_ownership(
            &MembersTransferOwnershipAccountRequest {
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">members_remove</a>(request: MembersRemoveAccountRequest) -> Result&lt;MembersRemoveAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .members_remove(
            &MembersRemoveAccountRequest {
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">invites_create</a>(request: InvitesCreateAccountRequest) -> Result&lt;InvitesCreateAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invites_create(
            &InvitesCreateAccountRequest {
                email: "email".to_string(),
                role: InvitesCreateAccountRequestRole::Admin,
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

**role:** `InvitesCreateAccountRequestRole` 
    
</dd>
</dl>

<dl>
<dd>

**locale:** `Option<InvitesCreateAccountRequestLocale>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">invites_list</a>(request: InvitesListAccountRequest) -> Result&lt;InvitesListAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invites_list(
            &InvitesListAccountRequest {
                ..Default::default()
            },
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">invites_revoke</a>(request: InvitesRevokeAccountRequest) -> Result&lt;InvitesRevokeAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invites_revoke(
            &InvitesRevokeAccountRequest {
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">invites_get</a>(request: InvitesGetAccountRequest) -> Result&lt;InvitesGetAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invites_get(
            &InvitesGetAccountRequest {
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">invites_accept</a>(request: InvitesAcceptAccountRequest) -> Result&lt;InvitesAcceptAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .invites_accept(
            &InvitesAcceptAccountRequest {
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

**locale:** `Option<InvitesAcceptAccountRequestLocale>` 
    
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">locale_set</a>(request: LocaleSetAccountRequest) -> Result&lt;LocaleSetAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .locale_set(
            &LocaleSetAccountRequest {
                locale: LocaleSetAccountRequestLocale::En,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**locale:** `LocaleSetAccountRequestLocale` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">companies_create</a>(request: CompaniesCreateAccountRequest) -> Result&lt;CompaniesCreateAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .companies_create(
            &CompaniesCreateAccountRequest {
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
                base_currency: None,
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

**vat_period:** `Option<CompaniesCreateAccountRequestVatPeriod>` 
    
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

**address:** `Option<CompaniesCreateAccountRequestAddress>` 
    
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

**accounts_kept_by:** `Option<CompaniesCreateAccountRequestAccountsKeptBy>` 
    
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

**country_code:** `Option<CompaniesCreateAccountRequestCountryCode>` — Jurisdiction the company is registered in (immutable after creation)
    
</dd>
</dl>

<dl>
<dd>

**base_currency:** `Option<String>` — Currency the ledger is kept in; defaults to the national currency of countryCode (immutable after creation)
    
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">companies_select</a>(request: CompaniesSelectAccountRequest) -> Result&lt;CompaniesSelectAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .companies_select(
            &CompaniesSelectAccountRequest {
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">companies_profile</a>(request: CompaniesProfileAccountRequest) -> Result&lt;CompaniesProfileAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .companies_profile(
            &CompaniesProfileAccountRequest {
                ..Default::default()
            },
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">companies_update</a>(request: CompaniesUpdateAccountRequest) -> Result&lt;CompaniesUpdateAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .companies_update(
            &CompaniesUpdateAccountRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

**vat_period:** `Option<Option<CompaniesUpdateAccountRequestVatPeriod>>` 
    
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

**address:** `Option<CompaniesUpdateAccountRequestAddress>` 
    
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

**accounts_kept_by:** `Option<Option<CompaniesUpdateAccountRequestAccountsKeptBy>>` 
    
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

**logo:** `Option<CompaniesUpdateAccountRequestLogo>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">companies_archive</a>(request: CompaniesArchiveAccountRequest) -> Result&lt;CompaniesArchiveAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .companies_archive(
            &CompaniesArchiveAccountRequest {
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">companies_delete</a>(request: CompaniesDeleteAccountRequest) -> Result&lt;CompaniesDeleteAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .companies_delete(
            &CompaniesDeleteAccountRequest {
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">companies_activate</a>(request: CompaniesActivateAccountRequest) -> Result&lt;CompaniesActivateAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .companies_activate(
            &CompaniesActivateAccountRequest {
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">api_keys_create</a>(request: ApiKeysCreateAccountRequest) -> Result&lt;ApiKeysCreateAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .api_keys_create(
            &APIKeysCreateAccountRequest {
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">api_keys_list</a>(request: ApiKeysListAccountRequest) -> Result&lt;ApiKeysListAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .api_keys_list(
            &APIKeysListAccountRequest {
                ..Default::default()
            },
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">api_keys_rotate</a>(request: ApiKeysRotateAccountRequest) -> Result&lt;ApiKeysRotateAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .api_keys_rotate(
            &APIKeysRotateAccountRequest {
                id: "id".to_string(),
                overlap_hours: None,
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">api_keys_revoke</a>(request: ApiKeysRevokeAccountRequest) -> Result&lt;ApiKeysRevokeAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .api_keys_revoke(
            &APIKeysRevokeAccountRequest {
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">consent_accept</a>(request: ConsentAcceptAccountRequest) -> Result&lt;ConsentAcceptAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .consent_accept(
            &ConsentAcceptAccountRequest {
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">profile_update</a>(request: ProfileUpdateAccountRequest) -> Result&lt;ProfileUpdateAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .profile_update(
            &ProfileUpdateAccountRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">email_change_request</a>(request: EmailChangeRequestAccountRequest) -> Result&lt;EmailChangeRequestAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .email_change_request(
            &EmailChangeRequestAccountRequest {
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

**locale:** `Option<EmailChangeRequestAccountRequestLocale>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">sessions_list</a>(request: SessionsListAccountRequest) -> Result&lt;SessionsListAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .sessions_list(
            &SessionsListAccountRequest {
                ..Default::default()
            },
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">sessions_revoke</a>(request: SessionsRevokeAccountRequest) -> Result&lt;SessionsRevokeAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .sessions_revoke(
            &SessionsRevokeAccountRequest {
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">sessions_revoke_others</a>(request: SessionsRevokeOthersAccountRequest) -> Result&lt;SessionsRevokeOthersAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .sessions_revoke_others(
            &SessionsRevokeOthersAccountRequest {
                ..Default::default()
            },
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">export</a>(request: ExportAccountRequest) -> Result&lt;ExportAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .export(
            &ExportAccountRequest {
                ..Default::default()
            },
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">delete</a>(request: DeleteAccountRequest) -> Result&lt;DeleteAccountResponse, ApiError&gt;</code></summary>
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
        .delete(
            &DeleteAccountRequest {
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">referral_get</a>(request: ReferralGetAccountRequest) -> Result&lt;ReferralGetAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .referral_get(
            &ReferralGetAccountRequest {
                ..Default::default()
            },
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">referral_convert</a>(request: ReferralConvertAccountRequest) -> Result&lt;ReferralConvertAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .referral_convert(&ReferralConvertAccountRequest { points: 1000000 }, None)
        .await;
}
```
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">table_settings_get</a>(request: TableSettingsGetAccountRequest) -> Result&lt;TableSettingsGetAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .table_settings_get(
            &TableSettingsGetAccountRequest {
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">table_settings_set</a>(request: TableSettingsSetAccountRequest) -> Result&lt;TableSettingsSetAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .table_settings_set(
            &TableSettingsSetAccountRequest {
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

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">table_settings_list</a>(request: TableSettingsListAccountRequest) -> Result&lt;TableSettingsListAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
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
        .table_settings_list(
            &TableSettingsListAccountRequest {
                ..Default::default()
            },
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

