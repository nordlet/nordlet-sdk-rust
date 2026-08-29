pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1FleetNaturaPreviewResponseRowsItem {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(rename = "employeeName")]
    #[serde(default)]
    pub employee_name: String,
    #[serde(rename = "vehicleId")]
    #[serde(default)]
    pub vehicle_id: String,
    #[serde(rename = "plateNumber")]
    #[serde(default)]
    pub plate_number: String,
    #[serde(default)]
    pub make: String,
    #[serde(default)]
    pub model: String,
    #[serde(rename = "marketValue")]
    #[serde(default)]
    pub market_value: String,
    #[serde(rename = "employerPaysFuel")]
    #[serde(default)]
    pub employer_pays_fuel: bool,
    #[serde(rename = "ratePercent")]
    #[serde(default)]
    pub rate_percent: String,
    #[serde(default)]
    pub amount: String,
}

impl PostV1FleetNaturaPreviewResponseRowsItem {
    pub fn builder() -> PostV1FleetNaturaPreviewResponseRowsItemBuilder {
        <PostV1FleetNaturaPreviewResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1FleetNaturaPreviewResponseRowsItemBuilder {
    employee_id: Option<String>,
    employee_name: Option<String>,
    vehicle_id: Option<String>,
    plate_number: Option<String>,
    make: Option<String>,
    model: Option<String>,
    market_value: Option<String>,
    employer_pays_fuel: Option<bool>,
    rate_percent: Option<String>,
    amount: Option<String>,
}

impl PostV1FleetNaturaPreviewResponseRowsItemBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn employee_name(mut self, value: impl Into<String>) -> Self {
        self.employee_name = Some(value.into());
        self
    }

    pub fn vehicle_id(mut self, value: impl Into<String>) -> Self {
        self.vehicle_id = Some(value.into());
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

    pub fn market_value(mut self, value: impl Into<String>) -> Self {
        self.market_value = Some(value.into());
        self
    }

    pub fn employer_pays_fuel(mut self, value: bool) -> Self {
        self.employer_pays_fuel = Some(value);
        self
    }

    pub fn rate_percent(mut self, value: impl Into<String>) -> Self {
        self.rate_percent = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1FleetNaturaPreviewResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](PostV1FleetNaturaPreviewResponseRowsItemBuilder::employee_id)
    /// - [`employee_name`](PostV1FleetNaturaPreviewResponseRowsItemBuilder::employee_name)
    /// - [`vehicle_id`](PostV1FleetNaturaPreviewResponseRowsItemBuilder::vehicle_id)
    /// - [`plate_number`](PostV1FleetNaturaPreviewResponseRowsItemBuilder::plate_number)
    /// - [`make`](PostV1FleetNaturaPreviewResponseRowsItemBuilder::make)
    /// - [`model`](PostV1FleetNaturaPreviewResponseRowsItemBuilder::model)
    /// - [`market_value`](PostV1FleetNaturaPreviewResponseRowsItemBuilder::market_value)
    /// - [`employer_pays_fuel`](PostV1FleetNaturaPreviewResponseRowsItemBuilder::employer_pays_fuel)
    /// - [`rate_percent`](PostV1FleetNaturaPreviewResponseRowsItemBuilder::rate_percent)
    /// - [`amount`](PostV1FleetNaturaPreviewResponseRowsItemBuilder::amount)
    pub fn build(self) -> Result<PostV1FleetNaturaPreviewResponseRowsItem, BuildError> {
        Ok(PostV1FleetNaturaPreviewResponseRowsItem {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            employee_name: self
                .employee_name
                .ok_or_else(|| BuildError::missing_field("employee_name"))?,
            vehicle_id: self
                .vehicle_id
                .ok_or_else(|| BuildError::missing_field("vehicle_id"))?,
            plate_number: self
                .plate_number
                .ok_or_else(|| BuildError::missing_field("plate_number"))?,
            make: self.make.ok_or_else(|| BuildError::missing_field("make"))?,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            market_value: self
                .market_value
                .ok_or_else(|| BuildError::missing_field("market_value"))?,
            employer_pays_fuel: self
                .employer_pays_fuel
                .ok_or_else(|| BuildError::missing_field("employer_pays_fuel"))?,
            rate_percent: self
                .rate_percent
                .ok_or_else(|| BuildError::missing_field("rate_percent"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
