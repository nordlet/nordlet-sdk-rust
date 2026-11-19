pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsW2ListDeclarationsResponseFormsItemStatesItem {
    #[serde(default)]
    pub box15: UsW2ListDeclarationsResponseFormsItemStatesItemBox15,
    #[serde(default)]
    pub box16: String,
    #[serde(default)]
    pub box17: String,
}

impl UsW2ListDeclarationsResponseFormsItemStatesItem {
    pub fn builder() -> UsW2ListDeclarationsResponseFormsItemStatesItemBuilder {
        <UsW2ListDeclarationsResponseFormsItemStatesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsW2ListDeclarationsResponseFormsItemStatesItemBuilder {
    box15: Option<UsW2ListDeclarationsResponseFormsItemStatesItemBox15>,
    box16: Option<String>,
    box17: Option<String>,
}

impl UsW2ListDeclarationsResponseFormsItemStatesItemBuilder {
    pub fn box15(mut self, value: UsW2ListDeclarationsResponseFormsItemStatesItemBox15) -> Self {
        self.box15 = Some(value);
        self
    }

    pub fn box16(mut self, value: impl Into<String>) -> Self {
        self.box16 = Some(value.into());
        self
    }

    pub fn box17(mut self, value: impl Into<String>) -> Self {
        self.box17 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsW2ListDeclarationsResponseFormsItemStatesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`box15`](UsW2ListDeclarationsResponseFormsItemStatesItemBuilder::box15)
    /// - [`box16`](UsW2ListDeclarationsResponseFormsItemStatesItemBuilder::box16)
    /// - [`box17`](UsW2ListDeclarationsResponseFormsItemStatesItemBuilder::box17)
    pub fn build(self) -> Result<UsW2ListDeclarationsResponseFormsItemStatesItem, BuildError> {
        Ok(UsW2ListDeclarationsResponseFormsItemStatesItem {
            box15: self
                .box15
                .ok_or_else(|| BuildError::missing_field("box15"))?,
            box16: self
                .box16
                .ok_or_else(|| BuildError::missing_field("box16"))?,
            box17: self
                .box17
                .ok_or_else(|| BuildError::missing_field("box17"))?,
        })
    }
}
