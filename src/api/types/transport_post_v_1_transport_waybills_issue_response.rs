pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1TransportWaybillsIssueResponse {
    #[serde(default)]
    pub id: String,
    pub status: PostV1TransportWaybillsIssueResponseStatus,
    #[serde(default)]
    pub series: String,
    #[serde(rename = "fullNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_number: Option<String>,
    #[serde(rename = "documentDate")]
    #[serde(default)]
    pub document_date: String,
    #[serde(rename = "dispatchAt")]
    #[serde(default)]
    pub dispatch_at: String,
    #[serde(rename = "estimatedArrivalAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_arrival_at: Option<String>,
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
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub lines: Vec<PostV1TransportWaybillsIssueResponseLinesItem>,
}

impl PostV1TransportWaybillsIssueResponse {
    pub fn builder() -> PostV1TransportWaybillsIssueResponseBuilder {
        <PostV1TransportWaybillsIssueResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1TransportWaybillsIssueResponseBuilder {
    id: Option<String>,
    status: Option<PostV1TransportWaybillsIssueResponseStatus>,
    series: Option<String>,
    full_number: Option<String>,
    document_date: Option<String>,
    dispatch_at: Option<String>,
    estimated_arrival_at: Option<String>,
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
    created_at: Option<String>,
    updated_at: Option<String>,
    lines: Option<Vec<PostV1TransportWaybillsIssueResponseLinesItem>>,
}

impl PostV1TransportWaybillsIssueResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostV1TransportWaybillsIssueResponseStatus) -> Self {
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

    pub fn document_date(mut self, value: impl Into<String>) -> Self {
        self.document_date = Some(value.into());
        self
    }

    pub fn dispatch_at(mut self, value: impl Into<String>) -> Self {
        self.dispatch_at = Some(value.into());
        self
    }

    pub fn estimated_arrival_at(mut self, value: impl Into<String>) -> Self {
        self.estimated_arrival_at = Some(value.into());
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<PostV1TransportWaybillsIssueResponseLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1TransportWaybillsIssueResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1TransportWaybillsIssueResponseBuilder::id)
    /// - [`status`](PostV1TransportWaybillsIssueResponseBuilder::status)
    /// - [`series`](PostV1TransportWaybillsIssueResponseBuilder::series)
    /// - [`document_date`](PostV1TransportWaybillsIssueResponseBuilder::document_date)
    /// - [`dispatch_at`](PostV1TransportWaybillsIssueResponseBuilder::dispatch_at)
    /// - [`consignee_partner_id`](PostV1TransportWaybillsIssueResponseBuilder::consignee_partner_id)
    /// - [`load_address`](PostV1TransportWaybillsIssueResponseBuilder::load_address)
    /// - [`unload_address`](PostV1TransportWaybillsIssueResponseBuilder::unload_address)
    /// - [`created_at`](PostV1TransportWaybillsIssueResponseBuilder::created_at)
    /// - [`updated_at`](PostV1TransportWaybillsIssueResponseBuilder::updated_at)
    /// - [`lines`](PostV1TransportWaybillsIssueResponseBuilder::lines)
    pub fn build(self) -> Result<PostV1TransportWaybillsIssueResponse, BuildError> {
        Ok(PostV1TransportWaybillsIssueResponse {
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
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
