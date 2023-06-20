/// GeneratePackagesSummaryRequest is the request body for the
/// GeneratePackagesSummary API method. It just takes a single name argument,
/// referring to the resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneratePackagesSummaryRequest {
    /// Required. The name of the resource to get a packages summary for in the
    /// form of `projects/\[PROJECT_ID]/resources/[RESOURCE_URL\]`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A summary of the packages found within the given resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackagesSummaryResponse {
    /// The unique URL of the image or the container for which this summary
    /// applies.
    #[prost(string, tag = "1")]
    pub resource_url: ::prost::alloc::string::String,
    /// A listing by license name of each of the licenses and their counts.
    #[prost(message, repeated, tag = "2")]
    pub licenses_summary: ::prost::alloc::vec::Vec<packages_summary_response::LicensesSummary>,
}
/// Nested message and enum types in `PackagesSummaryResponse`.
pub mod packages_summary_response {
    /// Per license count
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LicensesSummary {
        /// The license of the package. Note that the format of this value is not
        /// guaranteed. It may be nil, an empty string, a boolean value (A | B), a
        /// differently formed boolean value (A OR B), etc...
        #[prost(string, tag = "1")]
        pub license: ::prost::alloc::string::String,
        /// The number of fixable vulnerabilities associated with this resource.
        #[prost(int64, tag = "2")]
        pub count: i64,
    }
}
#[doc = r" Generated client implementations."]
pub mod container_analysis_v1_beta1_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Retrieves analysis results of Cloud components such as Docker container"]
    #[doc = " images. The Container Analysis API is an implementation of the"]
    #[doc = " [Grafeas](https://grafeas.io) API."]
    #[doc = ""]
    #[doc = " Analysis results are stored as a series of occurrences. An `Occurrence`"]
    #[doc = " contains information about a specific analysis instance on a resource. An"]
    #[doc = " occurrence refers to a `Note`. A note contains details describing the"]
    #[doc = " analysis and is generally stored in a separate project, called a `Provider`."]
    #[doc = " Multiple occurrences can refer to the same note."]
    #[doc = ""]
    #[doc = " For example, an SSL vulnerability could affect multiple images. In this case,"]
    #[doc = " there would be one note for the vulnerability and an occurrence for each"]
    #[doc = " image with the vulnerability referring to that note."]
    #[derive(Debug, Clone)]
    pub struct ContainerAnalysisV1Beta1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ContainerAnalysisV1Beta1Client<T>
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
        ) -> ContainerAnalysisV1Beta1Client<InterceptedService<T, F>>
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
            ContainerAnalysisV1Beta1Client::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Sets the access control policy on the specified note or occurrence."]
        #[doc = " Requires `containeranalysis.notes.setIamPolicy` or"]
        #[doc = " `containeranalysis.occurrences.setIamPolicy` permission if the resource is"]
        #[doc = " a note or an occurrence, respectively."]
        #[doc = ""]
        #[doc = " The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for"]
        #[doc = " notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for"]
        #[doc = " occurrences."]
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the access control policy for a note or an occurrence resource."]
        #[doc = " Requires `containeranalysis.notes.setIamPolicy` or"]
        #[doc = " `containeranalysis.occurrences.setIamPolicy` permission if the resource is"]
        #[doc = " a note or occurrence, respectively."]
        #[doc = ""]
        #[doc = " The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for"]
        #[doc = " notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for"]
        #[doc = " occurrences."]
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the permissions that a caller has on the specified note or"]
        #[doc = " occurrence. Requires list permission on the project (for example,"]
        #[doc = " `containeranalysis.notes.list`)."]
        #[doc = ""]
        #[doc = " The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for"]
        #[doc = " notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for"]
        #[doc = " occurrences."]
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::TestIamPermissionsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/TestIamPermissions") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a summary of the packages within a given resource."]
        pub async fn generate_packages_summary(
            &mut self,
            request: impl tonic::IntoRequest<super::GeneratePackagesSummaryRequest>,
        ) -> Result<tonic::Response<super::PackagesSummaryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/GeneratePackagesSummary") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
