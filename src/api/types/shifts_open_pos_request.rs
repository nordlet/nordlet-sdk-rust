pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ShiftsOpenPosRequest {
    #[serde(rename = "deviceId")]
    #[serde(default)]
    pub device_id: String,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    #[serde(rename = "openingCash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_cash: Option<String>,
}

impl ShiftsOpenPosRequest {
    pub fn builder() -> ShiftsOpenPosRequestBuilder {
        <ShiftsOpenPosRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShiftsOpenPosRequestBuilder {
    device_id: Option<String>,
    warehouse_id: Option<String>,
    opening_cash: Option<String>,
}

impl ShiftsOpenPosRequestBuilder {
    pub fn device_id(mut self, value: impl Into<String>) -> Self {
        self.device_id = Some(value.into());
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    pub fn opening_cash(mut self, value: impl Into<String>) -> Self {
        self.opening_cash = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ShiftsOpenPosRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`device_id`](ShiftsOpenPosRequestBuilder::device_id)
    pub fn build(self) -> Result<ShiftsOpenPosRequest, BuildError> {
        Ok(ShiftsOpenPosRequest {
            device_id: self
                .device_id
                .ok_or_else(|| BuildError::missing_field("device_id"))?,
            warehouse_id: self.warehouse_id,
            opening_cash: self.opening_cash,
        })
    }
}
