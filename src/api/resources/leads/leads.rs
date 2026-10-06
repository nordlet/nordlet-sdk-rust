use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct LeadsClient {
    pub http_client: HttpClient,
}

impl LeadsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn create(
        &self,
        request: &CreateLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn get(
        &self,
        request: &GetLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn update(
        &self,
        request: &UpdateLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn delete(
        &self,
        request: &DeleteLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeleteLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn list(
        &self,
        request: &ListLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn notes_create(
        &self,
        request: &NotesCreateLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<NotesCreateLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/notes/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn notes_delete(
        &self,
        request: &NotesDeleteLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<NotesDeleteLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/notes/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn notes_list(
        &self,
        request: &NotesListLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<NotesListLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/notes/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn files_list(
        &self,
        request: &FilesListLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<FilesListLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/files/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn sources_create(
        &self,
        request: &SourcesCreateLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SourcesCreateLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/sources/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn sources_update(
        &self,
        request: &SourcesUpdateLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SourcesUpdateLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/sources/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn sources_delete(
        &self,
        request: &SourcesDeleteLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SourcesDeleteLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/sources/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn sources_list(
        &self,
        request: &SourcesListLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SourcesListLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/sources/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn sources_options(
        &self,
        request: &SourcesOptionsLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SourcesOptionsLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/sources/options",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn types_create(
        &self,
        request: &TypesCreateLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TypesCreateLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/types/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn types_update(
        &self,
        request: &TypesUpdateLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TypesUpdateLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/types/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn types_delete(
        &self,
        request: &TypesDeleteLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TypesDeleteLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/types/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn types_list(
        &self,
        request: &TypesListLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TypesListLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/types/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn types_options(
        &self,
        request: &TypesOptionsLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TypesOptionsLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/types/options",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Create a customer partner from the lead, move the lead files to the partner, copy the lead notes into the partner notes and mark the lead as converted.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn convert(
        &self,
        request: &ConvertLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConvertLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/leads/convert",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
