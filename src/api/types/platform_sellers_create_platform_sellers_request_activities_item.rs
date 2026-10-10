pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreatePlatformSellersRequestActivitiesItem {
    #[serde(default)]
    pub year: i64,
    pub activity: CreatePlatformSellersRequestActivitiesItemActivity,
    #[serde(rename = "propertyAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_address: Option<CreatePlatformSellersRequestActivitiesItemPropertyAddress>,
    #[serde(rename = "landRegistrationNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub land_registration_number: Option<String>,
    #[serde(rename = "propertyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_type: Option<CreatePlatformSellersRequestActivitiesItemPropertyType>,
    #[serde(rename = "otherPropertyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_property_type: Option<String>,
    #[serde(rename = "rentedDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rented_days: Option<i64>,
    #[serde(default)]
    pub consideration: Vec<String>,
    #[serde(default)]
    pub fees: Vec<String>,
    #[serde(default)]
    pub taxes: Vec<String>,
    #[serde(rename = "numberOfActivities")]
    #[serde(default)]
    pub number_of_activities: Vec<i64>,
}

impl CreatePlatformSellersRequestActivitiesItem {
    pub fn builder() -> CreatePlatformSellersRequestActivitiesItemBuilder {
        <CreatePlatformSellersRequestActivitiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePlatformSellersRequestActivitiesItemBuilder {
    year: Option<i64>,
    activity: Option<CreatePlatformSellersRequestActivitiesItemActivity>,
    property_address: Option<CreatePlatformSellersRequestActivitiesItemPropertyAddress>,
    land_registration_number: Option<String>,
    property_type: Option<CreatePlatformSellersRequestActivitiesItemPropertyType>,
    other_property_type: Option<String>,
    rented_days: Option<i64>,
    consideration: Option<Vec<String>>,
    fees: Option<Vec<String>>,
    taxes: Option<Vec<String>>,
    number_of_activities: Option<Vec<i64>>,
}

impl CreatePlatformSellersRequestActivitiesItemBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn activity(mut self, value: CreatePlatformSellersRequestActivitiesItemActivity) -> Self {
        self.activity = Some(value);
        self
    }

    pub fn property_address(
        mut self,
        value: CreatePlatformSellersRequestActivitiesItemPropertyAddress,
    ) -> Self {
        self.property_address = Some(value);
        self
    }

    pub fn land_registration_number(mut self, value: impl Into<String>) -> Self {
        self.land_registration_number = Some(value.into());
        self
    }

    pub fn property_type(
        mut self,
        value: CreatePlatformSellersRequestActivitiesItemPropertyType,
    ) -> Self {
        self.property_type = Some(value);
        self
    }

    pub fn other_property_type(mut self, value: impl Into<String>) -> Self {
        self.other_property_type = Some(value.into());
        self
    }

    pub fn rented_days(mut self, value: i64) -> Self {
        self.rented_days = Some(value);
        self
    }

    pub fn consideration(mut self, value: Vec<String>) -> Self {
        self.consideration = Some(value);
        self
    }

    pub fn fees(mut self, value: Vec<String>) -> Self {
        self.fees = Some(value);
        self
    }

    pub fn taxes(mut self, value: Vec<String>) -> Self {
        self.taxes = Some(value);
        self
    }

    pub fn number_of_activities(mut self, value: Vec<i64>) -> Self {
        self.number_of_activities = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePlatformSellersRequestActivitiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](CreatePlatformSellersRequestActivitiesItemBuilder::year)
    /// - [`activity`](CreatePlatformSellersRequestActivitiesItemBuilder::activity)
    /// - [`consideration`](CreatePlatformSellersRequestActivitiesItemBuilder::consideration)
    /// - [`fees`](CreatePlatformSellersRequestActivitiesItemBuilder::fees)
    /// - [`taxes`](CreatePlatformSellersRequestActivitiesItemBuilder::taxes)
    /// - [`number_of_activities`](CreatePlatformSellersRequestActivitiesItemBuilder::number_of_activities)
    pub fn build(self) -> Result<CreatePlatformSellersRequestActivitiesItem, BuildError> {
        Ok(CreatePlatformSellersRequestActivitiesItem {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            activity: self
                .activity
                .ok_or_else(|| BuildError::missing_field("activity"))?,
            property_address: self.property_address,
            land_registration_number: self.land_registration_number,
            property_type: self.property_type,
            other_property_type: self.other_property_type,
            rented_days: self.rented_days,
            consideration: self
                .consideration
                .ok_or_else(|| BuildError::missing_field("consideration"))?,
            fees: self.fees.ok_or_else(|| BuildError::missing_field("fees"))?,
            taxes: self
                .taxes
                .ok_or_else(|| BuildError::missing_field("taxes"))?,
            number_of_activities: self
                .number_of_activities
                .ok_or_else(|| BuildError::missing_field("number_of_activities"))?,
        })
    }
}
