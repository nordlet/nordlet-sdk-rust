pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1TransportWaybillsCreateRequest {
    #[serde(rename = "consigneePartnerId")]
    #[serde(default)]
    pub consignee_partner_id: String,
    #[serde(rename = "transporterPartnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transporter_partner_id: Option<String>,
    #[serde(rename = "documentDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_date: Option<String>,
    #[serde(rename = "dispatchAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub dispatch_at: DateTime<FixedOffset>,
    #[serde(rename = "estimatedArrivalAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub estimated_arrival_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "vehiclePlate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_plate: Option<String>,
    #[serde(rename = "trailerPlate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailer_plate: Option<String>,
    #[serde(rename = "driverName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_name: Option<String>,
    #[serde(rename = "driverSurname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_surname: Option<String>,
    #[serde(rename = "loadWarehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_warehouse_id: Option<String>,
    #[serde(rename = "loadAddress")]
    #[serde(default)]
    pub load_address: String,
    #[serde(rename = "unloadAddress")]
    #[serde(default)]
    pub unload_address: String,
    #[serde(rename = "valueEur")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_eur: Option<String>,
    #[serde(rename = "saleInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_invoice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<PostV1TransportWaybillsCreateRequestLinesItem>>,
}

impl PostV1TransportWaybillsCreateRequest {
    pub fn builder() -> PostV1TransportWaybillsCreateRequestBuilder {
        <PostV1TransportWaybillsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1TransportWaybillsCreateRequestBuilder {
    consignee_partner_id: Option<String>,
    transporter_partner_id: Option<String>,
    document_date: Option<String>,
    dispatch_at: Option<DateTime<FixedOffset>>,
    estimated_arrival_at: Option<DateTime<FixedOffset>>,
    vehicle_plate: Option<String>,
    trailer_plate: Option<String>,
    driver_name: Option<String>,
    driver_surname: Option<String>,
    load_warehouse_id: Option<String>,
    load_address: Option<String>,
    unload_address: Option<String>,
    value_eur: Option<String>,
    sale_invoice_id: Option<String>,
    notes: Option<String>,
    series: Option<String>,
    lines: Option<Vec<PostV1TransportWaybillsCreateRequestLinesItem>>,
}

impl PostV1TransportWaybillsCreateRequestBuilder {
    pub fn consignee_partner_id(mut self, value: impl Into<String>) -> Self {
        self.consignee_partner_id = Some(value.into());
        self
    }

    pub fn transporter_partner_id(mut self, value: impl Into<String>) -> Self {
        self.transporter_partner_id = Some(value.into());
        self
    }

    pub fn document_date(mut self, value: impl Into<String>) -> Self {
        self.document_date = Some(value.into());
        self
    }

    pub fn dispatch_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.dispatch_at = Some(value);
        self
    }

    pub fn estimated_arrival_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.estimated_arrival_at = Some(value);
        self
    }

    pub fn vehicle_plate(mut self, value: impl Into<String>) -> Self {
        self.vehicle_plate = Some(value.into());
        self
    }

    pub fn trailer_plate(mut self, value: impl Into<String>) -> Self {
        self.trailer_plate = Some(value.into());
        self
    }

    pub fn driver_name(mut self, value: impl Into<String>) -> Self {
        self.driver_name = Some(value.into());
        self
    }

    pub fn driver_surname(mut self, value: impl Into<String>) -> Self {
        self.driver_surname = Some(value.into());
        self
    }

    pub fn load_warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.load_warehouse_id = Some(value.into());
        self
    }

    pub fn load_address(mut self, value: impl Into<String>) -> Self {
        self.load_address = Some(value.into());
        self
    }

    pub fn unload_address(mut self, value: impl Into<String>) -> Self {
        self.unload_address = Some(value.into());
        self
    }

    pub fn value_eur(mut self, value: impl Into<String>) -> Self {
        self.value_eur = Some(value.into());
        self
    }

    pub fn sale_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.sale_invoice_id = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn series(mut self, value: impl Into<String>) -> Self {
        self.series = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<PostV1TransportWaybillsCreateRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1TransportWaybillsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`consignee_partner_id`](PostV1TransportWaybillsCreateRequestBuilder::consignee_partner_id)
    /// - [`dispatch_at`](PostV1TransportWaybillsCreateRequestBuilder::dispatch_at)
    /// - [`load_address`](PostV1TransportWaybillsCreateRequestBuilder::load_address)
    /// - [`unload_address`](PostV1TransportWaybillsCreateRequestBuilder::unload_address)
    pub fn build(self) -> Result<PostV1TransportWaybillsCreateRequest, BuildError> {
        Ok(PostV1TransportWaybillsCreateRequest {
            consignee_partner_id: self
                .consignee_partner_id
                .ok_or_else(|| BuildError::missing_field("consignee_partner_id"))?,
            transporter_partner_id: self.transporter_partner_id,
            document_date: self.document_date,
            dispatch_at: self
                .dispatch_at
                .ok_or_else(|| BuildError::missing_field("dispatch_at"))?,
            estimated_arrival_at: self.estimated_arrival_at,
            vehicle_plate: self.vehicle_plate,
            trailer_plate: self.trailer_plate,
            driver_name: self.driver_name,
            driver_surname: self.driver_surname,
            load_warehouse_id: self.load_warehouse_id,
            load_address: self
                .load_address
                .ok_or_else(|| BuildError::missing_field("load_address"))?,
            unload_address: self
                .unload_address
                .ok_or_else(|| BuildError::missing_field("unload_address"))?,
            value_eur: self.value_eur,
            sale_invoice_id: self.sale_invoice_id,
            notes: self.notes,
            series: self.series,
            lines: self.lines,
        })
    }
}
