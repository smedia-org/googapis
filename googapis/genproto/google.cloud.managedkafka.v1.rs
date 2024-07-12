/// An Apache Kafka cluster deployed in a location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cluster {
    /// Identifier. The name of the cluster. Structured like:
    /// projects/{project_number}/locations/{location}/clusters/{cluster_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time when the cluster was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the cluster was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Labels as key value pairs.
    #[prost(map = "string, string", tag = "4")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Required. Capacity configuration for the Kafka cluster.
    #[prost(message, optional, tag = "5")]
    pub capacity_config: ::core::option::Option<CapacityConfig>,
    /// Optional. Rebalance configuration for the Kafka cluster.
    #[prost(message, optional, tag = "8")]
    pub rebalance_config: ::core::option::Option<RebalanceConfig>,
    /// Output only. The current state of the cluster.
    #[prost(enumeration = "cluster::State", tag = "10")]
    pub state: i32,
    /// Platform specific configuration properties for a Kafka cluster.
    #[prost(oneof = "cluster::PlatformConfig", tags = "9")]
    pub platform_config: ::core::option::Option<cluster::PlatformConfig>,
}
/// Nested message and enum types in `Cluster`.
pub mod cluster {
    /// The state of the cluster.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// A state was not specified.
        Unspecified = 0,
        /// The cluster is being created.
        Creating = 1,
        /// The cluster is active.
        Active = 2,
        /// The cluster is being deleted.
        Deleting = 3,
    }
    /// Platform specific configuration properties for a Kafka cluster.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PlatformConfig {
        /// Required. Configuration properties for a Kafka cluster deployed to Google
        /// Cloud Platform.
        #[prost(message, tag = "9")]
        GcpConfig(super::GcpConfig),
    }
}
/// A capacity configuration of a Kafka cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapacityConfig {
    /// Required. The number of vCPUs to provision for the cluster. Minimum: 3.
    #[prost(int64, tag = "1")]
    pub vcpu_count: i64,
    /// Required. The memory to provision for the cluster in bytes.
    /// The CPU:memory ratio (vCPU:GiB) must be between 1:1 and 1:8.
    /// Minimum: 3221225472 (3 GiB).
    #[prost(int64, tag = "2")]
    pub memory_bytes: i64,
}
/// Defines rebalancing behavior of a Kafka cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RebalanceConfig {
    /// Optional. The rebalance behavior for the cluster.
    /// When not specified, defaults to `NO_REBALANCE`.
    #[prost(enumeration = "rebalance_config::Mode", tag = "1")]
    pub mode: i32,
}
/// Nested message and enum types in `RebalanceConfig`.
pub mod rebalance_config {
    /// The partition rebalance mode for the cluster.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Mode {
        /// A mode was not specified. Do not use.
        Unspecified = 0,
        /// Do not rebalance automatically.
        NoRebalance = 1,
        /// Automatically rebalance topic partitions among brokers when the
        /// cluster is scaled up.
        AutoRebalanceOnScaleUp = 2,
    }
}
/// The configuration of a Virtual Private Cloud (VPC) network that can access
/// the Kafka cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConfig {
    /// Required. Name of the VPC subnet in which to create Private Service Connect
    /// (PSC) endpoints for the Kafka brokers and bootstrap address. Structured
    /// like: projects/{project}/regions/{region}/subnetworks/{subnet_id}
    ///
    /// The subnet must be located in the same region as the Kafka cluster. The
    /// project may differ. Multiple subnets from the same parent network must not
    /// be specified.
    ///
    /// The CIDR range of the subnet must be within the IPv4 address ranges for
    /// private networks, as specified in RFC 1918.
    #[prost(string, tag = "2")]
    pub subnet: ::prost::alloc::string::String,
}
/// The configuration of access to the Kafka cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessConfig {
    /// Required. Virtual Private Cloud (VPC) networks that must be granted direct
    /// access to the Kafka cluster. Minimum of 1 network is required. Maximum 10
    /// networks can be specified.
    #[prost(message, repeated, tag = "1")]
    pub network_configs: ::prost::alloc::vec::Vec<NetworkConfig>,
}
/// Configuration properties for a Kafka cluster deployed to Google Cloud
/// Platform.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcpConfig {
    /// Required. Access configuration for the Kafka cluster.
    #[prost(message, optional, tag = "3")]
    pub access_config: ::core::option::Option<AccessConfig>,
    /// Optional. Immutable. The Cloud KMS Key name to use for encryption. The key
    /// must be located in the same region as the cluster and cannot be changed.
    /// Structured like:
    /// projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}.
    /// Note that the project component only accepts a project ID, and not a
    /// project number.
    #[prost(string, tag = "2")]
    pub kms_key: ::prost::alloc::string::String,
}
/// A Kafka topic in a given cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Topic {
    /// Identifier. The name of the topic. The `topic` segment is used when
    /// connecting directly to the cluster. Structured like:
    /// projects/{project}/locations/{location}/clusters/{cluster}/topics/{topic}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The number of partitions this topic has. The partition count can
    /// only be increased, not decreased. Please note that if partitions are
    /// increased for a topic that has a key, the partitioning logic or the
    /// ordering of the messages will be affected.
    #[prost(int32, tag = "2")]
    pub partition_count: i32,
    /// Required. Immutable. The number of replicas of each partition. A
    /// replication factor of 3 is recommended for high availability.
    #[prost(int32, tag = "3")]
    pub replication_factor: i32,
    /// Optional. Configurations for the topic that are overridden from the cluster
    /// defaults. The key of the map is a Kafka topic property name, for example:
    /// `cleanup.policy`, `compression.type`.
    #[prost(map = "string, string", tag = "4")]
    pub configs:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Metadata for a consumer group corresponding to a specific topic.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsumerTopicMetadata {
    /// Optional. Metadata for this consumer group and topic for all partition
    /// indexes it has metadata for.
    #[prost(map = "int32, message", tag = "1")]
    pub partitions: ::std::collections::HashMap<i32, ConsumerPartitionMetadata>,
}
/// Metadata for a consumer group corresponding to a specific partition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsumerPartitionMetadata {
    /// Required. The offset for this partition, or 0 if no offset has been
    /// committed.
    #[prost(int64, tag = "1")]
    pub offset: i64,
    /// Optional. The associated metadata for this partition, or empty if it does
    /// not exist.
    #[prost(string, tag = "2")]
    pub metadata: ::prost::alloc::string::String,
}
/// A Kafka consumer group in a given cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsumerGroup {
    /// Identifier. The name of the consumer group. The `consumer_group` segment is
    /// used when connecting directly to the cluster. Structured like:
    /// projects/{project}/locations/{location}/clusters/{cluster}/consumerGroups/{consumer_group}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Metadata for this consumer group for all topics it has metadata
    /// for. The key of the map is a topic name, structured like:
    /// projects/{project}/locations/{location}/clusters/{cluster}/topics/{topic}
    #[prost(map = "string, message", tag = "2")]
    pub topics: ::std::collections::HashMap<::prost::alloc::string::String, ConsumerTopicMetadata>,
}
/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have been cancelled successfully
    /// have \[Operation.error][\] value with a
    /// \[google.rpc.Status.code][google.rpc.Status.code\] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Request for ListClusters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersRequest {
    /// Required. The parent location whose clusters are to be listed. Structured
    /// like `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of clusters to return. The service may return
    /// fewer than this value. If unspecified, server will pick an appropriate
    /// default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListClusters` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListClusters` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter expression for the result.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Order by fields for the result.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for ListClusters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersResponse {
    /// The list of Clusters in the requested parent.
    #[prost(message, repeated, tag = "1")]
    pub clusters: ::prost::alloc::vec::Vec<Cluster>,
    /// A token that can be sent as `page_token` to retrieve the next page of
    /// results. If this field is omitted, there are no more results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for GetCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterRequest {
    /// Required. The name of the cluster whose configuration to return.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for CreateCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClusterRequest {
    /// Required. The parent region in which to create the cluster. Structured like
    /// `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID to use for the cluster, which will become the final
    /// component of the cluster's name. The ID must be 1-63 characters long, and
    /// match the regular expression `\[a-z]([-a-z0-9]*[a-z0-9\])?` to comply with
    /// RFC 1035.
    ///
    /// This value is structured like: `my-cluster-id`.
    #[prost(string, tag = "2")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. Configuration of the cluster to create. Its `name` field is
    /// ignored.
    #[prost(message, optional, tag = "3")]
    pub cluster: ::core::option::Option<Cluster>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID to avoid duplication of requests. If a request times out or
    /// fails, retrying with the same ID allows the server to recognize the
    /// previous attempt. For at least 60 minutes, the server ignores duplicate
    /// requests bearing the same ID.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID
    /// within 60 minutes of the last request, the server checks if an original
    /// operation with the same request ID was received. If so, the server ignores
    /// the second request.
    ///
    /// The request ID must be a valid UUID. A zero UUID is not supported
    /// (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for UpdateCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateClusterRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// cluster resource by the update. The fields specified in the update_mask are
    /// relative to the resource, not the full request. A field will be overwritten
    /// if it is in the mask. The mask is required and a value of * will update all
    /// fields.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The cluster to update. Its `name` field must be populated.
    #[prost(message, optional, tag = "2")]
    pub cluster: ::core::option::Option<Cluster>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID to avoid duplication of requests. If a request times out or
    /// fails, retrying with the same ID allows the server to recognize the
    /// previous attempt. For at least 60 minutes, the server ignores duplicate
    /// requests bearing the same ID.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID
    /// within 60 minutes of the last request, the server checks if an original
    /// operation with the same request ID was received. If so, the server ignores
    /// the second request.
    ///
    /// The request ID must be a valid UUID. A zero UUID is not supported
    /// (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for DeleteCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteClusterRequest {
    /// Required. The name of the cluster to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID to avoid duplication of requests. If a request times out or
    /// fails, retrying with the same ID allows the server to recognize the
    /// previous attempt. For at least 60 minutes, the server ignores duplicate
    /// requests bearing the same ID.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID
    /// within 60 minutes of the last request, the server checks if an original
    /// operation with the same request ID was received. If so, the server ignores
    /// the second request.
    ///
    /// The request ID must be a valid UUID. A zero UUID is not supported
    /// (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request for ListTopics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTopicsRequest {
    /// Required. The parent cluster whose topics are to be listed. Structured like
    /// `projects/{project}/locations/{location}/clusters/{cluster}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of topics to return. The service may return
    /// fewer than this value. If unset or zero, all topics for the parent is
    /// returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListTopics` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListTopics` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for ListTopics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTopicsResponse {
    /// The list of topics in the requested parent. The order of the topics is
    /// unspecified.
    #[prost(message, repeated, tag = "1")]
    pub topics: ::prost::alloc::vec::Vec<Topic>,
    /// A token that can be sent as `page_token` to retrieve the next page of
    /// results. If this field is omitted, there are no more results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for GetTopic.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTopicRequest {
    /// Required. The name of the topic whose configuration to return. Structured
    /// like:
    /// projects/{project}/locations/{location}/clusters/{cluster}/topics/{topic}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for CreateTopic.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTopicRequest {
    /// Required. The parent cluster in which to create the topic.
    /// Structured like
    /// `projects/{project}/locations/{location}/clusters/{cluster}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID to use for the topic, which will become the final
    /// component of the topic's name.
    ///
    /// This value is structured like: `my-topic-name`.
    #[prost(string, tag = "2")]
    pub topic_id: ::prost::alloc::string::String,
    /// Required. Configuration of the topic to create. Its `name` field is
    /// ignored.
    #[prost(message, optional, tag = "3")]
    pub topic: ::core::option::Option<Topic>,
}
/// Request for UpdateTopic.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTopicRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// Topic resource by the update. The fields specified in the update_mask are
    /// relative to the resource, not the full request. A field will be overwritten
    /// if it is in the mask. The mask is required and a value of * will update all
    /// fields.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The topic to update. Its `name` field must be populated.
    #[prost(message, optional, tag = "2")]
    pub topic: ::core::option::Option<Topic>,
}
/// Request for DeleteTopic.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTopicRequest {
    /// Required. The name of the topic to delete.
    /// `projects/{project}/locations/{location}/clusters/{cluster}/topics/{topic}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for ListConsumerGroups.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConsumerGroupsRequest {
    /// Required. The parent cluster whose consumer groups are to be listed.
    /// Structured like
    /// `projects/{project}/locations/{location}/clusters/{cluster}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of consumer groups to return. The service may
    /// return fewer than this value. If unset or zero, all consumer groups for the
    /// parent is returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListConsumerGroups` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListConsumerGroups` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for ListConsumerGroups.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConsumerGroupsResponse {
    /// The list of consumer group in the requested parent. The order of the
    /// consumer groups is unspecified.
    #[prost(message, repeated, tag = "1")]
    pub consumer_groups: ::prost::alloc::vec::Vec<ConsumerGroup>,
    /// A token that can be sent as `page_token` to retrieve the next page of
    /// results. If this field is omitted, there are no more results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for GetConsumerGroup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConsumerGroupRequest {
    /// Required. The name of the consumer group whose configuration to return.
    /// `projects/{project}/locations/{location}/clusters/{cluster}/consumerGroups/{consumerGroup}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for UpdateConsumerGroup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConsumerGroupRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// ConsumerGroup resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. The
    /// mask is required and a value of * will update all fields.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The consumer group to update. Its `name` field must be populated.
    #[prost(message, optional, tag = "2")]
    pub consumer_group: ::core::option::Option<ConsumerGroup>,
}
/// Request for DeleteConsumerGroup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConsumerGroupRequest {
    /// Required. The name of the consumer group to delete.
    /// `projects/{project}/locations/{location}/clusters/{cluster}/consumerGroups/{consumerGroup}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod managed_kafka_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " The service that a client application uses to manage Apache Kafka clusters,"]
    #[doc = " topics and consumer groups."]
    #[derive(Debug, Clone)]
    pub struct ManagedKafkaClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ManagedKafkaClient<T>
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
        ) -> ManagedKafkaClient<InterceptedService<T, F>>
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
            ManagedKafkaClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists the clusters in a given project and location."]
        pub async fn list_clusters(
            &mut self,
            request: impl tonic::IntoRequest<super::ListClustersRequest>,
        ) -> Result<tonic::Response<super::ListClustersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.managedkafka.v1.ManagedKafka/ListClusters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the properties of a single cluster."]
        pub async fn get_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClusterRequest>,
        ) -> Result<tonic::Response<super::Cluster>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.managedkafka.v1.ManagedKafka/GetCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new cluster in a given project and location."]
        pub async fn create_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateClusterRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.managedkafka.v1.ManagedKafka/CreateCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the properties of a single cluster."]
        pub async fn update_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateClusterRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.managedkafka.v1.ManagedKafka/UpdateCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single cluster."]
        pub async fn delete_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteClusterRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.managedkafka.v1.ManagedKafka/DeleteCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the topics in a given cluster."]
        pub async fn list_topics(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTopicsRequest>,
        ) -> Result<tonic::Response<super::ListTopicsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.managedkafka.v1.ManagedKafka/ListTopics",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the properties of a single topic."]
        pub async fn get_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTopicRequest>,
        ) -> Result<tonic::Response<super::Topic>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.managedkafka.v1.ManagedKafka/GetTopic",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new topic in a given project and location."]
        pub async fn create_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTopicRequest>,
        ) -> Result<tonic::Response<super::Topic>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.managedkafka.v1.ManagedKafka/CreateTopic",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the properties of a single topic."]
        pub async fn update_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTopicRequest>,
        ) -> Result<tonic::Response<super::Topic>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.managedkafka.v1.ManagedKafka/UpdateTopic",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single topic."]
        pub async fn delete_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTopicRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.managedkafka.v1.ManagedKafka/DeleteTopic",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the consumer groups in a given cluster."]
        pub async fn list_consumer_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConsumerGroupsRequest>,
        ) -> Result<tonic::Response<super::ListConsumerGroupsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.managedkafka.v1.ManagedKafka/ListConsumerGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the properties of a single consumer group."]
        pub async fn get_consumer_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConsumerGroupRequest>,
        ) -> Result<tonic::Response<super::ConsumerGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.managedkafka.v1.ManagedKafka/GetConsumerGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the properties of a single consumer group."]
        pub async fn update_consumer_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConsumerGroupRequest>,
        ) -> Result<tonic::Response<super::ConsumerGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.managedkafka.v1.ManagedKafka/UpdateConsumerGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single consumer group."]
        pub async fn delete_consumer_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConsumerGroupRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.managedkafka.v1.ManagedKafka/DeleteConsumerGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
