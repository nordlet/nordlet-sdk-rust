pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1FleetVehiclesCreateRequest {
    #[serde(rename = "plateNumber")]
    #[serde(default)]
    pub plate_number: String,
    #[serde(default)]
    pub make: String,
    #[serde(default)]
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vin: Option<String>,
    #[serde(rename = "fuelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_type: Option<PostV1FleetVehiclesCreateRequestFuelType>,
    #[serde(rename = "acquisitionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition_date: Option<String>,
    #[serde(rename = "marketValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_value: Option<String>,
    #[serde(rename = "fixedAssetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_asset_id: Option<String>,
    #[serde(rename = "technicalInspectionDue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_inspection_due: Option<String>,
    #[serde(rename = "insuranceDue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurance_due: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1FleetVehiclesCreateRequest {
    pub fn builder() -> PostV1FleetVehiclesCreateRequestBuilder {
        <PostV1FleetVehiclesCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FleetVehiclesCreateRequestBuilder {
    plate_number: Option<String>,
    make: Option<String>,
    model: Option<String>,
    year: Option<i64>,
    vin: Option<String>,
    fuel_type: Option<PostV1FleetVehiclesCreateRequestFuelType>,
    acquisition_date: Option<String>,
    market_value: Option<String>,
    fixed_asset_id: Option<String>,
    technical_inspection_due: Option<String>,
    insurance_due: Option<String>,
    notes: Option<String>,
}

impl PostV1FleetVehiclesCreateRequestBuilder {
    pub fn plate_number(mut self, value: impl Into<String>) -> Self {
        self.plate_number = Some(value.into());
        self
    }

    pub fn make(mut self, value: impl Into<String>) -> Self {
        self.make = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn vin(mut self, value: impl Into<String>) -> Self {
        self.vin = Some(value.into());
        self
    }

    pub fn fuel_type(mut self, value: PostV1FleetVehiclesCreateRequestFuelType) -> Self {
        self.fuel_type = Some(value);
        self
    }

    pub fn acquisition_date(mut self, value: impl Into<String>) -> Self {
        self.acquisition_date = Some(value.into());
        self
    }

    pub fn market_value(mut self, value: impl Into<String>) -> Self {
        self.market_value = Some(value.into());
        self
    }

    pub fn fixed_asset_id(mut self, value: impl Into<String>) -> Self {
        self.fixed_asset_id = Some(value.into());
        self
    }

    pub fn technical_inspection_due(mut self, value: impl Into<String>) -> Self {
        self.technical_inspection_due = Some(value.into());
        self
    }

    pub fn insurance_due(mut self, value: impl Into<String>) -> Self {
        self.insurance_due = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1FleetVehiclesCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`plate_number`](PostV1FleetVehiclesCreateRequestBuilder::plate_number)
    /// - [`make`](PostV1FleetVehiclesCreateRequestBuilder::make)
    /// - [`model`](PostV1FleetVehiclesCreateRequestBuilder::model)
    pub fn build(self) -> Result<PostV1FleetVehiclesCreateRequest, BuildError> {
        Ok(PostV1FleetVehiclesCreateRequest {
            plate_number: self
                .plate_number
                .ok_or_else(|| BuildError::missing_field("plate_number"))?,
            make: self.make.ok_or_else(|| BuildError::missing_field("make"))?,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            year: self.year,
            vin: self.vin,
            fuel_type: self.fuel_type,
            acquisition_date: self.acquisition_date,
            market_value: self.market_value,
            fixed_asset_id: self.fixed_asset_id,
            technical_inspection_due: self.technical_inspection_due,
            insurance_due: self.insurance_due,
            notes: self.notes,
        })
    }
}
