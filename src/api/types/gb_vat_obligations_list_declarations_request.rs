pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GbVatObligationsListDeclarationsRequest {
    /// Facts of the browser the user files from, which HMRC requires with every VAT (MTD) API call: `navigator.userAgent`, a UUID the browser keeps for the device, every screen (width, height, scaling factor, colour depth in bits), the window size in pixels and the time zone as `UTC±hh:mm`.
    #[serde(default)]
    pub device: GbVatObligationsListDeclarationsRequestDevice,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GbVatObligationsListDeclarationsRequestStatus>,
}

impl GbVatObligationsListDeclarationsRequest {
    pub fn builder() -> GbVatObligationsListDeclarationsRequestBuilder {
        <GbVatObligationsListDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatObligationsListDeclarationsRequestBuilder {
    device: Option<GbVatObligationsListDeclarationsRequestDevice>,
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
    status: Option<GbVatObligationsListDeclarationsRequestStatus>,
}

impl GbVatObligationsListDeclarationsRequestBuilder {
    pub fn device(mut self, value: GbVatObligationsListDeclarationsRequestDevice) -> Self {
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

    pub fn status(mut self, value: GbVatObligationsListDeclarationsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GbVatObligationsListDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`device`](GbVatObligationsListDeclarationsRequestBuilder::device)
    pub fn build(self) -> Result<GbVatObligationsListDeclarationsRequest, BuildError> {
        Ok(GbVatObligationsListDeclarationsRequest {
            device: self
                .device
                .ok_or_else(|| BuildError::missing_field("device"))?,
            from: self.from,
            to: self.to,
            status: self.status,
        })
    }
}
