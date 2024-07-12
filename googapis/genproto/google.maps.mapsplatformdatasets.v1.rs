/// The details about the data source when it is a local file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalFileSource {
    /// The file name of the uploaded file.
    #[prost(string, tag = "1")]
    pub filename: ::prost::alloc::string::String,
    /// The format of the file that is being uploaded.
    #[prost(enumeration = "FileFormat", tag = "2")]
    pub file_format: i32,
}
/// The details about the data source when it is in Google Cloud Storage.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsSource {
    /// Source data URI. For example, `gs://my_bucket/my_object`.
    #[prost(string, tag = "1")]
    pub input_uri: ::prost::alloc::string::String,
    /// The file format of the Google Cloud Storage object. This is used mainly for
    /// validation.
    #[prost(enumeration = "FileFormat", tag = "2")]
    pub file_format: i32,
}
/// The format of the file being uploaded.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FileFormat {
    /// Unspecified file format.
    Unspecified = 0,
    /// GeoJson file.
    Geojson = 1,
    /// KML file.
    Kml = 2,
    /// CSV file.
    Csv = 3,
}
/// A representation of a dataset resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dataset {
    /// Resource name.
    /// Format: projects/{project}/datasets/{dataset_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Human readable name, shown in the console UI.
    ///
    /// Must be unique within a project.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// A description of this dataset.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// The version ID of the dataset.
    #[prost(string, tag = "4")]
    pub version_id: ::prost::alloc::string::String,
    /// Specified use case for this dataset.
    #[prost(enumeration = "Usage", repeated, tag = "5")]
    pub usage: ::prost::alloc::vec::Vec<i32>,
    /// Output only. The status of this dataset version.
    #[prost(message, optional, tag = "12")]
    pub status: ::core::option::Option<Status>,
    /// Output only. Time when the dataset was first created.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the dataset metadata was last updated.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when this version was created.
    #[prost(message, optional, tag = "10")]
    pub version_create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The description for this version of dataset. It is provided
    /// when importing data to the dataset.
    #[prost(string, tag = "11")]
    pub version_description: ::prost::alloc::string::String,
    /// Details about the source of the data for the dataset.
    #[prost(oneof = "dataset::DataSource", tags = "6, 7")]
    pub data_source: ::core::option::Option<dataset::DataSource>,
}
/// Nested message and enum types in `Dataset`.
pub mod dataset {
    /// Details about the source of the data for the dataset.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DataSource {
        /// A local file source for the dataset for a single upload.
        #[prost(message, tag = "6")]
        LocalFileSource(super::LocalFileSource),
        /// A Google Cloud Storage file source for the dataset for a single upload.
        #[prost(message, tag = "7")]
        GcsSource(super::GcsSource),
    }
}
/// Status of the dataset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    /// State enum for status.
    #[prost(enumeration = "status::State", tag = "1")]
    pub state: i32,
    /// Error message indicating reason of failure. It is empty if the datasets is
    /// not in a failed state.
    #[prost(string, tag = "2")]
    pub error_message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Status`.
pub mod status {
    /// A list of states for the dataset.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The state of this dataset is not set.
        Unspecified = 0,
        /// Data is being imported to a dataset.
        Importing = 1,
        /// Importing data to a dataset succeeded.
        ImportSucceeded = 2,
        /// Importing data to a dataset failed.
        ImportFailed = 3,
        /// The dataset is in the process of getting deleted.
        Deleting = 4,
        /// The deletion failed state. This state represents that dataset deletion
        /// has failed. Deletion may be retried.
        DeletionFailed = 5,
        /// Data is being processed.
        Processing = 6,
        /// The processing failed state. This state represents that processing has
        /// failed and may report errors.
        ProcessingFailed = 7,
        /// This state is currently not used.
        NeedsReview = 8,
        /// The publishing state. This state represents the publishing is in
        /// progress.
        Publishing = 9,
        /// The publishing failed states. This state represents that the publishing
        /// failed. Publishing may be retried.
        PublishingFailed = 10,
        /// The completed state. This state represents the dataset being available
        /// for its specific usage.
        Completed = 11,
    }
}
/// Usage specifies where the data is intended to be used to inform how to
/// process the data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Usage {
    /// The usage of this dataset is not set.
    Unspecified = 0,
    /// This dataset will be used for data driven styling.
    DataDrivenStyling = 1,
}
/// Request to create a maps dataset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatasetRequest {
    /// Required. Parent project that will own the dataset.
    /// Format: projects/{project}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The dataset version to create.
    #[prost(message, optional, tag = "2")]
    pub dataset: ::core::option::Option<Dataset>,
}
/// Request to update the metadata fields of the dataset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatasetMetadataRequest {
    /// Required. Resource name of the dataset to update.
    /// Format: projects/{project}/datasets/{dataset_id}
    #[prost(message, optional, tag = "1")]
    pub dataset: ::core::option::Option<Dataset>,
    /// The list of fields to be updated.
    ///
    /// The value "*" is used for full replacement (default).
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request to get the specified dataset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetRequest {
    /// Required. Resource name.
    /// Format: projects/{project}/datasets/{dataset_id}
    ///
    ///
    /// Can also fetch some special versions by appending "@" and a tag.
    /// Format: projects/{project}/datasets/{dataset_id}@{tag}
    ///
    /// Tag "active": The info of the latest completed version will be included,
    /// and NOT_FOUND if the dataset does not have one.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to list datasets for the project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatasetsRequest {
    /// Required. The name of the project to list all the datasets for.
    /// Format: projects/{project}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of datasets to return per page.
    ///
    /// If unspecified (or zero), all datasets will be returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token, received from a previous ListDatasets call.
    /// Provide this to retrieve the subsequent page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The tag that specifies the desired version for each dataset.
    ///
    /// Note that when pagination is also specified, some filtering can happen
    /// after pagination, which may cause the response to contain fewer datasets
    /// than the page size, even if it's not the last page.
    ///
    /// Tag "active": Each dataset in the response will include the info of its
    /// latest completed version, and the dataset will be skipped if it does not
    /// have one.
    #[prost(string, tag = "4")]
    pub tag: ::prost::alloc::string::String,
}
/// Response object of ListDatasets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatasetsResponse {
    /// All the datasets for the project.
    #[prost(message, repeated, tag = "1")]
    pub datasets: ::prost::alloc::vec::Vec<Dataset>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    ///
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request to delete a dataset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatasetRequest {
    /// Required. The name of the dataset to delete.
    /// Format: projects/{project}/datasets/{dataset_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod maps_platform_datasets_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service definition for the Maps Platform Datasets API."]
    #[derive(Debug, Clone)]
    pub struct MapsPlatformDatasetsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MapsPlatformDatasetsClient<T>
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
        ) -> MapsPlatformDatasetsClient<InterceptedService<T, F>>
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
            MapsPlatformDatasetsClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates a new dataset for the specified project."]
        pub async fn create_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDatasetRequest>,
        ) -> Result<tonic::Response<super::Dataset>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.mapsplatformdatasets.v1.MapsPlatformDatasets/CreateDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the metadata for the dataset."]
        pub async fn update_dataset_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDatasetMetadataRequest>,
        ) -> Result<tonic::Response<super::Dataset>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.mapsplatformdatasets.v1.MapsPlatformDatasets/UpdateDatasetMetadata",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the dataset."]
        pub async fn get_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatasetRequest>,
        ) -> Result<tonic::Response<super::Dataset>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.mapsplatformdatasets.v1.MapsPlatformDatasets/GetDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all the datasets for the specified project."]
        pub async fn list_datasets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDatasetsRequest>,
        ) -> Result<tonic::Response<super::ListDatasetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.mapsplatformdatasets.v1.MapsPlatformDatasets/ListDatasets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified dataset."]
        pub async fn delete_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDatasetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.mapsplatformdatasets.v1.MapsPlatformDatasets/DeleteDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
