pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetPlatformSellersResponse {
    #[serde(default)]
    pub id: String,
    pub kind: GetPlatformSellersResponseKind,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "middleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "entityName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_name: Option<String>,
    #[serde(rename = "taxResidences")]
    #[serde(default)]
    pub tax_residences: Vec<GetPlatformSellersResponseTaxResidencesItem>,
    #[serde(rename = "vatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<String>,
    #[serde(rename = "businessRegistrationNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_registration_number: Option<String>,
    #[serde(default)]
    pub address: GetPlatformSellersResponseAddress,
    #[serde(rename = "birthDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<NaiveDate>,
    #[serde(rename = "birthCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_city: Option<String>,
    #[serde(rename = "birthCountryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(rename = "accountHolderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
    #[serde(rename = "governmentEntity")]
    #[serde(default)]
    pub government_entity: bool,
    #[serde(rename = "listedEntity")]
    #[serde(default)]
    pub listed_entity: bool,
    #[serde(rename = "permanentEstablishments")]
    #[serde(default)]
    pub permanent_establishments: Vec<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub activities: Vec<GetPlatformSellersResponseActivitiesItem>,
}

impl GetPlatformSellersResponse {
    pub fn builder() -> GetPlatformSellersResponseBuilder {
        <GetPlatformSellersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPlatformSellersResponseBuilder {
    id: Option<String>,
    kind: Option<GetPlatformSellersResponseKind>,
    name: Option<String>,
    partner_id: Option<String>,
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
    entity_name: Option<String>,
    tax_residences: Option<Vec<GetPlatformSellersResponseTaxResidencesItem>>,
    vat_code: Option<String>,
    business_registration_number: Option<String>,
    address: Option<GetPlatformSellersResponseAddress>,
    birth_date: Option<NaiveDate>,
    birth_city: Option<String>,
    birth_country_code: Option<String>,
    iban: Option<String>,
    account_holder_name: Option<String>,
    government_entity: Option<bool>,
    listed_entity: Option<bool>,
    permanent_establishments: Option<Vec<String>>,
    created_at: Option<DateTime<FixedOffset>>,
    activities: Option<Vec<GetPlatformSellersResponseActivitiesItem>>,
}

impl GetPlatformSellersResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: GetPlatformSellersResponseKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn middle_name(mut self, value: impl Into<String>) -> Self {
        self.middle_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn entity_name(mut self, value: impl Into<String>) -> Self {
        self.entity_name = Some(value.into());
        self
    }

    pub fn tax_residences(
        mut self,
        value: Vec<GetPlatformSellersResponseTaxResidencesItem>,
    ) -> Self {
        self.tax_residences = Some(value);
        self
    }

    pub fn vat_code(mut self, value: impl Into<String>) -> Self {
        self.vat_code = Some(value.into());
        self
    }

    pub fn business_registration_number(mut self, value: impl Into<String>) -> Self {
        self.business_registration_number = Some(value.into());
        self
    }

    pub fn address(mut self, value: GetPlatformSellersResponseAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn birth_date(mut self, value: NaiveDate) -> Self {
        self.birth_date = Some(value);
        self
    }

    pub fn birth_city(mut self, value: impl Into<String>) -> Self {
        self.birth_city = Some(value.into());
        self
    }

    pub fn birth_country_code(mut self, value: impl Into<String>) -> Self {
        self.birth_country_code = Some(value.into());
        self
    }

    pub fn iban(mut self, value: impl Into<String>) -> Self {
        self.iban = Some(value.into());
        self
    }

    pub fn account_holder_name(mut self, value: impl Into<String>) -> Self {
        self.account_holder_name = Some(value.into());
        self
    }

    pub fn government_entity(mut self, value: bool) -> Self {
        self.government_entity = Some(value);
        self
    }

    pub fn listed_entity(mut self, value: bool) -> Self {
        self.listed_entity = Some(value);
        self
    }

    pub fn permanent_establishments(mut self, value: Vec<String>) -> Self {
        self.permanent_establishments = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn activities(mut self, value: Vec<GetPlatformSellersResponseActivitiesItem>) -> Self {
        self.activities = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPlatformSellersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetPlatformSellersResponseBuilder::id)
    /// - [`kind`](GetPlatformSellersResponseBuilder::kind)
    /// - [`name`](GetPlatformSellersResponseBuilder::name)
    /// - [`tax_residences`](GetPlatformSellersResponseBuilder::tax_residences)
    /// - [`address`](GetPlatformSellersResponseBuilder::address)
    /// - [`government_entity`](GetPlatformSellersResponseBuilder::government_entity)
    /// - [`listed_entity`](GetPlatformSellersResponseBuilder::listed_entity)
    /// - [`permanent_establishments`](GetPlatformSellersResponseBuilder::permanent_establishments)
    /// - [`created_at`](GetPlatformSellersResponseBuilder::created_at)
    /// - [`activities`](GetPlatformSellersResponseBuilder::activities)
    pub fn build(self) -> Result<GetPlatformSellersResponse, BuildError> {
        Ok(GetPlatformSellersResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            partner_id: self.partner_id,
            first_name: self.first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
            entity_name: self.entity_name,
            tax_residences: self
                .tax_residences
                .ok_or_else(|| BuildError::missing_field("tax_residences"))?,
            vat_code: self.vat_code,
            business_registration_number: self.business_registration_number,
            address: self
                .address
                .ok_or_else(|| BuildError::missing_field("address"))?,
            birth_date: self.birth_date,
            birth_city: self.birth_city,
            birth_country_code: self.birth_country_code,
            iban: self.iban,
            account_holder_name: self.account_holder_name,
            government_entity: self
                .government_entity
                .ok_or_else(|| BuildError::missing_field("government_entity"))?,
            listed_entity: self
                .listed_entity
                .ok_or_else(|| BuildError::missing_field("listed_entity"))?,
            permanent_establishments: self
                .permanent_establishments
                .ok_or_else(|| BuildError::missing_field("permanent_establishments"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            activities: self
                .activities
                .ok_or_else(|| BuildError::missing_field("activities"))?,
        })
    }
}
