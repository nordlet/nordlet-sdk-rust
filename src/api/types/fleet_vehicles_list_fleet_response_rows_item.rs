pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VehiclesListFleetResponseRowsItem {
    #[serde(default)]
    pub id: String,
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
    pub fuel_type: Option<String>,
    #[serde(rename = "acquisitionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition_date: Option<NaiveDate>,
    #[serde(rename = "marketValue")]
    #[serde(default)]
    pub market_value: String,
    #[serde(rename = "fixedAssetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_asset_id: Option<String>,
    #[serde(rename = "technicalInspectionDue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_inspection_due: Option<String>,
    #[serde(rename = "insuranceDue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurance_due: Option<String>,
    pub status: VehiclesListFleetResponseRowsItemStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<VehiclesListFleetResponseRowsItemDocumentsItem>>,
    #[serde(rename = "currentAssignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_assignment: Option<VehiclesListFleetResponseRowsItemCurrentAssignment>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl VehiclesListFleetResponseRowsItem {
    pub fn builder() -> VehiclesListFleetResponseRowsItemBuilder {
        <VehiclesListFleetResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VehiclesListFleetResponseRowsItemBuilder {
    id: Option<String>,
    plate_number: Option<String>,
    make: Option<String>,
    model: Option<String>,
    year: Option<i64>,
    vin: Option<String>,
    fuel_type: Option<String>,
    acquisition_date: Option<NaiveDate>,
    market_value: Option<String>,
    fixed_asset_id: Option<String>,
    technical_inspection_due: Option<String>,
    insurance_due: Option<String>,
    status: Option<VehiclesListFleetResponseRowsItemStatus>,
    notes: Option<String>,
    documents: Option<Vec<VehiclesListFleetResponseRowsItemDocumentsItem>>,
    current_assignment: Option<VehiclesListFleetResponseRowsItemCurrentAssignment>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl VehiclesListFleetResponseRowsItemBuilder {
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

    pub fn fuel_type(mut self, value: impl Into<String>) -> Self {
        self.fuel_type = Some(value.into());
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

    pub fn technical_inspection_due(mut self, value: impl Into<String>) -> Self {
        self.technical_inspection_due = Some(value.into());
        self
    }

    pub fn insurance_due(mut self, value: impl Into<String>) -> Self {
        self.insurance_due = Some(value.into());
        self
    }

    pub fn status(mut self, value: VehiclesListFleetResponseRowsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn documents(mut self, value: Vec<VehiclesListFleetResponseRowsItemDocumentsItem>) -> Self {
        self.documents = Some(value);
        self
    }

    pub fn current_assignment(
        mut self,
        value: VehiclesListFleetResponseRowsItemCurrentAssignment,
    ) -> Self {
        self.current_assignment = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VehiclesListFleetResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](VehiclesListFleetResponseRowsItemBuilder::id)
    /// - [`plate_number`](VehiclesListFleetResponseRowsItemBuilder::plate_number)
    /// - [`make`](VehiclesListFleetResponseRowsItemBuilder::make)
    /// - [`model`](VehiclesListFleetResponseRowsItemBuilder::model)
    /// - [`market_value`](VehiclesListFleetResponseRowsItemBuilder::market_value)
    /// - [`status`](VehiclesListFleetResponseRowsItemBuilder::status)
    /// - [`created_at`](VehiclesListFleetResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<VehiclesListFleetResponseRowsItem, BuildError> {
        Ok(VehiclesListFleetResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
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
            market_value: self
                .market_value
                .ok_or_else(|| BuildError::missing_field("market_value"))?,
            fixed_asset_id: self.fixed_asset_id,
            technical_inspection_due: self.technical_inspection_due,
            insurance_due: self.insurance_due,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            notes: self.notes,
            documents: self.documents,
            current_assignment: self.current_assignment,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
