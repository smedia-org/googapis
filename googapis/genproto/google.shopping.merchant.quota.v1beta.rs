/// The group information for methods in the Merchant API. The quota is shared
/// between all methods in the group. Even if none of the methods within the
/// group have usage the information for the group is returned.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaGroup {
    /// Identifier. The resource name of the quota group.
    /// Format: accounts/{account}/quotas/{group}
    /// Note: There is no guarantee on the format of {group}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The current quota usage, meaning the number of calls already
    /// made on a given day to the methods in the group. The daily quota limits
    /// reset at at 12:00 PM midday UTC.
    #[prost(int64, tag = "2")]
    pub quota_usage: i64,
    /// Output only. The maximum number of calls allowed per day for the group.
    #[prost(int64, tag = "3")]
    pub quota_limit: i64,
    /// Output only. The maximum number of calls allowed per minute for the group.
    #[prost(int64, tag = "5")]
    pub quota_minute_limit: i64,
    /// Output only. List of all methods group quota applies to.
    #[prost(message, repeated, tag = "4")]
    pub method_details: ::prost::alloc::vec::Vec<MethodDetails>,
}
/// The method details per method in the Merchant API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MethodDetails {
    /// Output only. The name of the method for example `products.list`.
    #[prost(string, tag = "1")]
    pub method: ::prost::alloc::string::String,
    /// Output only. The API version that the method belongs to.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    /// Output only. The sub-API that the method belongs to.
    #[prost(string, tag = "3")]
    pub subapi: ::prost::alloc::string::String,
    /// Output only. The path for the method such as
    /// `products/v1/productInputs.insert`
    #[prost(string, tag = "4")]
    pub path: ::prost::alloc::string::String,
}
/// Request message for the ListQuotaGroups method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQuotaGroupsRequest {
    /// Required. The merchant account who owns the collection of method quotas
    /// Format: accounts/{account}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of quotas to return in the response, used
    /// for paging. Defaults to 500; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Token (if provided) to retrieve the subsequent page. All other
    /// parameters must match the original call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for the ListMethodGroups method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQuotaGroupsResponse {
    /// The methods, current quota usage and limits per each group. The quota is
    /// shared between all methods in the group. The groups are sorted in
    /// descending order based on
    /// \[quotaUsage][google.shopping.merchant.quota.v1main.QuotaGroup.quota_usage\].
    #[prost(message, repeated, tag = "1")]
    pub quota_groups: ::prost::alloc::vec::Vec<QuotaGroup>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod quota_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to get method call quota information per Merchant API method."]
    #[derive(Debug, Clone)]
    pub struct QuotaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> QuotaServiceClient<T>
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
        ) -> QuotaServiceClient<InterceptedService<T, F>>
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
            QuotaServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists the daily call quota and usage per group for your Merchant"]
        #[doc = " Center account."]
        pub async fn list_quota_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListQuotaGroupsRequest>,
        ) -> Result<tonic::Response<super::ListQuotaGroupsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.shopping.merchant.quota.v1beta.QuotaService/ListQuotaGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
