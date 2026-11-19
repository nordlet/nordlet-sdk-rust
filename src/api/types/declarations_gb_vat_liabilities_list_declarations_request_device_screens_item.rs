pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GbVatLiabilitiesListDeclarationsRequestDeviceScreensItem {
    #[serde(default)]
    pub width: i64,
    #[serde(default)]
    pub height: i64,
    #[serde(rename = "scalingFactor")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub scaling_factor: f64,
    #[serde(rename = "colourDepth")]
    #[serde(default)]
    pub colour_depth: i64,
}

impl GbVatLiabilitiesListDeclarationsRequestDeviceScreensItem {
    pub fn builder() -> GbVatLiabilitiesListDeclarationsRequestDeviceScreensItemBuilder {
        <GbVatLiabilitiesListDeclarationsRequestDeviceScreensItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatLiabilitiesListDeclarationsRequestDeviceScreensItemBuilder {
    width: Option<i64>,
    height: Option<i64>,
    scaling_factor: Option<f64>,
    colour_depth: Option<i64>,
}

impl GbVatLiabilitiesListDeclarationsRequestDeviceScreensItemBuilder {
    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
        self
    }

    pub fn scaling_factor(mut self, value: f64) -> Self {
        self.scaling_factor = Some(value);
        self
    }

    pub fn colour_depth(mut self, value: i64) -> Self {
        self.colour_depth = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GbVatLiabilitiesListDeclarationsRequestDeviceScreensItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`width`](GbVatLiabilitiesListDeclarationsRequestDeviceScreensItemBuilder::width)
    /// - [`height`](GbVatLiabilitiesListDeclarationsRequestDeviceScreensItemBuilder::height)
    /// - [`scaling_factor`](GbVatLiabilitiesListDeclarationsRequestDeviceScreensItemBuilder::scaling_factor)
    /// - [`colour_depth`](GbVatLiabilitiesListDeclarationsRequestDeviceScreensItemBuilder::colour_depth)
    pub fn build(
        self,
    ) -> Result<GbVatLiabilitiesListDeclarationsRequestDeviceScreensItem, BuildError> {
        Ok(GbVatLiabilitiesListDeclarationsRequestDeviceScreensItem {
            width: self
                .width
                .ok_or_else(|| BuildError::missing_field("width"))?,
            height: self
                .height
                .ok_or_else(|| BuildError::missing_field("height"))?,
            scaling_factor: self
                .scaling_factor
                .ok_or_else(|| BuildError::missing_field("scaling_factor"))?,
            colour_depth: self
                .colour_depth
                .ok_or_else(|| BuildError::missing_field("colour_depth"))?,
        })
    }
}
