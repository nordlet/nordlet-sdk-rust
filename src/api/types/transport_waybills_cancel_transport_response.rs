pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WaybillsCancelTransportResponse {
    #[serde(default)]
    pub id: String,
    pub status: WaybillsCancelTransportResponseStatus,
    #[serde(default)]
    pub series: String,
    #[serde(rename = "fullNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_number: Option<String>,
    #[serde(rename = "documentDate")]
    #[serde(default)]
    pub document_date: NaiveDate,
    #[serde(rename = "dispatchAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub dispatch_at: DateTime<FixedOffset>,
    #[serde(rename = "estimatedArrivalAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub estimated_arrival_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "consigneePartnerId")]
    #[serde(default)]
    pub consignee_partner_id: String,
    #[serde(rename = "transporterPartnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transporter_partner_id: Option<String>,
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
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl WaybillsCancelTransportResponse {
    pub fn builder() -> WaybillsCancelTransportResponseBuilder {
        <WaybillsCancelTransportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WaybillsCancelTransportResponseBuilder {
    id: Option<String>,
    status: Option<WaybillsCancelTransportResponseStatus>,
    series: Option<String>,
    full_number: Option<String>,
    document_date: Option<NaiveDate>,
    dispatch_at: Option<DateTime<FixedOffset>>,
    estimated_arrival_at: Option<DateTime<FixedOffset>>,
    consignee_partner_id: Option<String>,
    transporter_partner_id: Option<String>,
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
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl WaybillsCancelTransportResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: WaybillsCancelTransportResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn series(mut self, value: impl Into<String>) -> Self {
        self.series = Some(value.into());
        self
    }

    pub fn full_number(mut self, value: impl Into<String>) -> Self {
        self.full_number = Some(value.into());
        self
    }

    pub fn document_date(mut self, value: NaiveDate) -> Self {
        self.document_date = Some(value);
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

    pub fn consignee_partner_id(mut self, value: impl Into<String>) -> Self {
        self.consignee_partner_id = Some(value.into());
        self
    }

    pub fn transporter_partner_id(mut self, value: impl Into<String>) -> Self {
        self.transporter_partner_id = Some(value.into());
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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WaybillsCancelTransportResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WaybillsCancelTransportResponseBuilder::id)
    /// - [`status`](WaybillsCancelTransportResponseBuilder::status)
    /// - [`series`](WaybillsCancelTransportResponseBuilder::series)
    /// - [`document_date`](WaybillsCancelTransportResponseBuilder::document_date)
    /// - [`dispatch_at`](WaybillsCancelTransportResponseBuilder::dispatch_at)
    /// - [`consignee_partner_id`](WaybillsCancelTransportResponseBuilder::consignee_partner_id)
    /// - [`load_address`](WaybillsCancelTransportResponseBuilder::load_address)
    /// - [`unload_address`](WaybillsCancelTransportResponseBuilder::unload_address)
    /// - [`created_at`](WaybillsCancelTransportResponseBuilder::created_at)
    /// - [`updated_at`](WaybillsCancelTransportResponseBuilder::updated_at)
    pub fn build(self) -> Result<WaybillsCancelTransportResponse, BuildError> {
        Ok(WaybillsCancelTransportResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            series: self
                .series
                .ok_or_else(|| BuildError::missing_field("series"))?,
            full_number: self.full_number,
            document_date: self
                .document_date
                .ok_or_else(|| BuildError::missing_field("document_date"))?,
            dispatch_at: self
                .dispatch_at
                .ok_or_else(|| BuildError::missing_field("dispatch_at"))?,
            estimated_arrival_at: self.estimated_arrival_at,
            consignee_partner_id: self
                .consignee_partner_id
                .ok_or_else(|| BuildError::missing_field("consignee_partner_id"))?,
            transporter_partner_id: self.transporter_partner_id,
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
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
