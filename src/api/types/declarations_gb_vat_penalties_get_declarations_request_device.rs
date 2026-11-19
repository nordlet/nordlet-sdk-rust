pub use crate::prelude::*;

/// Facts of the browser the user files from, which HMRC requires with every VAT (MTD) API call: `navigator.userAgent`, a UUID the browser keeps for the device, every screen (width, height, scaling factor, colour depth in bits), the window size in pixels and the time zone as `UTC±hh:mm`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GbVatPenaltiesGetDeclarationsRequestDevice {
    #[serde(rename = "userAgent")]
    #[serde(default)]
    pub user_agent: String,
    #[serde(rename = "deviceId")]
    #[serde(default)]
    pub device_id: String,
    #[serde(default)]
    pub screens: Vec<GbVatPenaltiesGetDeclarationsRequestDeviceScreensItem>,
    #[serde(rename = "windowSize")]
    #[serde(default)]
    pub window_size: GbVatPenaltiesGetDeclarationsRequestDeviceWindowSize,
    #[serde(default)]
    pub timezone: String,
}

impl GbVatPenaltiesGetDeclarationsRequestDevice {
    pub fn builder() -> GbVatPenaltiesGetDeclarationsRequestDeviceBuilder {
        <GbVatPenaltiesGetDeclarationsRequestDeviceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatPenaltiesGetDeclarationsRequestDeviceBuilder {
    user_agent: Option<String>,
    device_id: Option<String>,
    screens: Option<Vec<GbVatPenaltiesGetDeclarationsRequestDeviceScreensItem>>,
    window_size: Option<GbVatPenaltiesGetDeclarationsRequestDeviceWindowSize>,
    timezone: Option<String>,
}

impl GbVatPenaltiesGetDeclarationsRequestDeviceBuilder {
    pub fn user_agent(mut self, value: impl Into<String>) -> Self {
        self.user_agent = Some(value.into());
        self
    }

    pub fn device_id(mut self, value: impl Into<String>) -> Self {
        self.device_id = Some(value.into());
        self
    }

    pub fn screens(
        mut self,
        value: Vec<GbVatPenaltiesGetDeclarationsRequestDeviceScreensItem>,
    ) -> Self {
        self.screens = Some(value);
        self
    }

    pub fn window_size(
        mut self,
        value: GbVatPenaltiesGetDeclarationsRequestDeviceWindowSize,
    ) -> Self {
        self.window_size = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GbVatPenaltiesGetDeclarationsRequestDevice`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_agent`](GbVatPenaltiesGetDeclarationsRequestDeviceBuilder::user_agent)
    /// - [`device_id`](GbVatPenaltiesGetDeclarationsRequestDeviceBuilder::device_id)
    /// - [`screens`](GbVatPenaltiesGetDeclarationsRequestDeviceBuilder::screens)
    /// - [`window_size`](GbVatPenaltiesGetDeclarationsRequestDeviceBuilder::window_size)
    /// - [`timezone`](GbVatPenaltiesGetDeclarationsRequestDeviceBuilder::timezone)
    pub fn build(self) -> Result<GbVatPenaltiesGetDeclarationsRequestDevice, BuildError> {
        Ok(GbVatPenaltiesGetDeclarationsRequestDevice {
            user_agent: self
                .user_agent
                .ok_or_else(|| BuildError::missing_field("user_agent"))?,
            device_id: self
                .device_id
                .ok_or_else(|| BuildError::missing_field("device_id"))?,
            screens: self
                .screens
                .ok_or_else(|| BuildError::missing_field("screens"))?,
            window_size: self
                .window_size
                .ok_or_else(|| BuildError::missing_field("window_size"))?,
            timezone: self
                .timezone
                .ok_or_else(|| BuildError::missing_field("timezone"))?,
        })
    }
}
