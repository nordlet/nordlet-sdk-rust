pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DevicesUpdatePosRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "registrationNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

impl DevicesUpdatePosRequest {
    pub fn builder() -> DevicesUpdatePosRequestBuilder {
        <DevicesUpdatePosRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DevicesUpdatePosRequestBuilder {
    id: Option<String>,
    is_active: Option<bool>,
    name: Option<String>,
    serial_number: Option<String>,
    model: Option<String>,
    registration_number: Option<String>,
    address: Option<String>,
}

impl DevicesUpdatePosRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn serial_number(mut self, value: impl Into<String>) -> Self {
        self.serial_number = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn registration_number(mut self, value: impl Into<String>) -> Self {
        self.registration_number = Some(value.into());
        self
    }

    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DevicesUpdatePosRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DevicesUpdatePosRequestBuilder::id)
    pub fn build(self) -> Result<DevicesUpdatePosRequest, BuildError> {
        Ok(DevicesUpdatePosRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_active: self.is_active,
            name: self.name,
            serial_number: self.serial_number,
            model: self.model,
            registration_number: self.registration_number,
            address: self.address,
        })
    }
}
