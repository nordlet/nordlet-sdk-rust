pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VehiclesUpdateFleetRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "plateNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plate_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vin: Option<String>,
    #[serde(rename = "fuelType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_type: Option<VehiclesUpdateFleetRequestFuelType>,
    #[serde(rename = "acquisitionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition_date: Option<NaiveDate>,
    #[serde(rename = "marketValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_value: Option<String>,
    #[serde(rename = "fixedAssetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_asset_id: Option<String>,
    #[serde(rename = "technicalInspectionDue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_inspection_due: Option<NaiveDate>,
    #[serde(rename = "insuranceDue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurance_due: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<VehiclesUpdateFleetRequestStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl VehiclesUpdateFleetRequest {
    pub fn builder() -> VehiclesUpdateFleetRequestBuilder {
        <VehiclesUpdateFleetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VehiclesUpdateFleetRequestBuilder {
    id: Option<String>,
    plate_number: Option<String>,
    make: Option<String>,
    model: Option<String>,
    year: Option<i64>,
    vin: Option<String>,
    fuel_type: Option<VehiclesUpdateFleetRequestFuelType>,
    acquisition_date: Option<NaiveDate>,
    market_value: Option<String>,
    fixed_asset_id: Option<String>,
    technical_inspection_due: Option<NaiveDate>,
    insurance_due: Option<NaiveDate>,
    status: Option<VehiclesUpdateFleetRequestStatus>,
    notes: Option<String>,
}

impl VehiclesUpdateFleetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

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

    pub fn fuel_type(mut self, value: VehiclesUpdateFleetRequestFuelType) -> Self {
        self.fuel_type = Some(value);
        self
    }

    pub fn acquisition_date(mut self, value: NaiveDate) -> Self {
        self.acquisition_date = Some(value);
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

    pub fn technical_inspection_due(mut self, value: NaiveDate) -> Self {
        self.technical_inspection_due = Some(value);
        self
    }

    pub fn insurance_due(mut self, value: NaiveDate) -> Self {
        self.insurance_due = Some(value);
        self
    }

    pub fn status(mut self, value: VehiclesUpdateFleetRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VehiclesUpdateFleetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](VehiclesUpdateFleetRequestBuilder::id)
    pub fn build(self) -> Result<VehiclesUpdateFleetRequest, BuildError> {
        Ok(VehiclesUpdateFleetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            plate_number: self.plate_number,
            make: self.make,
            model: self.model,
            year: self.year,
            vin: self.vin,
            fuel_type: self.fuel_type,
            acquisition_date: self.acquisition_date,
            market_value: self.market_value,
            fixed_asset_id: self.fixed_asset_id,
            technical_inspection_due: self.technical_inspection_due,
            insurance_due: self.insurance_due,
            status: self.status,
            notes: self.notes,
        })
    }
}
