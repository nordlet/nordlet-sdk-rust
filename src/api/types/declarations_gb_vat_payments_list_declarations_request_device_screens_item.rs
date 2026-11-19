pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GbVatPaymentsListDeclarationsRequestDeviceScreensItem {
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

impl GbVatPaymentsListDeclarationsRequestDeviceScreensItem {
    pub fn builder() -> GbVatPaymentsListDeclarationsRequestDeviceScreensItemBuilder {
        <GbVatPaymentsListDeclarationsRequestDeviceScreensItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatPaymentsListDeclarationsRequestDeviceScreensItemBuilder {
    width: Option<i64>,
    height: Option<i64>,
    scaling_factor: Option<f64>,
    colour_depth: Option<i64>,
}

impl GbVatPaymentsListDeclarationsRequestDeviceScreensItemBuilder {
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

    /// Consumes the builder and constructs a [`GbVatPaymentsListDeclarationsRequestDeviceScreensItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`width`](GbVatPaymentsListDeclarationsRequestDeviceScreensItemBuilder::width)
    /// - [`height`](GbVatPaymentsListDeclarationsRequestDeviceScreensItemBuilder::height)
    /// - [`scaling_factor`](GbVatPaymentsListDeclarationsRequestDeviceScreensItemBuilder::scaling_factor)
    /// - [`colour_depth`](GbVatPaymentsListDeclarationsRequestDeviceScreensItemBuilder::colour_depth)
    pub fn build(
        self,
    ) -> Result<GbVatPaymentsListDeclarationsRequestDeviceScreensItem, BuildError> {
        Ok(GbVatPaymentsListDeclarationsRequestDeviceScreensItem {
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
