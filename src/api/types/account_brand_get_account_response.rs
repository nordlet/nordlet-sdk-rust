pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BrandGetAccountResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "logoUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
}

impl BrandGetAccountResponse {
    pub fn builder() -> BrandGetAccountResponseBuilder {
        <BrandGetAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BrandGetAccountResponseBuilder {
    name: Option<String>,
    logo_url: Option<String>,
    color: Option<String>,
    footer: Option<String>,
}

impl BrandGetAccountResponseBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
        self
    }

    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.color = Some(value.into());
        self
    }

    pub fn footer(mut self, value: impl Into<String>) -> Self {
        self.footer = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BrandGetAccountResponse`].
    pub fn build(self) -> Result<BrandGetAccountResponse, BuildError> {
        Ok(BrandGetAccountResponse {
            name: self.name,
            logo_url: self.logo_url,
            color: self.color,
            footer: self.footer,
        })
    }
}
