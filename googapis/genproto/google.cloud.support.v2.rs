/// An object containing information about the effective user and
/// authenticated principal responsible for an action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Actor {
    /// The name to display for the actor. If not provided, it is inferred from
    /// credentials supplied during case creation. When an email is provided, a
    /// display name must also be provided. This will be obfuscated if the user
    /// is a Google Support agent.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// The email address of the actor. If not provided, it is inferred from
    /// credentials supplied during case creation. If the authenticated principal
    /// does not have an email address, one must be provided. When a name is
    /// provided, an email must also be provided. This will be obfuscated if the
    /// user is a Google Support agent.
    #[prost(string, tag = "2")]
    pub email: ::prost::alloc::string::String,
    /// Output only. Whether the actor is a Google support actor.
    #[prost(bool, tag = "4")]
    pub google_support: bool,
}
/// Represents a file attached to a support case.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attachment {
    /// Output only. The resource name of the attachment.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time at which the attachment was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The user who uploaded the attachment. Note, the name and email
    /// will be obfuscated if the attachment was uploaded by Google support.
    #[prost(message, optional, tag = "3")]
    pub creator: ::core::option::Option<Actor>,
    /// The filename of the attachment (e.g. `"graph.jpg"`).
    #[prost(string, tag = "4")]
    pub filename: ::prost::alloc::string::String,
    /// Output only. The MIME type of the attachment (e.g. text/plain).
    #[prost(string, tag = "5")]
    pub mime_type: ::prost::alloc::string::String,
    /// Output only. The size of the attachment in bytes.
    #[prost(int64, tag = "6")]
    pub size_bytes: i64,
}
/// The request message for the ListAttachments endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAttachmentsRequest {
    /// Required. The resource name of Case object for which attachments should be
    /// listed.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of attachments fetched with each request. If not
    /// provided, the default is 10. The maximum page size that will be returned is
    /// 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying the page of results to return. If unspecified, the
    /// first page is retrieved.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for the ListAttachments endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAttachmentsResponse {
    /// The list of attachments associated with the given case.
    #[prost(message, repeated, tag = "1")]
    pub attachments: ::prost::alloc::vec::Vec<Attachment>,
    /// A token to retrieve the next page of results. This should be set in the
    /// `page_token` field of subsequent `cases.attachments.list` requests. If
    /// unspecified, there are no more results to retrieve.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod case_attachment_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " A service to manage file attachment for Google Cloud support cases."]
    #[derive(Debug, Clone)]
    pub struct CaseAttachmentServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CaseAttachmentServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CaseAttachmentServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            CaseAttachmentServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Retrieve all attachments associated with a support case."]
        pub async fn list_attachments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAttachmentsRequest>,
        ) -> Result<tonic::Response<super::ListAttachmentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.support.v2.CaseAttachmentService/ListAttachments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// A support case.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Case {
    /// The resource name for the case.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The short summary of the issue reported in this case.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// A broad description of the issue.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// The issue classification applicable to this case.
    #[prost(message, optional, tag = "4")]
    pub classification: ::core::option::Option<CaseClassification>,
    /// The timezone of the user who created the support case.
    /// It should be in a format IANA recognizes: <https://www.iana.org/time-zones.>
    /// There is no additional validation done by the API.
    #[prost(string, tag = "8")]
    pub time_zone: ::prost::alloc::string::String,
    /// The email addresses to receive updates on this case.
    #[prost(string, repeated, tag = "9")]
    pub subscriber_email_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The current status of the support case.
    #[prost(enumeration = "case::State", tag = "12")]
    pub state: i32,
    /// Output only. The time this case was created.
    #[prost(message, optional, tag = "13")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this case was last updated.
    #[prost(message, optional, tag = "14")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The user who created the case.
    ///
    /// Note: The name and email will be obfuscated if the case was created by
    /// Google Support.
    #[prost(message, optional, tag = "15")]
    pub creator: ::core::option::Option<Actor>,
    /// A user-supplied email address to send case update notifications for. This
    /// should only be used in BYOID flows, where we cannot infer the user's email
    /// address directly from their EUCs.
    #[prost(string, tag = "35")]
    pub contact_email: ::prost::alloc::string::String,
    /// Whether the case is currently escalated.
    #[prost(bool, tag = "17")]
    pub escalated: bool,
    /// Whether this case was created for internal API testing and should not be
    /// acted on by the support team.
    #[prost(bool, tag = "19")]
    pub test_case: bool,
    /// The language the user has requested to receive support in. This should be a
    /// BCP 47 language code (e.g., `"en"`, `"zh-CN"`, `"zh-TW"`, `"ja"`, `"ko"`).
    /// If no language or an unsupported language is specified, this field defaults
    /// to English (en).
    ///
    /// Language selection during case creation may affect your available support
    /// options. For a list of supported languages and their support working hours,
    /// see: <https://cloud.google.com/support/docs/language-working-hours>
    #[prost(string, tag = "23")]
    pub language_code: ::prost::alloc::string::String,
    /// The priority of this case.
    #[prost(enumeration = "case::Priority", tag = "32")]
    pub priority: i32,
}
/// Nested message and enum types in `Case`.
pub mod case {
    /// The status of a support case.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Case is in an unknown state.
        Unspecified = 0,
        /// The case has been created but no one is assigned to work on it yet.
        New = 1,
        /// The case is currently being handled by Google support.
        InProgressGoogleSupport = 2,
        /// Google is waiting for a response.
        ActionRequired = 3,
        /// A solution has been offered for the case, but it isn't yet closed.
        SolutionProvided = 4,
        /// The case has been resolved.
        Closed = 5,
    }
    /// The case Priority. P0 is most urgent and P4 the least.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Priority {
        /// Priority is undefined or has not been set yet.
        Unspecified = 0,
        /// Extreme impact on a production service. Service is hard down.
        P0 = 1,
        /// Critical impact on a production service. Service is currently unusable.
        P1 = 2,
        /// Severe impact on a production service. Service is usable but greatly
        /// impaired.
        P2 = 3,
        /// Medium impact on a production service.  Service is available, but
        /// moderately impaired.
        P3 = 4,
        /// General questions or minor issues.  Production service is fully
        /// available.
        P4 = 5,
    }
}
/// A classification object with a product type and value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaseClassification {
    /// The unique ID for a classification. Must be specified for case creation.
    ///
    /// To retrieve valid classification IDs for case creation, use
    /// `caseClassifications.search`.
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    /// The display name of the classification.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
}
/// An escalation of a support case.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Escalation {
    /// Required. The reason why the Case is being escalated.
    #[prost(enumeration = "escalation::Reason", tag = "4")]
    pub reason: i32,
    /// Required. A free text description to accompany the `reason` field above.
    /// Provides additional context on why the case is being escalated.
    #[prost(string, tag = "5")]
    pub justification: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Escalation`.
pub mod escalation {
    /// An enum detailing the possible reasons a case may be escalated.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Reason {
        /// The escalation reason is in an unknown state or has not been specified.
        Unspecified = 0,
        /// The case is taking too long to resolve.
        ResolutionTime = 1,
        /// The support agent does not have the expertise required to successfully
        /// resolve the issue.
        TechnicalExpertise = 2,
        /// The issue is having a significant business impact.
        BusinessImpact = 3,
    }
}
/// The request message for the GetCase endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCaseRequest {
    /// Required. The fully qualified name of a case to be retrieved.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for the CreateCase endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCaseRequest {
    /// Required. The name of the Google Cloud Resource under which the case should
    /// be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The case to be created.
    #[prost(message, optional, tag = "2")]
    pub case: ::core::option::Option<Case>,
}
/// The request message for the ListCases endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCasesRequest {
    /// Required. The fully qualified name of parent resource to list cases under.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// An expression written in filter language. If non-empty, the query returns
    /// the cases that match the filter. Else, the query doesn't filter the cases.
    ///
    /// Filter expressions use the following fields with the operators equals (`=`)
    /// and `AND`:
    ///
    /// - `state`: The accepted values are `OPEN` or `CLOSED`.
    /// - `priority`: The accepted values are `P0`, `P1`, `P2`, `P3`, or `P4`. You
    /// can specify multiple values for priority using the `OR` operator. For
    /// example, `priority=P1 OR priority=P2`.
    /// - `creator.email`: The email address of the case creator.
    ///
    /// Examples:
    ///
    /// - `state=CLOSED`
    /// - `state=OPEN AND creator.email="tester@example.com"`
    /// - `state=OPEN AND (priority=P0 OR priority=P1)`
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The maximum number of cases fetched with each request. Defaults to 10.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// A token identifying the page of results to return. If unspecified, the
    /// first page is retrieved.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for the ListCases endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCasesResponse {
    /// The list of cases associated with the Google Cloud Resource, after any
    /// filters have been applied.
    #[prost(message, repeated, tag = "1")]
    pub cases: ::prost::alloc::vec::Vec<Case>,
    /// A token to retrieve the next page of results. This should be set in the
    /// `page_token` field of the subsequent `ListCasesRequest` message that is
    /// issued. If unspecified, there are no more results to retrieve.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for the SearchCases endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCasesRequest {
    /// The fully qualified name of parent resource to search cases under.
    #[prost(string, tag = "4")]
    pub parent: ::prost::alloc::string::String,
    /// An expression written in filter language.
    ///
    /// A query uses the following fields with the operators equals (`=`) and
    /// `AND`:
    ///
    /// - `organization`: An organization name in the form
    /// `organizations/<organization_id>`.
    /// - `project`: A project name in the form `projects/<project_id>`.
    /// - `state`: The accepted values are `OPEN` or `CLOSED`.
    /// - `priority`: The accepted values are `P0`, `P1`, `P2`, `P3`, or `P4`. You
    /// can specify multiple values for priority using the `OR` operator. For
    /// example, `priority=P1 OR priority=P2`.
    /// - `creator.email`: The email address of the case creator.
    /// - `billingAccount`: A billing account in the form
    /// `billingAccounts/<billing_account_id>`
    ///
    /// You must specify either `organization` or `project`.
    ///
    /// To search across `displayName`, `description`, and comments, use a global
    /// restriction with no keyword or operator. For example, `"my search"`.
    ///
    /// To search only cases updated after a certain date, use `update_time`
    /// restricted with that particular date, time, and timezone in ISO datetime
    /// format. For example, `update_time>"2020-01-01T00:00:00-05:00"`.
    /// `update_time` only supports the greater than operator (`>`).
    ///
    /// Examples:
    ///
    /// - `organization="organizations/123456789"`
    /// - `project="projects/my-project-id"`
    /// - `project="projects/123456789"`
    /// - `billing_account="billingAccounts/123456-A0B0C0-CUZ789"`
    /// - `organization="organizations/123456789" AND state=CLOSED`
    /// - `project="projects/my-project-id" AND creator.email="tester@example.com"`
    /// - `project="projects/my-project-id" AND (priority=P0 OR priority=P1)`
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
    /// The maximum number of cases fetched with each request. The default page
    /// size is 10.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying the page of results to return. If unspecified, the
    /// first page is retrieved.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for the SearchCases endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCasesResponse {
    /// The list of cases associated with the Google Cloud Resource, after any
    /// filters have been applied.
    #[prost(message, repeated, tag = "1")]
    pub cases: ::prost::alloc::vec::Vec<Case>,
    /// A token to retrieve the next page of results. This should be set in the
    /// `page_token` field of subsequent `SearchCaseRequest` message that is
    /// issued. If unspecified, there are no more results to retrieve.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for the EscalateCase endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EscalateCaseRequest {
    /// Required. The fully qualified name of the Case resource to be escalated.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The escalation object to be sent with the escalation request.
    #[prost(message, optional, tag = "2")]
    pub escalation: ::core::option::Option<Escalation>,
}
/// The request message for the UpdateCase endpoint
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCaseRequest {
    /// Required. The case object to update.
    #[prost(message, optional, tag = "1")]
    pub case: ::core::option::Option<Case>,
    /// A list of attributes of the case object that should be updated
    /// as part of this request. Supported values are `priority`, `display_name`,
    /// and `subscriber_email_addresses`. If no fields are specified, all supported
    /// fields are updated.
    ///
    /// WARNING: If you do not provide a field mask, then you might accidentally
    /// clear some fields. For example, if you leave the field mask empty and do
    /// not provide a value for `subscriber_email_addresses`, then
    /// `subscriber_email_addresses` is updated to empty.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for the CloseCase endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseCaseRequest {
    /// Required. The fully qualified name of the case resource to be closed.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for SearchCaseClassifications endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCaseClassificationsRequest {
    /// An expression written in the Google Cloud filter language. If non-empty,
    /// then only cases whose fields match the filter are returned. If empty, then
    /// no messages are filtered out.
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
    /// The maximum number of cases fetched with each request.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying the page of results to return. If unspecified, the
    /// first page is retrieved.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for SearchCaseClassifications endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCaseClassificationsResponse {
    /// The classifications retrieved.
    #[prost(message, repeated, tag = "1")]
    pub case_classifications: ::prost::alloc::vec::Vec<CaseClassification>,
    /// A token to retrieve the next page of results. This should be set in the
    /// `page_token` field of subsequent `SearchCaseClassificationsRequest` message
    /// that is issued. If unspecified, there are no more results to retrieve.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod case_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " A service to manage Google Cloud support cases."]
    #[derive(Debug, Clone)]
    pub struct CaseServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CaseServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CaseServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            CaseServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Retrieve the specified case."]
        pub async fn get_case(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCaseRequest>,
        ) -> Result<tonic::Response<super::Case>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.support.v2.CaseService/GetCase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieve all cases under the specified parent."]
        #[doc = ""]
        #[doc = " Note: Listing cases under an Organization returns only the cases directly"]
        #[doc = " parented by that organization. To retrieve all cases under an organization,"]
        #[doc = " including cases parented by projects under that organization, use"]
        #[doc = " `cases.search`."]
        pub async fn list_cases(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCasesRequest>,
        ) -> Result<tonic::Response<super::ListCasesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.support.v2.CaseService/ListCases",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Search cases using the specified query."]
        pub async fn search_cases(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchCasesRequest>,
        ) -> Result<tonic::Response<super::SearchCasesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.support.v2.CaseService/SearchCases",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create a new case and associate it with the given Google Cloud Resource."]
        #[doc = " The case object must have the following fields set: `display_name`,"]
        #[doc = " `description`, `classification`, and `priority`."]
        pub async fn create_case(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCaseRequest>,
        ) -> Result<tonic::Response<super::Case>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.support.v2.CaseService/CreateCase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update the specified case. Only a subset of fields can be updated."]
        pub async fn update_case(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCaseRequest>,
        ) -> Result<tonic::Response<super::Case>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.support.v2.CaseService/UpdateCase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Escalate a case. Escalating a case will initiate the Google Cloud Support"]
        #[doc = " escalation management process."]
        #[doc = ""]
        #[doc = " This operation is only available to certain Customer Care tiers. Go to"]
        #[doc = " https://cloud.google.com/support and look for 'Technical support"]
        #[doc = " escalations' in the feature list to find out which tiers are able to"]
        #[doc = " perform escalations."]
        pub async fn escalate_case(
            &mut self,
            request: impl tonic::IntoRequest<super::EscalateCaseRequest>,
        ) -> Result<tonic::Response<super::Case>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.support.v2.CaseService/EscalateCase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Close the specified case."]
        pub async fn close_case(
            &mut self,
            request: impl tonic::IntoRequest<super::CloseCaseRequest>,
        ) -> Result<tonic::Response<super::Case>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.support.v2.CaseService/CloseCase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieve valid classifications to be used when creating a support case."]
        #[doc = " The classications are hierarchical, with each classification containing"]
        #[doc = " all levels of the hierarchy, separated by \" > \". For example \"Technical"]
        #[doc = " Issue > Compute > Compute Engine\"."]
        pub async fn search_case_classifications(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchCaseClassificationsRequest>,
        ) -> Result<tonic::Response<super::SearchCaseClassificationsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.support.v2.CaseService/SearchCaseClassifications",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// A comment associated with a support case.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Comment {
    /// Output only. The resource name for the comment.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time when this comment was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The user or Google Support agent created this comment.
    #[prost(message, optional, tag = "3")]
    pub creator: ::core::option::Option<Actor>,
    /// The full comment body. Maximum of 12800 characters. This can contain rich
    /// text syntax.
    #[prost(string, tag = "4")]
    pub body: ::prost::alloc::string::String,
    /// Output only. DEPRECATED. An automatically generated plain text version of
    /// body with all rich text syntax stripped.
    #[prost(string, tag = "5")]
    pub plain_text_body: ::prost::alloc::string::String,
}
/// The request message for the ListComments endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCommentsRequest {
    /// Required. The resource name of Case object for which comments should be
    /// listed.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of comments fetched with each request. Defaults to 10.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// A token identifying the page of results to return. If unspecified, the
    /// first page is retrieved.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for the ListComments endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCommentsResponse {
    /// The list of Comments associated with the given Case.
    #[prost(message, repeated, tag = "1")]
    pub comments: ::prost::alloc::vec::Vec<Comment>,
    /// A token to retrieve the next page of results. This should be set in the
    /// `page_token` field of subsequent `ListCommentsRequest` message that is
    /// issued. If unspecified, there are no more results to retrieve.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for CreateComment endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCommentRequest {
    /// Required. The resource name of Case to which this comment should be added.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Comment object to be added to this Case.
    #[prost(message, optional, tag = "2")]
    pub comment: ::core::option::Option<Comment>,
}
#[doc = r" Generated client implementations."]
pub mod comment_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " A service to manage comments on cases."]
    #[derive(Debug, Clone)]
    pub struct CommentServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CommentServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CommentServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            CommentServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Retrieve all Comments associated with the Case object."]
        pub async fn list_comments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCommentsRequest>,
        ) -> Result<tonic::Response<super::ListCommentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.support.v2.CommentService/ListComments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Add a new comment to the specified Case."]
        #[doc = " The comment object must have the following fields set: body."]
        pub async fn create_comment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCommentRequest>,
        ) -> Result<tonic::Response<super::Comment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.support.v2.CommentService/CreateComment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
