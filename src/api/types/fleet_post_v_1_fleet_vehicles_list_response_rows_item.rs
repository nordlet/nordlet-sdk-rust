pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1FleetVehiclesListResponseRowsItem {
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
    pub acquisition_date: Option<String>,
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
    pub status: PostV1FleetVehiclesListResponseRowsItemStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<PostV1FleetVehiclesListResponseRowsItemDocumentsItem>>,
    #[serde(rename = "currentAssignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_assignment: Option<PostV1FleetVehiclesListResponseRowsItemCurrentAssignment>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1FleetVehiclesListResponseRowsItem {
    pub fn builder() -> PostV1FleetVehiclesListResponseRowsItemBuilder {
        <PostV1FleetVehiclesListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FleetVehiclesListResponseRowsItemBuilder {
    id: Option<String>,
    plate_number: Option<String>,
    make: Option<String>,
    model: Option<String>,
    year: Option<i64>,
    vin: Option<String>,
    fuel_type: Option<String>,
    acquisition_date: Option<String>,
    market_value: Option<String>,
    fixed_asset_id: Option<String>,
    technical_inspection_due: Option<String>,
    insurance_due: Option<String>,
    status: Option<PostV1FleetVehiclesListResponseRowsItemStatus>,
    notes: Option<String>,
    documents: Option<Vec<PostV1FleetVehiclesListResponseRowsItemDocumentsItem>>,
    current_assignment: Option<PostV1FleetVehiclesListResponseRowsItemCurrentAssignment>,
    created_at: Option<String>,
}

impl PostV1FleetVehiclesListResponseRowsItemBuilder {
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

    pub fn status(mut self, value: PostV1FleetVehiclesListResponseRowsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn documents(
        mut self,
        value: Vec<PostV1FleetVehiclesListResponseRowsItemDocumentsItem>,
    ) -> Self {
        self.documents = Some(value);
        self
    }

    pub fn current_assignment(
        mut self,
        value: PostV1FleetVehiclesListResponseRowsItemCurrentAssignment,
    ) -> Self {
        self.current_assignment = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1FleetVehiclesListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1FleetVehiclesListResponseRowsItemBuilder::id)
    /// - [`plate_number`](PostV1FleetVehiclesListResponseRowsItemBuilder::plate_number)
    /// - [`make`](PostV1FleetVehiclesListResponseRowsItemBuilder::make)
    /// - [`model`](PostV1FleetVehiclesListResponseRowsItemBuilder::model)
    /// - [`market_value`](PostV1FleetVehiclesListResponseRowsItemBuilder::market_value)
    /// - [`status`](PostV1FleetVehiclesListResponseRowsItemBuilder::status)
    /// - [`created_at`](PostV1FleetVehiclesListResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1FleetVehiclesListResponseRowsItem, BuildError> {
        Ok(PostV1FleetVehiclesListResponseRowsItem {
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
