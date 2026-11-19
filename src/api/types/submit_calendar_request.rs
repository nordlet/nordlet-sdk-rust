pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubmitCalendarRequest {
    #[serde(default)]
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correction: Option<SubmitCalendarRequestCorrection>,
    #[serde(rename = "documentKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_keys: Option<Vec<String>>,
    /// Facts of the browser the user files from, which HMRC requires with every VAT (MTD) API call: `navigator.userAgent`, a UUID the browser keeps for the device, every screen (width, height, scaling factor, colour depth in bits), the window size in pixels and the time zone as `UTC±hh:mm`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<SubmitCalendarRequestDevice>,
}

impl SubmitCalendarRequest {
    pub fn builder() -> SubmitCalendarRequestBuilder {
        <SubmitCalendarRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitCalendarRequestBuilder {
    key: Option<String>,
    amend: Option<bool>,
    correction: Option<SubmitCalendarRequestCorrection>,
    document_keys: Option<Vec<String>>,
    device: Option<SubmitCalendarRequestDevice>,
}

impl SubmitCalendarRequestBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn amend(mut self, value: bool) -> Self {
        self.amend = Some(value);
        self
    }

    pub fn correction(mut self, value: SubmitCalendarRequestCorrection) -> Self {
        self.correction = Some(value);
        self
    }

    pub fn document_keys(mut self, value: Vec<String>) -> Self {
        self.document_keys = Some(value);
        self
    }

    pub fn device(mut self, value: SubmitCalendarRequestDevice) -> Self {
        self.device = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmitCalendarRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](SubmitCalendarRequestBuilder::key)
    pub fn build(self) -> Result<SubmitCalendarRequest, BuildError> {
        Ok(SubmitCalendarRequest {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            amend: self.amend,
            correction: self.correction,
            document_keys: self.document_keys,
            device: self.device,
        })
    }
}
