pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreatePlatformSellersRequest {
    pub kind: CreatePlatformSellersRequestKind,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_residences: Option<Vec<CreatePlatformSellersRequestTaxResidencesItem>>,
    #[serde(rename = "vatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<String>,
    #[serde(rename = "businessRegistrationNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_registration_number: Option<String>,
    #[serde(default)]
    pub address: CreatePlatformSellersRequestAddress,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub government_entity: Option<bool>,
    #[serde(rename = "listedEntity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listed_entity: Option<bool>,
    #[serde(rename = "permanentEstablishments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent_establishments: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activities: Option<Vec<CreatePlatformSellersRequestActivitiesItem>>,
}

impl CreatePlatformSellersRequest {
    pub fn builder() -> CreatePlatformSellersRequestBuilder {
        <CreatePlatformSellersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePlatformSellersRequestBuilder {
    kind: Option<CreatePlatformSellersRequestKind>,
    partner_id: Option<String>,
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
    entity_name: Option<String>,
    tax_residences: Option<Vec<CreatePlatformSellersRequestTaxResidencesItem>>,
    vat_code: Option<String>,
    business_registration_number: Option<String>,
    address: Option<CreatePlatformSellersRequestAddress>,
    birth_date: Option<NaiveDate>,
    birth_city: Option<String>,
    birth_country_code: Option<String>,
    iban: Option<String>,
    account_holder_name: Option<String>,
    government_entity: Option<bool>,
    listed_entity: Option<bool>,
    permanent_establishments: Option<Vec<String>>,
    activities: Option<Vec<CreatePlatformSellersRequestActivitiesItem>>,
}

impl CreatePlatformSellersRequestBuilder {
    pub fn kind(mut self, value: CreatePlatformSellersRequestKind) -> Self {
        self.kind = Some(value);
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
        value: Vec<CreatePlatformSellersRequestTaxResidencesItem>,
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

    pub fn address(mut self, value: CreatePlatformSellersRequestAddress) -> Self {
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

    pub fn activities(mut self, value: Vec<CreatePlatformSellersRequestActivitiesItem>) -> Self {
        self.activities = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePlatformSellersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](CreatePlatformSellersRequestBuilder::kind)
    /// - [`address`](CreatePlatformSellersRequestBuilder::address)
    pub fn build(self) -> Result<CreatePlatformSellersRequest, BuildError> {
        Ok(CreatePlatformSellersRequest {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            partner_id: self.partner_id,
            first_name: self.first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
            entity_name: self.entity_name,
            tax_residences: self.tax_residences,
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
            government_entity: self.government_entity,
            listed_entity: self.listed_entity,
            permanent_establishments: self.permanent_establishments,
            activities: self.activities,
        })
    }
}
