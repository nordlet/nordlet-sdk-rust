pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GbVatReturnsGetDeclarationsRequestDeviceWindowSize {
    #[serde(default)]
    pub width: i64,
    #[serde(default)]
    pub height: i64,
}

impl GbVatReturnsGetDeclarationsRequestDeviceWindowSize {
    pub fn builder() -> GbVatReturnsGetDeclarationsRequestDeviceWindowSizeBuilder {
        <GbVatReturnsGetDeclarationsRequestDeviceWindowSizeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatReturnsGetDeclarationsRequestDeviceWindowSizeBuilder {
    width: Option<i64>,
    height: Option<i64>,
}

impl GbVatReturnsGetDeclarationsRequestDeviceWindowSizeBuilder {
    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GbVatReturnsGetDeclarationsRequestDeviceWindowSize`].
    /// This method will fail if any of the following fields are not set:
    /// - [`width`](GbVatReturnsGetDeclarationsRequestDeviceWindowSizeBuilder::width)
    /// - [`height`](GbVatReturnsGetDeclarationsRequestDeviceWindowSizeBuilder::height)
    pub fn build(self) -> Result<GbVatReturnsGetDeclarationsRequestDeviceWindowSize, BuildError> {
        Ok(GbVatReturnsGetDeclarationsRequestDeviceWindowSize {
            width: self
                .width
                .ok_or_else(|| BuildError::missing_field("width"))?,
            height: self
                .height
                .ok_or_else(|| BuildError::missing_field("height"))?,
        })
    }
}
