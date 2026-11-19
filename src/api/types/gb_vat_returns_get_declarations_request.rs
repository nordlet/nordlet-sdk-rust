pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GbVatReturnsGetDeclarationsRequest {
    /// Facts of the browser the user files from, which HMRC requires with every VAT (MTD) API call: `navigator.userAgent`, a UUID the browser keeps for the device, every screen (width, height, scaling factor, colour depth in bits), the window size in pixels and the time zone as `UTC±hh:mm`.
    #[serde(default)]
    pub device: GbVatReturnsGetDeclarationsRequestDevice,
    #[serde(rename = "periodKey")]
    #[serde(default)]
    pub period_key: String,
}

impl GbVatReturnsGetDeclarationsRequest {
    pub fn builder() -> GbVatReturnsGetDeclarationsRequestBuilder {
        <GbVatReturnsGetDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatReturnsGetDeclarationsRequestBuilder {
    device: Option<GbVatReturnsGetDeclarationsRequestDevice>,
    period_key: Option<String>,
}

impl GbVatReturnsGetDeclarationsRequestBuilder {
    pub fn device(mut self, value: GbVatReturnsGetDeclarationsRequestDevice) -> Self {
        self.device = Some(value);
        self
    }

    pub fn period_key(mut self, value: impl Into<String>) -> Self {
        self.period_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GbVatReturnsGetDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`device`](GbVatReturnsGetDeclarationsRequestBuilder::device)
    /// - [`period_key`](GbVatReturnsGetDeclarationsRequestBuilder::period_key)
    pub fn build(self) -> Result<GbVatReturnsGetDeclarationsRequest, BuildError> {
        Ok(GbVatReturnsGetDeclarationsRequest {
            device: self
                .device
                .ok_or_else(|| BuildError::missing_field("device"))?,
            period_key: self
                .period_key
                .ok_or_else(|| BuildError::missing_field("period_key"))?,
        })
    }
}
