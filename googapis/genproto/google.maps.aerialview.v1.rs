/// An object that encapsulates all of the data about a video.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Video {
    /// A mapping of media types to their URIs.
    /// This field is only included for `ACTIVE` videos.
    /// The key is an enum value from `MediaFormat`.
    #[prost(map = "string, message", tag = "1")]
    pub uris: ::std::collections::HashMap<::prost::alloc::string::String, Uris>,
    /// Current state of the render request.
    #[prost(enumeration = "video::State", tag = "2")]
    pub state: i32,
    /// Contains the video's metadata, only set if the state is `ACTIVE`.
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<VideoMetadata>,
}
/// Nested message and enum types in `Video`.
pub mod video {
    /// The different states a video can be in.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// The video is currently processing.
        Processing = 1,
        /// The video has finished rendering, and can be viewed through
        /// `LookupVideo`.
        Active = 2,
        /// The video has failed to render.
        Failed = 3,
    }
}
/// Contains all the uris for a given video format.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uris {
    /// A signed short-lived URI for the media in a landscape orientation.
    #[prost(string, tag = "1")]
    pub landscape_uri: ::prost::alloc::string::String,
    /// A signed short-lived URI for the media in a portrait orientation.
    #[prost(string, tag = "2")]
    pub portrait_uri: ::prost::alloc::string::String,
}
/// Contains metadata about a video, such as its videoId and duration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoMetadata {
    /// An ID for the video, and the recommended way to retrieve a video.
    #[prost(string, tag = "1")]
    pub video_id: ::prost::alloc::string::String,
    /// The date at which the imagery used in the video was captured.
    /// This will be at a month-level granularity.
    #[prost(message, optional, tag = "2")]
    pub capture_date: ::core::option::Option<super::super::super::r#type::Date>,
    /// The length of the video.
    #[prost(message, optional, tag = "3")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
/// Request message for `AerialView.RenderVideo`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenderVideoRequest {
    /// Required. A US postal address for the location to be rendered in the video.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Response message for `AerialView.RenderVideo`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenderVideoResponse {
    /// Current state of the render request.
    #[prost(enumeration = "video::State", tag = "1")]
    pub state: i32,
    /// Contains the video's metadata, only set if the state is `ACTIVE`.
    #[prost(message, optional, tag = "2")]
    pub metadata: ::core::option::Option<VideoMetadata>,
}
/// Request message for `AerialView.LookupVideo`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupVideoRequest {
    /// Required.
    /// A key used to look-up a video.
    #[prost(oneof = "lookup_video_request::Key", tags = "1, 2")]
    pub key: ::core::option::Option<lookup_video_request::Key>,
}
/// Nested message and enum types in `LookupVideoRequest`.
pub mod lookup_video_request {
    /// Required.
    /// A key used to look-up a video.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Key {
        /// An ID returned from `RenderVideo`.
        #[prost(string, tag = "1")]
        VideoId(::prost::alloc::string::String),
        /// A US postal address.
        #[prost(string, tag = "2")]
        Address(::prost::alloc::string::String),
    }
}
#[doc = r" Generated client implementations."]
pub mod aerial_view_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service definition for the Aerial View API."]
    #[derive(Debug, Clone)]
    pub struct AerialViewClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AerialViewClient<T>
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
        ) -> AerialViewClient<InterceptedService<T, F>>
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
            AerialViewClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Adds an address to the renderer's queue if a video hasn't already been"]
        #[doc = " rendered. Otherwise, returns metadata about the video."]
        pub async fn render_video(
            &mut self,
            request: impl tonic::IntoRequest<super::RenderVideoRequest>,
        ) -> Result<tonic::Response<super::RenderVideoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.aerialview.v1.AerialView/RenderVideo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Fetches a video given its address or videoId. The response will either be"]
        #[doc = " a video with a set of playback URIs for ACTIVE videos, a PROCESSING state"]
        #[doc = " for pending videos, or a 404 error if the video does not exist. Receiving a"]
        #[doc = " video is a billable event, so callers of this method should be ready to use"]
        #[doc = " the returned URIs at the time of request."]
        pub async fn lookup_video(
            &mut self,
            request: impl tonic::IntoRequest<super::LookupVideoRequest>,
        ) -> Result<tonic::Response<super::Video>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.aerialview.v1.AerialView/LookupVideo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
