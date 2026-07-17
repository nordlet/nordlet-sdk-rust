pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PosDevicesCreateRequest {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "serialNumber")]
    #[serde(default)]
    pub serial_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "registrationNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

impl PostV1PosDevicesCreateRequest {
    pub fn builder() -> PostV1PosDevicesCreateRequestBuilder {
        <PostV1PosDevicesCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PosDevicesCreateRequestBuilder {
    name: Option<String>,
    serial_number: Option<String>,
    model: Option<String>,
    registration_number: Option<String>,
    address: Option<String>,
}

impl PostV1PosDevicesCreateRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostV1PosDevicesCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1PosDevicesCreateRequestBuilder::name)
    /// - [`serial_number`](PostV1PosDevicesCreateRequestBuilder::serial_number)
    pub fn build(self) -> Result<PostV1PosDevicesCreateRequest, BuildError> {
        Ok(PostV1PosDevicesCreateRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            serial_number: self
                .serial_number
                .ok_or_else(|| BuildError::missing_field("serial_number"))?,
            model: self.model,
            registration_number: self.registration_number,
            address: self.address,
        })
    }
}
