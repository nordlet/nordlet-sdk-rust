pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GbVatLiabilitiesListDeclarationsRequestDeviceWindowSize {
    #[serde(default)]
    pub width: i64,
    #[serde(default)]
    pub height: i64,
}

impl GbVatLiabilitiesListDeclarationsRequestDeviceWindowSize {
    pub fn builder() -> GbVatLiabilitiesListDeclarationsRequestDeviceWindowSizeBuilder {
        <GbVatLiabilitiesListDeclarationsRequestDeviceWindowSizeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatLiabilitiesListDeclarationsRequestDeviceWindowSizeBuilder {
    width: Option<i64>,
    height: Option<i64>,
}

impl GbVatLiabilitiesListDeclarationsRequestDeviceWindowSizeBuilder {
    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GbVatLiabilitiesListDeclarationsRequestDeviceWindowSize`].
    /// This method will fail if any of the following fields are not set:
    /// - [`width`](GbVatLiabilitiesListDeclarationsRequestDeviceWindowSizeBuilder::width)
    /// - [`height`](GbVatLiabilitiesListDeclarationsRequestDeviceWindowSizeBuilder::height)
    pub fn build(
        self,
    ) -> Result<GbVatLiabilitiesListDeclarationsRequestDeviceWindowSize, BuildError> {
        Ok(GbVatLiabilitiesListDeclarationsRequestDeviceWindowSize {
            width: self
                .width
                .ok_or_else(|| BuildError::missing_field("width"))?,
            height: self
                .height
                .ok_or_else(|| BuildError::missing_field("height"))?,
        })
    }
}
