use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct CalendarClient {
    pub http_client: HttpClient,
}

impl CalendarClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists, in due-date order, the filing deadlines Nordlet derives for the company from its country and profile together with the company's own events, between `from` and `to` (default 45 days back to 120 days ahead of the company's current date). Each deadline carries its submissions, whether it can be filed with `calendar/submit`, amended, or built with `calendar/download`, and whether its automation is on; `includeDone: false` leaves out the rows that are completed, submitted or accepted.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &ListCalendarRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCalendarResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/calendar/list",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Reads one calendar row by `key`: the id of a company event, or `ruleKey:period` for a filing deadline with its submissions.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        request: &GetCalendarRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetCalendarResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/calendar/get",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// With amend: true the return is filed again as a correction of the one already submitted or accepted for the period; only returns whose format has a correction mark accept it.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn submit(
        &self,
        request: &SubmitCalendarRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubmitCalendarResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/calendar/submit",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Builds the file of a deadline whose format Nordlet produces but whose administration takes it only through the company's own account or program. Nothing is sent and no filing is recorded.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn download(
        &self,
        request: &DownloadCalendarRequest,
        options: Option<RequestOptions>,
    ) -> Result<DownloadCalendarResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/calendar/download",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates a company event (kind `custom`) with a title, a due date, notes and a done flag.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &CreateCalendarRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateCalendarResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/calendar/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Changes a calendar row by `key`. A company event takes a new title, due date, notes or done flag; a filing deadline (`ruleKey:period`) takes only notes and done, keeps its own title and date, and gets a stored row the first time it is annotated.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        request: &UpdateCalendarRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateCalendarResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/calendar/update",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a company event by its id, or the stored notes and done mark of a filing deadline by `ruleKey:period`; the deadline itself still comes from the rules.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        request: &DeleteCalendarRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeleteCalendarResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/calendar/delete",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
