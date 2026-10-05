pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RoEtransportBuildDeclarationsResponse {
    #[serde(rename = "waybillId")]
    #[serde(default)]
    pub waybill_id: String,
    #[serde(rename = "fileId")]
    #[serde(default)]
    pub file_id: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub xml: String,
    #[serde(rename = "operationType")]
    #[serde(default)]
    pub operation_type: String,
    #[serde(rename = "vehiclePlate")]
    #[serde(default)]
    pub vehicle_plate: String,
    #[serde(default)]
    pub blockers: Vec<String>,
    #[serde(default)]
    pub goods: i64,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl RoEtransportBuildDeclarationsResponse {
    pub fn builder() -> RoEtransportBuildDeclarationsResponseBuilder {
        <RoEtransportBuildDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RoEtransportBuildDeclarationsResponseBuilder {
    waybill_id: Option<String>,
    file_id: Option<String>,
    file_name: Option<String>,
    xml: Option<String>,
    operation_type: Option<String>,
    vehicle_plate: Option<String>,
    blockers: Option<Vec<String>>,
    goods: Option<i64>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl RoEtransportBuildDeclarationsResponseBuilder {
    pub fn waybill_id(mut self, value: impl Into<String>) -> Self {
        self.waybill_id = Some(value.into());
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn xml(mut self, value: impl Into<String>) -> Self {
        self.xml = Some(value.into());
        self
    }

    pub fn operation_type(mut self, value: impl Into<String>) -> Self {
        self.operation_type = Some(value.into());
        self
    }

    pub fn vehicle_plate(mut self, value: impl Into<String>) -> Self {
        self.vehicle_plate = Some(value.into());
        self
    }

    pub fn blockers(mut self, value: Vec<String>) -> Self {
        self.blockers = Some(value);
        self
    }

    pub fn goods(mut self, value: i64) -> Self {
        self.goods = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn notes(mut self, value: Vec<String>) -> Self {
        self.notes = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RoEtransportBuildDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`waybill_id`](RoEtransportBuildDeclarationsResponseBuilder::waybill_id)
    /// - [`file_id`](RoEtransportBuildDeclarationsResponseBuilder::file_id)
    /// - [`file_name`](RoEtransportBuildDeclarationsResponseBuilder::file_name)
    /// - [`xml`](RoEtransportBuildDeclarationsResponseBuilder::xml)
    /// - [`operation_type`](RoEtransportBuildDeclarationsResponseBuilder::operation_type)
    /// - [`vehicle_plate`](RoEtransportBuildDeclarationsResponseBuilder::vehicle_plate)
    /// - [`blockers`](RoEtransportBuildDeclarationsResponseBuilder::blockers)
    /// - [`goods`](RoEtransportBuildDeclarationsResponseBuilder::goods)
    /// - [`warnings`](RoEtransportBuildDeclarationsResponseBuilder::warnings)
    /// - [`notes`](RoEtransportBuildDeclarationsResponseBuilder::notes)
    /// - [`source`](RoEtransportBuildDeclarationsResponseBuilder::source)
    pub fn build(self) -> Result<RoEtransportBuildDeclarationsResponse, BuildError> {
        Ok(RoEtransportBuildDeclarationsResponse {
            waybill_id: self
                .waybill_id
                .ok_or_else(|| BuildError::missing_field("waybill_id"))?,
            file_id: self
                .file_id
                .ok_or_else(|| BuildError::missing_field("file_id"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            xml: self.xml.ok_or_else(|| BuildError::missing_field("xml"))?,
            operation_type: self
                .operation_type
                .ok_or_else(|| BuildError::missing_field("operation_type"))?,
            vehicle_plate: self
                .vehicle_plate
                .ok_or_else(|| BuildError::missing_field("vehicle_plate"))?,
            blockers: self
                .blockers
                .ok_or_else(|| BuildError::missing_field("blockers"))?,
            goods: self
                .goods
                .ok_or_else(|| BuildError::missing_field("goods"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
