pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtIvazAmendDeclarationsRequestCorrectionsItem {
    #[serde(rename = "waybillId")]
    #[serde(default)]
    pub waybill_id: String,
    #[serde(rename = "consigneePartnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consignee_partner_id: Option<String>,
    #[serde(rename = "unloadAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unload_address: Option<String>,
    #[serde(rename = "vehiclePlate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_plate: Option<String>,
    #[serde(rename = "trailerPlate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailer_plate: Option<String>,
    #[serde(rename = "driverName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_name: Option<String>,
    #[serde(rename = "driverSurname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_surname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<LtIvazAmendDeclarationsRequestCorrectionsItemLinesItem>>,
}

impl LtIvazAmendDeclarationsRequestCorrectionsItem {
    pub fn builder() -> LtIvazAmendDeclarationsRequestCorrectionsItemBuilder {
        <LtIvazAmendDeclarationsRequestCorrectionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtIvazAmendDeclarationsRequestCorrectionsItemBuilder {
    waybill_id: Option<String>,
    consignee_partner_id: Option<String>,
    unload_address: Option<String>,
    vehicle_plate: Option<String>,
    trailer_plate: Option<String>,
    driver_name: Option<String>,
    driver_surname: Option<String>,
    lines: Option<Vec<LtIvazAmendDeclarationsRequestCorrectionsItemLinesItem>>,
}

impl LtIvazAmendDeclarationsRequestCorrectionsItemBuilder {
    pub fn waybill_id(mut self, value: impl Into<String>) -> Self {
        self.waybill_id = Some(value.into());
        self
    }

    pub fn consignee_partner_id(mut self, value: impl Into<String>) -> Self {
        self.consignee_partner_id = Some(value.into());
        self
    }

    pub fn unload_address(mut self, value: impl Into<String>) -> Self {
        self.unload_address = Some(value.into());
        self
    }

    pub fn vehicle_plate(mut self, value: impl Into<String>) -> Self {
        self.vehicle_plate = Some(value.into());
        self
    }

    pub fn trailer_plate(mut self, value: impl Into<String>) -> Self {
        self.trailer_plate = Some(value.into());
        self
    }

    pub fn driver_name(mut self, value: impl Into<String>) -> Self {
        self.driver_name = Some(value.into());
        self
    }

    pub fn driver_surname(mut self, value: impl Into<String>) -> Self {
        self.driver_surname = Some(value.into());
        self
    }

    pub fn lines(
        mut self,
        value: Vec<LtIvazAmendDeclarationsRequestCorrectionsItemLinesItem>,
    ) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtIvazAmendDeclarationsRequestCorrectionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`waybill_id`](LtIvazAmendDeclarationsRequestCorrectionsItemBuilder::waybill_id)
    pub fn build(self) -> Result<LtIvazAmendDeclarationsRequestCorrectionsItem, BuildError> {
        Ok(LtIvazAmendDeclarationsRequestCorrectionsItem {
            waybill_id: self
                .waybill_id
                .ok_or_else(|| BuildError::missing_field("waybill_id"))?,
            consignee_partner_id: self.consignee_partner_id,
            unload_address: self.unload_address,
            vehicle_plate: self.vehicle_plate,
            trailer_plate: self.trailer_plate,
            driver_name: self.driver_name,
            driver_surname: self.driver_surname,
            lines: self.lines,
        })
    }
}
