pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlZusDraKeduDeclarationsResponseInsuredItemKodTytulu {
    #[serde(default)]
    pub p1: String,
    #[serde(default)]
    pub p2: String,
    #[serde(default)]
    pub p3: String,
}

impl PlZusDraKeduDeclarationsResponseInsuredItemKodTytulu {
    pub fn builder() -> PlZusDraKeduDeclarationsResponseInsuredItemKodTytuluBuilder {
        <PlZusDraKeduDeclarationsResponseInsuredItemKodTytuluBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlZusDraKeduDeclarationsResponseInsuredItemKodTytuluBuilder {
    p1: Option<String>,
    p2: Option<String>,
    p3: Option<String>,
}

impl PlZusDraKeduDeclarationsResponseInsuredItemKodTytuluBuilder {
    pub fn p1(mut self, value: impl Into<String>) -> Self {
        self.p1 = Some(value.into());
        self
    }

    pub fn p2(mut self, value: impl Into<String>) -> Self {
        self.p2 = Some(value.into());
        self
    }

    pub fn p3(mut self, value: impl Into<String>) -> Self {
        self.p3 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PlZusDraKeduDeclarationsResponseInsuredItemKodTytulu`].
    /// This method will fail if any of the following fields are not set:
    /// - [`p1`](PlZusDraKeduDeclarationsResponseInsuredItemKodTytuluBuilder::p1)
    /// - [`p2`](PlZusDraKeduDeclarationsResponseInsuredItemKodTytuluBuilder::p2)
    /// - [`p3`](PlZusDraKeduDeclarationsResponseInsuredItemKodTytuluBuilder::p3)
    pub fn build(self) -> Result<PlZusDraKeduDeclarationsResponseInsuredItemKodTytulu, BuildError> {
        Ok(PlZusDraKeduDeclarationsResponseInsuredItemKodTytulu {
            p1: self.p1.ok_or_else(|| BuildError::missing_field("p1"))?,
            p2: self.p2.ok_or_else(|| BuildError::missing_field("p2"))?,
            p3: self.p3.ok_or_else(|| BuildError::missing_field("p3"))?,
        })
    }
}
