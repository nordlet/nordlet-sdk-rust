pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GbVatPaymentsListDeclarationsRequest {
    /// Facts of the browser the user files from, which HMRC requires with every VAT (MTD) API call: `navigator.userAgent`, a UUID the browser keeps for the device, every screen (width, height, scaling factor, colour depth in bits), the window size in pixels and the time zone as `UTC±hh:mm`.
    #[serde(default)]
    pub device: GbVatPaymentsListDeclarationsRequestDevice,
    #[serde(default)]
    pub from: NaiveDate,
    #[serde(default)]
    pub to: NaiveDate,
}

impl GbVatPaymentsListDeclarationsRequest {
    pub fn builder() -> GbVatPaymentsListDeclarationsRequestBuilder {
        <GbVatPaymentsListDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatPaymentsListDeclarationsRequestBuilder {
    device: Option<GbVatPaymentsListDeclarationsRequestDevice>,
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
}

impl GbVatPaymentsListDeclarationsRequestBuilder {
    pub fn device(mut self, value: GbVatPaymentsListDeclarationsRequestDevice) -> Self {
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

    /// Consumes the builder and constructs a [`GbVatPaymentsListDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`device`](GbVatPaymentsListDeclarationsRequestBuilder::device)
    /// - [`from`](GbVatPaymentsListDeclarationsRequestBuilder::from)
    /// - [`to`](GbVatPaymentsListDeclarationsRequestBuilder::to)
    pub fn build(self) -> Result<GbVatPaymentsListDeclarationsRequest, BuildError> {
        Ok(GbVatPaymentsListDeclarationsRequest {
            device: self
                .device
                .ok_or_else(|| BuildError::missing_field("device"))?,
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
        })
    }
}
