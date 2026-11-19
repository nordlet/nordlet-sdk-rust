pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GbVatPaymentsListDeclarationsRequestDeviceWindowSize {
    #[serde(default)]
    pub width: i64,
    #[serde(default)]
    pub height: i64,
}

impl GbVatPaymentsListDeclarationsRequestDeviceWindowSize {
    pub fn builder() -> GbVatPaymentsListDeclarationsRequestDeviceWindowSizeBuilder {
        <GbVatPaymentsListDeclarationsRequestDeviceWindowSizeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatPaymentsListDeclarationsRequestDeviceWindowSizeBuilder {
    width: Option<i64>,
    height: Option<i64>,
}

impl GbVatPaymentsListDeclarationsRequestDeviceWindowSizeBuilder {
    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GbVatPaymentsListDeclarationsRequestDeviceWindowSize`].
    /// This method will fail if any of the following fields are not set:
    /// - [`width`](GbVatPaymentsListDeclarationsRequestDeviceWindowSizeBuilder::width)
    /// - [`height`](GbVatPaymentsListDeclarationsRequestDeviceWindowSizeBuilder::height)
    pub fn build(self) -> Result<GbVatPaymentsListDeclarationsRequestDeviceWindowSize, BuildError> {
        Ok(GbVatPaymentsListDeclarationsRequestDeviceWindowSize {
            width: self
                .width
                .ok_or_else(|| BuildError::missing_field("width"))?,
            height: self
                .height
                .ok_or_else(|| BuildError::missing_field("height"))?,
        })
    }
}
