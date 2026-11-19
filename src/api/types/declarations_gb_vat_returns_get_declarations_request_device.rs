pub use crate::prelude::*;

/// Facts of the browser the user files from, which HMRC requires with every VAT (MTD) API call: `navigator.userAgent`, a UUID the browser keeps for the device, every screen (width, height, scaling factor, colour depth in bits), the window size in pixels and the time zone as `UTC±hh:mm`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GbVatReturnsGetDeclarationsRequestDevice {
    #[serde(rename = "userAgent")]
    #[serde(default)]
    pub user_agent: String,
    #[serde(rename = "deviceId")]
    #[serde(default)]
    pub device_id: String,
    #[serde(default)]
    pub screens: Vec<GbVatReturnsGetDeclarationsRequestDeviceScreensItem>,
    #[serde(rename = "windowSize")]
    #[serde(default)]
    pub window_size: GbVatReturnsGetDeclarationsRequestDeviceWindowSize,
    #[serde(default)]
    pub timezone: String,
}

impl GbVatReturnsGetDeclarationsRequestDevice {
    pub fn builder() -> GbVatReturnsGetDeclarationsRequestDeviceBuilder {
        <GbVatReturnsGetDeclarationsRequestDeviceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatReturnsGetDeclarationsRequestDeviceBuilder {
    user_agent: Option<String>,
    device_id: Option<String>,
    screens: Option<Vec<GbVatReturnsGetDeclarationsRequestDeviceScreensItem>>,
    window_size: Option<GbVatReturnsGetDeclarationsRequestDeviceWindowSize>,
    timezone: Option<String>,
}

impl GbVatReturnsGetDeclarationsRequestDeviceBuilder {
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
        value: Vec<GbVatReturnsGetDeclarationsRequestDeviceScreensItem>,
    ) -> Self {
        self.screens = Some(value);
        self
    }

    pub fn window_size(
        mut self,
        value: GbVatReturnsGetDeclarationsRequestDeviceWindowSize,
    ) -> Self {
        self.window_size = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GbVatReturnsGetDeclarationsRequestDevice`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_agent`](GbVatReturnsGetDeclarationsRequestDeviceBuilder::user_agent)
    /// - [`device_id`](GbVatReturnsGetDeclarationsRequestDeviceBuilder::device_id)
    /// - [`screens`](GbVatReturnsGetDeclarationsRequestDeviceBuilder::screens)
    /// - [`window_size`](GbVatReturnsGetDeclarationsRequestDeviceBuilder::window_size)
    /// - [`timezone`](GbVatReturnsGetDeclarationsRequestDeviceBuilder::timezone)
    pub fn build(self) -> Result<GbVatReturnsGetDeclarationsRequestDevice, BuildError> {
        Ok(GbVatReturnsGetDeclarationsRequestDevice {
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
