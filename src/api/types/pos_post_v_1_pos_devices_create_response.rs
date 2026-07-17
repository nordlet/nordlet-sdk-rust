pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PosDevicesCreateResponse {
    #[serde(default)]
    pub id: String,
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
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1PosDevicesCreateResponse {
    pub fn builder() -> PostV1PosDevicesCreateResponseBuilder {
        <PostV1PosDevicesCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PosDevicesCreateResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    serial_number: Option<String>,
    model: Option<String>,
    registration_number: Option<String>,
    address: Option<String>,
    is_active: Option<bool>,
    created_at: Option<String>,
}

impl PostV1PosDevicesCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PosDevicesCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PosDevicesCreateResponseBuilder::id)
    /// - [`name`](PostV1PosDevicesCreateResponseBuilder::name)
    /// - [`serial_number`](PostV1PosDevicesCreateResponseBuilder::serial_number)
    /// - [`is_active`](PostV1PosDevicesCreateResponseBuilder::is_active)
    /// - [`created_at`](PostV1PosDevicesCreateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1PosDevicesCreateResponse, BuildError> {
        Ok(PostV1PosDevicesCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            serial_number: self
                .serial_number
                .ok_or_else(|| BuildError::missing_field("serial_number"))?,
            model: self.model,
            registration_number: self.registration_number,
            address: self.address,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
