pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GbVatPenaltiesGetDeclarationsRequest {
    /// Facts of the browser the user files from, which HMRC requires with every VAT (MTD) API call: `navigator.userAgent`, a UUID the browser keeps for the device, every screen (width, height, scaling factor, colour depth in bits), the window size in pixels and the time zone as `UTC±hh:mm`.
    #[serde(default)]
    pub device: GbVatPenaltiesGetDeclarationsRequestDevice,
}

impl GbVatPenaltiesGetDeclarationsRequest {
    pub fn builder() -> GbVatPenaltiesGetDeclarationsRequestBuilder {
        <GbVatPenaltiesGetDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatPenaltiesGetDeclarationsRequestBuilder {
    device: Option<GbVatPenaltiesGetDeclarationsRequestDevice>,
}

impl GbVatPenaltiesGetDeclarationsRequestBuilder {
    pub fn device(mut self, value: GbVatPenaltiesGetDeclarationsRequestDevice) -> Self {
        self.device = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GbVatPenaltiesGetDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`device`](GbVatPenaltiesGetDeclarationsRequestBuilder::device)
    pub fn build(self) -> Result<GbVatPenaltiesGetDeclarationsRequest, BuildError> {
        Ok(GbVatPenaltiesGetDeclarationsRequest {
            device: self
                .device
                .ok_or_else(|| BuildError::missing_field("device"))?,
        })
    }
}
