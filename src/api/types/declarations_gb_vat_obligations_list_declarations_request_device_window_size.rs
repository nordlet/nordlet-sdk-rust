pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GbVatObligationsListDeclarationsRequestDeviceWindowSize {
    #[serde(default)]
    pub width: i64,
    #[serde(default)]
    pub height: i64,
}

impl GbVatObligationsListDeclarationsRequestDeviceWindowSize {
    pub fn builder() -> GbVatObligationsListDeclarationsRequestDeviceWindowSizeBuilder {
        <GbVatObligationsListDeclarationsRequestDeviceWindowSizeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatObligationsListDeclarationsRequestDeviceWindowSizeBuilder {
    width: Option<i64>,
    height: Option<i64>,
}

impl GbVatObligationsListDeclarationsRequestDeviceWindowSizeBuilder {
    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GbVatObligationsListDeclarationsRequestDeviceWindowSize`].
    /// This method will fail if any of the following fields are not set:
    /// - [`width`](GbVatObligationsListDeclarationsRequestDeviceWindowSizeBuilder::width)
    /// - [`height`](GbVatObligationsListDeclarationsRequestDeviceWindowSizeBuilder::height)
    pub fn build(
        self,
    ) -> Result<GbVatObligationsListDeclarationsRequestDeviceWindowSize, BuildError> {
        Ok(GbVatObligationsListDeclarationsRequestDeviceWindowSize {
            width: self
                .width
                .ok_or_else(|| BuildError::missing_field("width"))?,
            height: self
                .height
                .ok_or_else(|| BuildError::missing_field("height"))?,
        })
    }
}
