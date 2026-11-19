pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GbVatLiabilitiesListDeclarationsRequest {
    /// Facts of the browser the user files from, which HMRC requires with every VAT (MTD) API call: `navigator.userAgent`, a UUID the browser keeps for the device, every screen (width, height, scaling factor, colour depth in bits), the window size in pixels and the time zone as `UTC±hh:mm`.
    #[serde(default)]
    pub device: GbVatLiabilitiesListDeclarationsRequestDevice,
    #[serde(default)]
    pub from: NaiveDate,
    #[serde(default)]
    pub to: NaiveDate,
}

impl GbVatLiabilitiesListDeclarationsRequest {
    pub fn builder() -> GbVatLiabilitiesListDeclarationsRequestBuilder {
        <GbVatLiabilitiesListDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatLiabilitiesListDeclarationsRequestBuilder {
    device: Option<GbVatLiabilitiesListDeclarationsRequestDevice>,
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
}

impl GbVatLiabilitiesListDeclarationsRequestBuilder {
    pub fn device(mut self, value: GbVatLiabilitiesListDeclarationsRequestDevice) -> Self {
        self.device = Some(value);
        self
    }

    pub fn from(mut self, value: NaiveDate) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: NaiveDate) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GbVatLiabilitiesListDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`device`](GbVatLiabilitiesListDeclarationsRequestBuilder::device)
    /// - [`from`](GbVatLiabilitiesListDeclarationsRequestBuilder::from)
    /// - [`to`](GbVatLiabilitiesListDeclarationsRequestBuilder::to)
    pub fn build(self) -> Result<GbVatLiabilitiesListDeclarationsRequest, BuildError> {
        Ok(GbVatLiabilitiesListDeclarationsRequest {
            device: self
                .device
                .ok_or_else(|| BuildError::missing_field("device"))?,
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
        })
    }
}
