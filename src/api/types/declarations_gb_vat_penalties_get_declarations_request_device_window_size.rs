pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GbVatPenaltiesGetDeclarationsRequestDeviceWindowSize {
    #[serde(default)]
    pub width: i64,
    #[serde(default)]
    pub height: i64,
}

impl GbVatPenaltiesGetDeclarationsRequestDeviceWindowSize {
    pub fn builder() -> GbVatPenaltiesGetDeclarationsRequestDeviceWindowSizeBuilder {
        <GbVatPenaltiesGetDeclarationsRequestDeviceWindowSizeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatPenaltiesGetDeclarationsRequestDeviceWindowSizeBuilder {
    width: Option<i64>,
    height: Option<i64>,
}

impl GbVatPenaltiesGetDeclarationsRequestDeviceWindowSizeBuilder {
    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GbVatPenaltiesGetDeclarationsRequestDeviceWindowSize`].
    /// This method will fail if any of the following fields are not set:
    /// - [`width`](GbVatPenaltiesGetDeclarationsRequestDeviceWindowSizeBuilder::width)
    /// - [`height`](GbVatPenaltiesGetDeclarationsRequestDeviceWindowSizeBuilder::height)
    pub fn build(self) -> Result<GbVatPenaltiesGetDeclarationsRequestDeviceWindowSize, BuildError> {
        Ok(GbVatPenaltiesGetDeclarationsRequestDeviceWindowSize {
            width: self
                .width
                .ok_or_else(|| BuildError::missing_field("width"))?,
            height: self
                .height
                .ok_or_else(|| BuildError::missing_field("height"))?,
        })
    }
}
