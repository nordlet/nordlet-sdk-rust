pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtFr0600ComputeResponseBreakdownItem {
    pub direction: PostV1DeclarationsLtFr0600ComputeResponseBreakdownItemDirection,
    #[serde(rename = "taxCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    #[serde(default)]
    pub net: String,
    #[serde(default)]
    pub vat: String,
    #[serde(rename = "taxableFields")]
    #[serde(default)]
    pub taxable_fields: Vec<String>,
    #[serde(rename = "vatFields")]
    #[serde(default)]
    pub vat_fields: Vec<String>,
}

impl PostV1DeclarationsLtFr0600ComputeResponseBreakdownItem {
    pub fn builder() -> PostV1DeclarationsLtFr0600ComputeResponseBreakdownItemBuilder {
        <PostV1DeclarationsLtFr0600ComputeResponseBreakdownItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtFr0600ComputeResponseBreakdownItemBuilder {
    direction: Option<PostV1DeclarationsLtFr0600ComputeResponseBreakdownItemDirection>,
    tax_code: Option<String>,
    net: Option<String>,
    vat: Option<String>,
    taxable_fields: Option<Vec<String>>,
    vat_fields: Option<Vec<String>>,
}

impl PostV1DeclarationsLtFr0600ComputeResponseBreakdownItemBuilder {
    pub fn direction(
        mut self,
        value: PostV1DeclarationsLtFr0600ComputeResponseBreakdownItemDirection,
    ) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn tax_code(mut self, value: impl Into<String>) -> Self {
        self.tax_code = Some(value.into());
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    pub fn vat(mut self, value: impl Into<String>) -> Self {
        self.vat = Some(value.into());
        self
    }

    pub fn taxable_fields(mut self, value: Vec<String>) -> Self {
        self.taxable_fields = Some(value);
        self
    }

    pub fn vat_fields(mut self, value: Vec<String>) -> Self {
        self.vat_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtFr0600ComputeResponseBreakdownItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`direction`](PostV1DeclarationsLtFr0600ComputeResponseBreakdownItemBuilder::direction)
    /// - [`net`](PostV1DeclarationsLtFr0600ComputeResponseBreakdownItemBuilder::net)
    /// - [`vat`](PostV1DeclarationsLtFr0600ComputeResponseBreakdownItemBuilder::vat)
    /// - [`taxable_fields`](PostV1DeclarationsLtFr0600ComputeResponseBreakdownItemBuilder::taxable_fields)
    /// - [`vat_fields`](PostV1DeclarationsLtFr0600ComputeResponseBreakdownItemBuilder::vat_fields)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsLtFr0600ComputeResponseBreakdownItem, BuildError> {
        Ok(PostV1DeclarationsLtFr0600ComputeResponseBreakdownItem {
            direction: self
                .direction
                .ok_or_else(|| BuildError::missing_field("direction"))?,
            tax_code: self.tax_code,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
            vat: self.vat.ok_or_else(|| BuildError::missing_field("vat"))?,
            taxable_fields: self
                .taxable_fields
                .ok_or_else(|| BuildError::missing_field("taxable_fields"))?,
            vat_fields: self
                .vat_fields
                .ok_or_else(|| BuildError::missing_field("vat_fields"))?,
        })
    }
}
