pub use crate::prelude::*;

/// Facts of the browser the user files from, which HMRC requires with every VAT (MTD) API call: `navigator.userAgent`, a UUID the browser keeps for the device, every screen (width, height, scaling factor, colour depth in bits), the window size in pixels and the time zone as `UTC±hh:mm`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubmitCalendarRequestDevice {
    #[serde(rename = "userAgent")]
    #[serde(default)]
    pub user_agent: String,
    #[serde(rename = "deviceId")]
    #[serde(default)]
    pub device_id: String,
    #[serde(default)]
    pub screens: Vec<SubmitCalendarRequestDeviceScreensItem>,
    #[serde(rename = "windowSize")]
    #[serde(default)]
    pub window_size: SubmitCalendarRequestDeviceWindowSize,
    #[serde(default)]
    pub timezone: String,
}

impl SubmitCalendarRequestDevice {
    pub fn builder() -> SubmitCalendarRequestDeviceBuilder {
        <SubmitCalendarRequestDeviceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitCalendarRequestDeviceBuilder {
    user_agent: Option<String>,
    device_id: Option<String>,
    screens: Option<Vec<SubmitCalendarRequestDeviceScreensItem>>,
    window_size: Option<SubmitCalendarRequestDeviceWindowSize>,
    timezone: Option<String>,
}

impl SubmitCalendarRequestDeviceBuilder {
    pub fn user_agent(mut self, value: impl Into<String>) -> Self {
        self.user_agent = Some(value.into());
        self
    }

    pub fn device_id(mut self, value: impl Into<String>) -> Self {
        self.device_id = Some(value.into());
        self
    }

    pub fn screens(mut self, value: Vec<SubmitCalendarRequestDeviceScreensItem>) -> Self {
        self.screens = Some(value);
        self
    }

    pub fn window_size(mut self, value: SubmitCalendarRequestDeviceWindowSize) -> Self {
        self.window_size = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubmitCalendarRequestDevice`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_agent`](SubmitCalendarRequestDeviceBuilder::user_agent)
    /// - [`device_id`](SubmitCalendarRequestDeviceBuilder::device_id)
    /// - [`screens`](SubmitCalendarRequestDeviceBuilder::screens)
    /// - [`window_size`](SubmitCalendarRequestDeviceBuilder::window_size)
    /// - [`timezone`](SubmitCalendarRequestDeviceBuilder::timezone)
    pub fn build(self) -> Result<SubmitCalendarRequestDevice, BuildError> {
        Ok(SubmitCalendarRequestDevice {
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
