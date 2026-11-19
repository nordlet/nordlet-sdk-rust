pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GbVatLiabilitiesListDeclarationsResponseLiabilitiesItem {
    #[serde(rename = "taxPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_period: Option<GbVatLiabilitiesListDeclarationsResponseLiabilitiesItemTaxPeriod>,
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "originalAmount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub original_amount: f64,
    #[serde(rename = "outstandingAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub outstanding_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<String>,
}

impl GbVatLiabilitiesListDeclarationsResponseLiabilitiesItem {
    pub fn builder() -> GbVatLiabilitiesListDeclarationsResponseLiabilitiesItemBuilder {
        <GbVatLiabilitiesListDeclarationsResponseLiabilitiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GbVatLiabilitiesListDeclarationsResponseLiabilitiesItemBuilder {
    tax_period: Option<GbVatLiabilitiesListDeclarationsResponseLiabilitiesItemTaxPeriod>,
    r#type: Option<String>,
    original_amount: Option<f64>,
    outstanding_amount: Option<f64>,
    due: Option<String>,
}

impl GbVatLiabilitiesListDeclarationsResponseLiabilitiesItemBuilder {
    pub fn tax_period(
        mut self,
        value: GbVatLiabilitiesListDeclarationsResponseLiabilitiesItemTaxPeriod,
    ) -> Self {
        self.tax_period = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn original_amount(mut self, value: f64) -> Self {
        self.original_amount = Some(value);
        self
    }

    pub fn outstanding_amount(mut self, value: f64) -> Self {
        self.outstanding_amount = Some(value);
        self
    }

    pub fn due(mut self, value: impl Into<String>) -> Self {
        self.due = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GbVatLiabilitiesListDeclarationsResponseLiabilitiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](GbVatLiabilitiesListDeclarationsResponseLiabilitiesItemBuilder::r#type)
    /// - [`original_amount`](GbVatLiabilitiesListDeclarationsResponseLiabilitiesItemBuilder::original_amount)
    pub fn build(
        self,
    ) -> Result<GbVatLiabilitiesListDeclarationsResponseLiabilitiesItem, BuildError> {
        Ok(GbVatLiabilitiesListDeclarationsResponseLiabilitiesItem {
            tax_period: self.tax_period,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            original_amount: self
                .original_amount
                .ok_or_else(|| BuildError::missing_field("original_amount"))?,
            outstanding_amount: self.outstanding_amount,
            due: self.due,
        })
    }
}
